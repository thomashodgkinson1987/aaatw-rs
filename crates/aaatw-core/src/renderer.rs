use crate::Result;
use crate::color::Color;
use crate::surface::Surface;

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn draw_surface(&mut self, surface: &Surface, x: i32, y: i32);
    fn present(&mut self) -> Result<()>;
}

pub trait AssetProvider {
    fn get(&self, uri: &str) -> Option<Vec<u8>>;
}
