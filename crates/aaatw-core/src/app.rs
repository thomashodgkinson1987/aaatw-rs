use crate::Result;
use crate::color::Color;
use crate::config::Config;
use crate::renderer::{AssetProvider, Renderer};
use crate::surface::Surface;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const SPLASH_DURATIONS: [f64; 5] = [1.25, 1.25, 1.25, 1.75, 2.5];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Splash(usize),
    Title,
    Quit,
}

#[derive(Debug, Default, Clone)]
pub struct SplashImages {
    pub distributor: Option<Surface>,
    pub publisher: Option<Surface>,
    pub spintopsplash: Option<Surface>,
    pub title_background: Option<Surface>,
}

impl SplashImages {
    pub fn load(provider: &dyn AssetProvider) -> Result<Self> {
        let distributor = load_surface(provider, "distributor.jpg")
            .or_else(|| load_surface(provider, "std:distributor.jpg"))
            .or_else(|| load_surface(provider, "drm/logo.jpg"));
        let publisher = load_surface(provider, "publisher.jpg")
            .or_else(|| load_surface(provider, "std:publisher.jpg"));
        let spintopsplash = load_surface(provider, "spintopsplash.jpg");
        let title_background = load_surface(provider, "title_background.jpg");

        Ok(Self {
            distributor,
            publisher,
            spintopsplash,
            title_background,
        })
    }
}

fn load_surface(provider: &dyn AssetProvider, uri: &str) -> Option<Surface> {
    let data = provider.get(uri)?;
    Surface::from_encoded(&data).ok()
}

pub struct App {
    pub screen: Screen,
    pub images: SplashImages,
    pub elapsed: f64,
    pub config: Config,
}

impl App {
    pub fn new(provider: &dyn AssetProvider, config: Config) -> Result<Self> {
        let images = SplashImages::load(provider)?;
        let screen = if config.skip_splash {
            Screen::Title
        } else {
            Screen::Splash(0)
        };

        Ok(Self {
            screen,
            images,
            elapsed: 0.0,
            config,
        })
    }

    pub fn update(&mut self, dt: f64) {
        let Screen::Splash(index) = self.screen else {
            return;
        };

        self.elapsed += dt;
        if self.elapsed < SPLASH_DURATIONS[index] {
            return;
        }

        self.elapsed = 0.0;
        self.screen = if index + 1 < SPLASH_DURATIONS.len() {
            Screen::Splash(index + 1)
        } else {
            Screen::Title
        };
    }

    pub fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.clear(Color::BLACK);

        match self.screen {
            Screen::Splash(0) => draw_centered(renderer, self.images.distributor.as_ref()),
            Screen::Splash(1) => draw_centered(renderer, self.images.publisher.as_ref()),
            Screen::Splash(2) => draw_at(renderer, self.images.spintopsplash.as_ref(), 0, 0),
            Screen::Splash(3..=4) | Screen::Title => {
                draw_at(renderer, self.images.title_background.as_ref(), 0, 0);
            }
            Screen::Splash(_) | Screen::Quit => {}
        }

        renderer.present()
    }
}

fn draw_at(renderer: &mut dyn Renderer, surface: Option<&Surface>, x: i32, y: i32) {
    if let Some(surface) = surface {
        renderer.draw_surface(surface, x, y);
    }
}

fn draw_centered(renderer: &mut dyn Renderer, surface: Option<&Surface>) {
    if let Some(surface) = surface {
        let x = (SCREEN_WIDTH - surface.width as i32) / 2;
        let y = (SCREEN_HEIGHT - surface.height as i32) / 2;
        renderer.draw_surface(surface, x.max(0), y.max(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::AssetProvider;

    struct EmptyProvider;

    impl AssetProvider for EmptyProvider {
        fn get(&self, _uri: &str) -> Option<Vec<u8>> {
            None
        }
    }

    #[test]
    fn splash_sequence_advances() {
        let mut app = App::new(
            &EmptyProvider,
            Config {
                skip_splash: false,
                ..Config::default()
            },
        )
        .unwrap();

        assert_eq!(app.screen, Screen::Splash(0));
        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(1));
        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(2));
        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(3));
        app.update(1.8);
        assert_eq!(app.screen, Screen::Splash(4));
        app.update(2.6);
        assert_eq!(app.screen, Screen::Title);
    }

    #[test]
    fn skip_splash_starts_at_title() {
        let app = App::new(
            &EmptyProvider,
            Config {
                skip_splash: true,
                ..Config::default()
            },
        )
        .unwrap();
        assert_eq!(app.screen, Screen::Title);
    }
}
