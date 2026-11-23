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

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: Option<Screen>,
    pub to: Screen,
    pub elapsed: f64,
    pub duration: f64,
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
        let distributor = load_surface(provider, "std:distributor.jpg");
        let publisher = load_surface(provider, "std:publisher.jpg");
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
    pub transition: Option<Transition>,
    pub images: SplashImages,
    pub config: Config,
}

impl App {
    pub fn new(provider: &dyn AssetProvider, config: Config) -> Result<Self> {
        let images = SplashImages::load(provider)?;

        if config.skip_splash {
            return Ok(Self {
                screen: Screen::Title,
                transition: None,
                images,
                config,
            });
        }

        let transition = Transition {
            from: None,
            to: Screen::Splash(0),
            elapsed: 0.0,
            duration: SPLASH_DURATIONS[0],
        };

        Ok(Self {
            screen: Screen::Splash(0),
            transition: Some(transition),
            images,
            config,
        })
    }

    pub fn update(&mut self, dt: f64) {
        let mut remaining = dt;

        while remaining > 0.0 {
            if let Some(transition) = &mut self.transition {
                let left = transition.duration - transition.elapsed;
                if remaining < left {
                    transition.elapsed += remaining;
                    return;
                }

                remaining -= left;
                self.screen = transition.to;
                self.transition = None;

                if let Screen::Splash(index) = self.screen {
                    if index + 1 < SPLASH_DURATIONS.len() {
                        self.transition = Some(Transition {
                            from: Some(self.screen),
                            to: Screen::Splash(index + 1),
                            elapsed: 0.0,
                            duration: SPLASH_DURATIONS[index],
                        });
                    } else {
                        self.transition = Some(Transition {
                            from: Some(self.screen),
                            to: Screen::Title,
                            elapsed: 0.0,
                            duration: SPLASH_DURATIONS[index],
                        });
                    }
                }
                continue;
            }

            if let Screen::Splash(index) = self.screen {
                if index + 1 < SPLASH_DURATIONS.len() {
                    self.transition = Some(Transition {
                        from: Some(self.screen),
                        to: Screen::Splash(index + 1),
                        elapsed: 0.0,
                        duration: SPLASH_DURATIONS[index],
                    });
                } else {
                    self.transition = Some(Transition {
                        from: Some(self.screen),
                        to: Screen::Title,
                        elapsed: 0.0,
                        duration: SPLASH_DURATIONS[index],
                    });
                }
                continue;
            }

            break;
        }
    }

    pub fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.clear(Color::BLACK);

        if let Some(transition) = &self.transition {
            if let Some(from) = transition.from {
                self.draw_screen(renderer, from, 1.0);
            }
            let alpha = (transition.elapsed / transition.duration).clamp(0.0, 1.0) as f32;
            self.draw_screen(renderer, transition.to, alpha);
        } else {
            self.draw_screen(renderer, self.screen, 1.0);
        }

        renderer.present()
    }

    fn draw_screen(&self, renderer: &mut dyn Renderer, screen: Screen, alpha: f32) {
        match screen {
            Screen::Splash(0) => draw_centered(renderer, self.images.distributor.as_ref(), alpha),
            Screen::Splash(1) => draw_centered(renderer, self.images.publisher.as_ref(), alpha),
            Screen::Splash(2) => draw_at(renderer, self.images.spintopsplash.as_ref(), 0, 0, alpha),
            Screen::Splash(3..=4) | Screen::Title => {
                draw_at(renderer, self.images.title_background.as_ref(), 0, 0, alpha)
            }
            Screen::Splash(_) | Screen::Quit => {}
        }
    }
}

fn draw_at(renderer: &mut dyn Renderer, surface: Option<&Surface>, x: i32, y: i32, alpha: f32) {
    if let Some(surface) = surface {
        if alpha >= 0.999 {
            renderer.draw_surface(surface, x, y);
        } else {
            renderer.draw_surface_alpha(surface, x, y, alpha);
        }
    }
}

fn draw_centered(renderer: &mut dyn Renderer, surface: Option<&Surface>, alpha: f32) {
    if let Some(surface) = surface {
        let x = (SCREEN_WIDTH - surface.width as i32) / 2;
        let y = (SCREEN_HEIGHT - surface.height as i32) / 2;
        draw_at(renderer, Some(surface), x.max(0), y.max(0), alpha);
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
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Splash(0));

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(0));
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Splash(1));

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(1));
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Splash(2));

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(2));
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Splash(3));

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(3));
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Splash(4));

        app.update(1.8);
        assert_eq!(app.screen, Screen::Splash(4));
        assert_eq!(app.transition.as_ref().unwrap().to, Screen::Title);

        app.update(2.6);
        assert_eq!(app.screen, Screen::Title);
        assert!(app.transition.is_none());
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
        assert!(app.transition.is_none());
    }
}
