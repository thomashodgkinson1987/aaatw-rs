use std::collections::HashMap;
use std::fs;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid PE: {0}")]
    InvalidPe(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("resource directory out of bounds")]
    OutOfBounds,
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceEntry {
    pub rva: u32,
    pub size: u32,
}

#[derive(Debug)]
struct Section {
    virtual_address: u32,
    virtual_size: u32,
    raw_offset: u32,
    raw_size: u32,
}

#[derive(Debug)]
pub struct ResourceDll {
    data: Vec<u8>,
    sections: Vec<Section>,
    resources: HashMap<String, ResourceEntry>,
    names: Vec<String>,
}

impl ResourceDll {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ResourceError> {
        let data = fs::read(path)?;
        Self::from_bytes(data)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, ResourceError> {
        let sections = parse_sections(&data)?;
        let resource_rva = parse_resource_rva(&data)?;
        let mut resources = HashMap::new();
        let mut names = Vec::new();

        if resource_rva != 0 {
            let base = rva_to_offset(&sections, resource_rva, data.len())
                .ok_or(ResourceError::OutOfBounds)?;
            collect_resources(&data, base, &mut resources, &mut names)?;
        }

        Ok(Self {
            data,
            sections,
            resources,
            names,
        })
    }

    pub fn get(&self, name: &str) -> Option<&[u8]> {
        let key = normalize_name(name);
        let entry = self.resources.get(&key)?;
        let offset = rva_to_offset(&self.sections, entry.rva, self.data.len())?;
        let end = offset.checked_add(entry.size as usize)?;
        self.data.get(offset..end)
    }

    pub fn list(&self) -> &[String] {
        &self.names
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

fn normalize_name(name: &str) -> String {
    let name = name.strip_prefix("std:").unwrap_or(name);
    name.trim().to_ascii_uppercase()
}

fn parse_sections(data: &[u8]) -> Result<Vec<Section>, ResourceError> {
    if data.len() < 0x40 || &data[0..2] != b"MZ" {
        return Err(ResourceError::InvalidPe("missing MZ signature".to_string()));
    }

    let e_lfanew = read_u32(data, 0x3c)? as usize;
    if data.len() < e_lfanew + 24 || &data[e_lfanew..e_lfanew + 4] != b"PE\0\0" {
        return Err(ResourceError::InvalidPe("missing PE signature".to_string()));
    }

    let coff = e_lfanew + 4;
    let section_count = read_u16(data, coff + 2)? as usize;
    let optional_size = read_u16(data, coff + 16)? as usize;
    let optional = coff + 20;

    if data.len() < optional + optional_size {
        return Err(ResourceError::InvalidPe(
            "optional header out of bounds".to_string(),
        ));
    }

    let magic = read_u16(data, optional)?;
    if magic != 0x10b {
        return Err(ResourceError::InvalidPe(format!(
            "unsupported optional header magic {magic:#x}"
        )));
    }

    let section_table = optional + optional_size;
    let mut sections = Vec::with_capacity(section_count);

    for index in 0..section_count {
        let offset = section_table + index * 40;
        if data.len() < offset + 40 {
            return Err(ResourceError::InvalidPe(
                "section table out of bounds".to_string(),
            ));
        }

        sections.push(Section {
            virtual_size: read_u32(data, offset + 8)?,
            virtual_address: read_u32(data, offset + 12)?,
            raw_size: read_u32(data, offset + 16)?,
            raw_offset: read_u32(data, offset + 20)?,
        });
    }

    Ok(sections)
}

fn parse_resource_rva(data: &[u8]) -> Result<u32, ResourceError> {
    let e_lfanew = read_u32(data, 0x3c)? as usize;
    let optional = e_lfanew + 24;
    let magic = read_u16(data, optional)?;
    let data_directory = match magic {
        0x10b => optional + 96,
        0x20b => optional + 112,
        _ => {
            return Err(ResourceError::InvalidPe(format!(
                "unsupported optional header magic {magic:#x}"
            )));
        }
    };

    read_u32(data, data_directory + 2 * 8)
}

fn rva_to_offset(sections: &[Section], rva: u32, len: usize) -> Option<usize> {
    for section in sections {
        let span = section.virtual_size.max(section.raw_size);
        if rva >= section.virtual_address && rva < section.virtual_address.saturating_add(span) {
            let offset = section.raw_offset as usize + (rva - section.virtual_address) as usize;
            return if offset < len { Some(offset) } else { None };
        }
    }
    None
}

fn collect_resources(
    data: &[u8],
    base: usize,
    resources: &mut HashMap<String, ResourceEntry>,
    names: &mut Vec<String>,
) -> Result<(), ResourceError> {
    for (type_name, type_offset) in read_directory(data, base, base)? {
        if type_name.starts_with('#') || type_offset & 0x8000_0000 == 0 {
            continue;
        }

        let type_dir = base + (type_offset & 0x7fff_ffff) as usize;
        for (name, name_offset) in read_directory(data, base, type_dir)? {
            if name_offset & 0x8000_0000 == 0 {
                insert_data(data, base, name, name_offset, resources, names)?;
                continue;
            }

            let language_dir = base + (name_offset & 0x7fff_ffff) as usize;
            for (_language, language_offset) in read_directory(data, base, language_dir)? {
                if language_offset & 0x8000_0000 == 0 {
                    insert_data(data, base, name.clone(), language_offset, resources, names)?;
                }
            }
        }
    }

    Ok(())
}

fn insert_data(
    data: &[u8],
    base: usize,
    name: String,
    offset: u32,
    resources: &mut HashMap<String, ResourceEntry>,
    names: &mut Vec<String>,
) -> Result<(), ResourceError> {
    let entry_offset = base + offset as usize;
    if data.len() < entry_offset + 16 {
        return Err(ResourceError::OutOfBounds);
    }

    let entry = ResourceEntry {
        rva: read_u32(data, entry_offset)?,
        size: read_u32(data, entry_offset + 4)?,
    };

    let key = normalize_name(&name);
    if resources.contains_key(&key) {
        return Ok(());
    }

    resources.insert(key.clone(), entry);
    names.push(key);

    if let Some(stem) = Path::new(&name)
        .file_stem()
        .and_then(|value| value.to_str())
    {
        let stem_key = normalize_name(stem);
        resources.entry(stem_key).or_insert(entry);
    }

    Ok(())
}

fn read_directory(
    data: &[u8],
    base: usize,
    offset: usize,
) -> Result<Vec<(String, u32)>, ResourceError> {
    if data.len() < offset + 16 {
        return Err(ResourceError::OutOfBounds);
    }

    let named = read_u16(data, offset + 12)? as usize;
    let id = read_u16(data, offset + 14)? as usize;
    let count = named + id;
    let mut entries = Vec::with_capacity(count);

    for index in 0..count {
        let entry_offset = offset + 16 + index * 8;
        if data.len() < entry_offset + 8 {
            return Err(ResourceError::OutOfBounds);
        }

        let name = read_u32(data, entry_offset)?;
        let target = read_u32(data, entry_offset + 4)?;
        entries.push((read_name(data, base, name)?, target));
    }

    Ok(entries)
}

fn read_name(data: &[u8], base: usize, name: u32) -> Result<String, ResourceError> {
    if name & 0x8000_0000 == 0 {
        return Ok(format!("#{name}"));
    }

    let offset = base + (name & 0x7fff_ffff) as usize;
    if data.len() < offset + 2 {
        return Err(ResourceError::OutOfBounds);
    }

    let length = read_u16(data, offset)? as usize;
    let start = offset + 2;
    let end = start + length * 2;
    if data.len() < end {
        return Err(ResourceError::OutOfBounds);
    }

    let units: Vec<u16> = data[start..end]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|chunk| u16::from_le_bytes(*chunk))
        .collect();

    Ok(String::from_utf16_lossy(&units))
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, ResourceError> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or(ResourceError::OutOfBounds)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, ResourceError> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or(ResourceError::OutOfBounds)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strips_std_and_uppercases() {
        assert_eq!(normalize_name("std:publisher.jpg"), "PUBLISHER.JPG");
        assert_eq!(normalize_name("  splashscreens.xui  "), "SPLASHSCREENS.XUI");
    }

    #[test]
    fn read_u16_and_u32_are_little_endian() {
        let bytes = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_u16(&bytes, 0).unwrap(), 0x5678);
        assert_eq!(read_u32(&bytes, 0).unwrap(), 0x12345678);
    }

    #[test]
    fn out_of_bounds_reads_error() {
        assert!(read_u16(&[0], 1).is_err());
        assert!(read_u32(&[0, 0, 0], 0).is_err());
    }
}
