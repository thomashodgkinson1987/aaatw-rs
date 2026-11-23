use thiserror::Error;

#[derive(Debug, Error)]
pub enum XuiError {
    #[error("xml parse error: {0}")]
    Xml(#[from] roxmltree::Error),
    #[error("missing attribute: {0}")]
    MissingAttribute(String),
}

#[derive(Debug, Clone)]
pub struct XuiTexture {
    pub id: String,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct XuiImage {
    pub id: Option<String>,
    pub tex: String,
    pub x: i32,
    pub y: i32,
    pub w: Option<u32>,
    pub h: Option<u32>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct XuiDocument {
    pub textures: Vec<XuiTexture>,
    pub images: Vec<XuiImage>,
    pub containers: Vec<String>,
}

impl XuiDocument {
    pub fn parse(text: &str) -> Result<Self, XuiError> {
        let document = roxmltree::Document::parse(text)?;
        let mut parsed = Self::default();

        for node in document.descendants() {
            match node.tag_name().name() {
                "texture" => {
                    let id = node
                        .attribute("id")
                        .ok_or_else(|| XuiError::MissingAttribute("id".to_string()))?;
                    let uri = node
                        .attribute("uri")
                        .ok_or_else(|| XuiError::MissingAttribute("uri".to_string()))?;
                    parsed.textures.push(XuiTexture {
                        id: id.to_string(),
                        uri: uri.to_string(),
                    });
                }
                "image" => {
                    let tex = node
                        .attribute("tex")
                        .ok_or_else(|| XuiError::MissingAttribute("tex".to_string()))?;
                    parsed.images.push(XuiImage {
                        id: node.attribute("id").map(str::to_string),
                        tex: tex.to_string(),
                        x: parse_attr(node, "x").unwrap_or(0),
                        y: parse_attr(node, "y").unwrap_or(0),
                        w: parse_attr(node, "w"),
                        h: parse_attr(node, "h"),
                        flags: node
                            .attribute("flags")
                            .map(|value| {
                                value
                                    .split_whitespace()
                                    .map(str::to_string)
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    });
                }
                "container" => {
                    if let Some(id) = node.attribute("id") {
                        parsed.containers.push(id.to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(parsed)
    }
}

fn parse_attr<T>(node: roxmltree::Node<'_, '_>, name: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    node.attribute(name).and_then(|value| value.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_splash_like_xui() {
        let xml = r#"
            <xui>
                <texture id="title_tex" uri="title_background.jpg"/>
                <image id="titleback" tex="title_tex" x="1" y="2" flags="traverse"/>
                <container id="title" flags="traverse"/>
            </xui>
        "#;

        let document = XuiDocument::parse(xml).unwrap();
        assert_eq!(document.textures.len(), 1);
        assert_eq!(document.textures[0].id, "title_tex");
        assert_eq!(document.images.len(), 1);
        assert_eq!(document.images[0].tex, "title_tex");
        assert_eq!(document.images[0].x, 1);
        assert_eq!(document.images[0].y, 2);
        assert_eq!(document.images[0].flags, vec!["traverse"]);
        assert_eq!(document.containers, vec!["title"]);
    }

    #[test]
    fn missing_required_texture_attribute_is_an_error() {
        let xml = r#"<xui><texture id="broken"/></xui>"#;
        assert!(XuiDocument::parse(xml).is_err());
    }
}
