use std::sync::atomic::{AtomicU64, Ordering};

use crate::{Color, Error, Result};

static NEXT_SURFACE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone)]
pub struct Surface {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        let len = width as usize * height as usize * 4;
        Self {
            id: NEXT_SURFACE_ID.fetch_add(1, Ordering::Relaxed),
            width,
            height,
            pixels: vec![0; len],
        }
    }

    pub fn from_rgba(width: u32, height: u32, pixels: Vec<u8>) -> Result<Self> {
        let expected = width as usize * height as usize * 4;
        if pixels.len() != expected {
            return Err(Error::InvalidSurface {
                expected,
                got: pixels.len(),
            });
        }
        Ok(Self {
            id: NEXT_SURFACE_ID.fetch_add(1, Ordering::Relaxed),
            width,
            height,
            pixels,
        })
    }

    pub fn from_encoded(data: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(data)?;
        let rgba = image.to_rgba8();
        Self::from_rgba(rgba.width(), rgba.height(), rgba.into_raw())
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.pixels.as_chunks_mut::<4>().0 {
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x0 = rect.x.max(0) as u32;
        let y0 = rect.y.max(0) as u32;
        let x1 = (rect.x + rect.w as i32).clamp(0, self.width as i32) as u32;
        let y1 = (rect.y + rect.h as i32).clamp(0, self.height as i32) as u32;

        if x0 >= x1 || y0 >= y1 {
            return;
        }

        let row_stride = self.width as usize * 4;
        for y in y0..y1 {
            let row = y as usize * row_stride;
            for x in x0..x1 {
                let offset = row + x as usize * 4;
                self.pixels[offset..offset + 4]
                    .copy_from_slice(&[color.r, color.g, color.b, color.a]);
            }
        }
    }

    pub fn blit(&mut self, src: &Surface, x: i32, y: i32) {
        let dest_x0 = x.max(0) as u32;
        let dest_y0 = y.max(0) as u32;
        let dest_x1 = (x + src.width as i32).clamp(0, self.width as i32) as u32;
        let dest_y1 = (y + src.height as i32).clamp(0, self.height as i32) as u32;

        if dest_x0 >= dest_x1 || dest_y0 >= dest_y1 {
            return;
        }

        let src_x0 = (dest_x0 as i32 - x) as u32;
        let src_y0 = (dest_y0 as i32 - y) as u32;
        let src_x1 = src_x0 + (dest_x1 - dest_x0);
        let src_y1 = src_y0 + (dest_y1 - dest_y0);
        let dst_stride = self.width as usize * 4;
        let src_stride = src.width as usize * 4;

        for sy in src_y0..src_y1 {
            let dy = (y + sy as i32) as u32;
            let dst_row = dy as usize * dst_stride;
            for sx in src_x0..src_x1 {
                let dx = (x + sx as i32) as u32;
                let si = sy as usize * src_stride + sx as usize * 4;
                let di = dst_row + dx as usize * 4;
                let alpha = src.pixels[si + 3] as u16;

                if alpha == 0 {
                    continue;
                }
                if alpha == 255 {
                    self.pixels[di..di + 4].copy_from_slice(&src.pixels[si..si + 4]);
                } else {
                    let inverse = 255 - alpha;
                    self.pixels[di] = ((src.pixels[si] as u16 * alpha
                        + self.pixels[di] as u16 * inverse)
                        / 255) as u8;
                    self.pixels[di + 1] = ((src.pixels[si + 1] as u16 * alpha
                        + self.pixels[di + 1] as u16 * inverse)
                        / 255) as u8;
                    self.pixels[di + 2] = ((src.pixels[si + 2] as u16 * alpha
                        + self.pixels[di + 2] as u16 * inverse)
                        / 255) as u8;
                    self.pixels[di + 3] = 255;
                }
            }
        }
    }

    pub fn pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        let offset = (y as usize * self.width as usize + x as usize) * 4;
        Some(Color {
            r: self.pixels[offset],
            g: self.pixels[offset + 1],
            b: self.pixels[offset + 2],
            a: self.pixels[offset + 3],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_sets_every_pixel() {
        let mut surface = Surface::new(2, 2);
        surface.clear(Color::rgba(1, 2, 3, 4));
        assert_eq!(surface.pixel(0, 0), Some(Color::rgba(1, 2, 3, 4)));
        assert_eq!(surface.pixel(1, 1), Some(Color::rgba(1, 2, 3, 4)));
    }

    #[test]
    fn blit_alpha_blends() {
        let mut dst = Surface::new(1, 1);
        dst.clear(Color::rgb(0, 0, 0));

        let mut src = Surface::new(1, 1);
        src.clear(Color::rgba(255, 0, 0, 128));
        dst.blit(&src, 0, 0);

        let pixel = dst.pixel(0, 0).unwrap();
        assert_eq!(pixel.r, 128);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
        assert_eq!(pixel.a, 255);
    }

    #[test]
    fn blit_clips_negative_destination() {
        let mut dst = Surface::new(2, 2);
        dst.clear(Color::BLACK);

        let mut src = Surface::new(2, 2);
        src.clear(Color::WHITE);
        dst.blit(&src, -1, -1);

        assert_eq!(dst.pixel(0, 0), Some(Color::WHITE));
        assert_eq!(dst.pixel(1, 1), Some(Color::BLACK));
        assert_eq!(dst.pixel(2, 2), None);
    }

    #[test]
    fn fill_rect_clips() {
        let mut surface = Surface::new(2, 2);
        surface.fill_rect(
            Rect {
                x: -1,
                y: -1,
                w: 3,
                h: 3,
            },
            Color::rgb(9, 8, 7),
        );
        assert_eq!(surface.pixel(0, 0), Some(Color::rgb(9, 8, 7)));
        assert_eq!(surface.pixel(1, 1), Some(Color::rgb(9, 8, 7)));
    }
}
