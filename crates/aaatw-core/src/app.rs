use crate::Result;
use crate::color::Color;
use crate::config::Config;
use crate::renderer::{AssetProvider, Renderer};
use crate::surface::Surface;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const SPLASH_HOLD: f64 = 0.5;
const SPLASH_FADE: f64 = 1.25;
const TITLEBACK_HOLD: f64 = 1.75;
const TITLE_HOLD: f64 = 2.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Splash(usize),
    Title,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionPhase {
    Hold,
    FadeOut,
    FadeIn,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: Option<Screen>,
    pub to: Screen,
    pub phase: TransitionPhase,
    pub elapsed: f64,
    pub hold_duration: f64,
    pub fade_duration: f64,
    pub fade_out: bool,
}

impl Transition {
    fn new(from: Option<Screen>, to: Screen, hold: f64, fade: f64, fade_out: bool) -> Self {
        let phase = if hold > 0.0 {
            TransitionPhase::Hold
        } else if fade > 0.0 && fade_out {
            TransitionPhase::FadeOut
        } else if fade > 0.0 {
            TransitionPhase::FadeIn
        } else {
            TransitionPhase::Hold
        };

        Self {
            from,
            to,
            phase,
            elapsed: 0.0,
            hold_duration: hold,
            fade_duration: fade,
            fade_out,
        }
    }

    fn duration(&self) -> f64 {
        match self.phase {
            TransitionPhase::Hold => self.hold_duration,
            TransitionPhase::FadeOut | TransitionPhase::FadeIn => self.fade_duration,
        }
    }
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

        let transition = Transition::new(
            Some(Screen::Splash(0)),
            Screen::Splash(1),
            SPLASH_HOLD,
            SPLASH_FADE,
            images.distributor.is_some(),
        );

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
            let mut transition = match self.transition.take() {
                Some(transition) => transition,
                None => {
                    self.start_next_transition();
                    if self.transition.is_none() {
                        break;
                    }
                    continue;
                }
            };

            let left = (transition.duration() - transition.elapsed).max(0.0);
            if remaining < left {
                transition.elapsed += remaining;
                self.transition = Some(transition);
                return;
            }

            remaining -= left;
            transition.elapsed = 0.0;

            match transition.phase {
                TransitionPhase::Hold => {
                    if transition.fade_duration > 0.0 && transition.fade_out {
                        transition.phase = TransitionPhase::FadeOut;
                        self.transition = Some(transition);
                        continue;
                    }

                    if transition.fade_duration > 0.0 {
                        transition.phase = TransitionPhase::FadeIn;
                        self.transition = Some(transition);
                        continue;
                    }

                    self.screen = transition.to;
                    self.start_next_transition();
                }
                TransitionPhase::FadeOut => {
                    transition.phase = TransitionPhase::FadeIn;
                    self.transition = Some(transition);
                }
                TransitionPhase::FadeIn => {
                    self.screen = transition.to;
                    self.start_next_transition();
                }
            }
        }
    }

    fn start_next_transition(&mut self) {
        let next = match self.screen {
            Screen::Splash(0) => Some((Screen::Splash(1), SPLASH_HOLD, SPLASH_FADE)),
            Screen::Splash(1) => Some((Screen::Splash(2), SPLASH_HOLD, SPLASH_FADE)),
            Screen::Splash(2) => Some((Screen::Splash(3), SPLASH_HOLD, SPLASH_FADE)),
            Screen::Splash(3) => Some((Screen::Splash(4), TITLEBACK_HOLD, 0.0)),
            Screen::Splash(4) => Some((Screen::Title, TITLE_HOLD, 0.0)),
            _ => None,
        };

        if let Some((to, hold, fade)) = next {
            let fade_out = fade > 0.0 && to != self.screen && self.screen_has_image(self.screen);
            self.transition = Some(Transition::new(Some(self.screen), to, hold, fade, fade_out));
        }
    }

    fn screen_has_image(&self, screen: Screen) -> bool {
        match screen {
            Screen::Splash(0) => self.images.distributor.is_some(),
            Screen::Splash(1) => self.images.publisher.is_some(),
            Screen::Splash(2) => self.images.spintopsplash.is_some(),
            Screen::Splash(3..=4) | Screen::Title => self.images.title_background.is_some(),
            _ => false,
        }
    }

    pub fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
        renderer.clear(Color::BLACK);

        if let Some(transition) = &self.transition {
            match transition.phase {
                TransitionPhase::Hold => {
                    if let Some(from) = transition.from {
                        self.draw_screen(renderer, from, 1.0);
                    }
                }
                TransitionPhase::FadeOut => {
                    if let Some(from) = transition.from {
                        let alpha = if transition.fade_duration > 0.0 {
                            1.0 - (transition.elapsed / transition.fade_duration).clamp(0.0, 1.0)
                                as f32
                        } else {
                            0.0
                        };
                        self.draw_screen(renderer, from, alpha);
                    }
                }
                TransitionPhase::FadeIn => {
                    let alpha = if transition.fade_duration > 0.0 {
                        (transition.elapsed / transition.fade_duration).clamp(0.0, 1.0) as f32
                    } else {
                        1.0
                    };
                    self.draw_screen(renderer, transition.to, alpha);
                }
            }
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
        } else if alpha > 0.0 {
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

    struct StubProvider {
        png: Vec<u8>,
    }

    impl StubProvider {
        fn new() -> Self {
            let mut bytes = Vec::new();
            let image = image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 0, 0, 255]));
            image::DynamicImage::ImageRgba8(image)
                .write_to(
                    &mut std::io::Cursor::new(&mut bytes),
                    image::ImageFormat::Png,
                )
                .unwrap();
            Self { png: bytes }
        }
    }

    impl AssetProvider for StubProvider {
        fn get(&self, _uri: &str) -> Option<Vec<u8>> {
            Some(self.png.clone())
        }
    }

    #[test]
    fn splash_holds_before_fading() {
        let mut app = App::new(
            &EmptyProvider,
            Config {
                skip_splash: false,
                ..Config::default()
            },
        )
        .unwrap();

        assert_eq!(app.screen, Screen::Splash(0));
        let transition = app.transition.as_ref().unwrap();
        assert_eq!(transition.from, Some(Screen::Splash(0)));
        assert_eq!(transition.to, Screen::Splash(1));
        assert_eq!(transition.phase, TransitionPhase::Hold);
        assert!((transition.hold_duration - SPLASH_HOLD).abs() < 1e-9);
        assert!((transition.fade_duration - SPLASH_FADE).abs() < 1e-9);

        app.update(0.25);
        assert_eq!(app.screen, Screen::Splash(0));
        assert_eq!(
            app.transition.as_ref().unwrap().phase,
            TransitionPhase::Hold
        );
        assert!((app.transition.as_ref().unwrap().elapsed - 0.25).abs() < 1e-9);

        app.update(0.3);
        assert_eq!(app.screen, Screen::Splash(0));
        assert_eq!(
            app.transition.as_ref().unwrap().phase,
            TransitionPhase::FadeIn
        );
    }

    #[test]
    fn transition_fades_through_black() {
        let mut app = App::new(
            &StubProvider::new(),
            Config {
                skip_splash: false,
                ..Config::default()
            },
        )
        .unwrap();

        app.screen = Screen::Splash(1);
        app.transition = Some(Transition::new(
            Some(Screen::Splash(1)),
            Screen::Splash(2),
            0.0,
            1.25,
            true,
        ));

        app.update(0.1);
        assert_eq!(app.screen, Screen::Splash(1));
        assert_eq!(
            app.transition.as_ref().unwrap().phase,
            TransitionPhase::FadeOut
        );

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(1));
        assert_eq!(
            app.transition.as_ref().unwrap().phase,
            TransitionPhase::FadeIn
        );

        app.update(1.3);
        assert_eq!(app.screen, Screen::Splash(2));
    }

    #[test]
    fn splash_sequence_reaches_title() {
        let mut app = App::new(
            &StubProvider::new(),
            Config {
                skip_splash: false,
                ..Config::default()
            },
        )
        .unwrap();

        app.update(13.4);
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
