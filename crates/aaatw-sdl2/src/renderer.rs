use aaatw_core::{Color, Renderer, Result, Surface};
use sdl2::pixels::{Color as SdlColor, PixelFormatEnum};
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{BlendMode, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Sdl2Renderer {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

impl Sdl2Renderer {
    pub fn new(canvas: WindowCanvas, texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            canvas,
            texture_creator,
        }
    }

    pub fn canvas_mut(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}

impl Renderer for Sdl2Renderer {
    fn clear(&mut self, color: Color) {
        self.canvas
            .set_draw_color(SdlColor::RGBA(color.r, color.g, color.b, color.a));
        self.canvas.clear();
    }

    fn draw_surface(&mut self, surface: &Surface, x: i32, y: i32) {
        let mut texture = match self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGBA32,
            surface.width,
            surface.height,
        ) {
            Ok(texture) => texture,
            Err(_) => return,
        };

        if texture
            .update(None, &surface.pixels, surface.width as usize * 4)
            .is_err()
        {
            return;
        }

        texture.set_blend_mode(BlendMode::BLEND);

        let target = SdlRect::new(x, y, surface.width, surface.height);
        let _ = self.canvas.copy(&texture, None, target);
    }

    fn draw_surface_alpha(&mut self, surface: &Surface, x: i32, y: i32, alpha: f32) {
        let alpha = alpha.clamp(0.0, 1.0);
        let mut texture = match self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGBA32,
            surface.width,
            surface.height,
        ) {
            Ok(texture) => texture,
            Err(_) => return,
        };

        if texture
            .update(None, &surface.pixels, surface.width as usize * 4)
            .is_err()
        {
            return;
        }

        texture.set_blend_mode(BlendMode::BLEND);
        texture.set_alpha_mod((alpha * 255.0).round() as u8);

        let target = SdlRect::new(x, y, surface.width, surface.height);
        let _ = self.canvas.copy(&texture, None, target);
    }

    fn present(&mut self) -> Result<()> {
        self.canvas.present();
        Ok(())
    }
}
