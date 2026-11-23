use std::time::{Duration, Instant};

use aaatw_core::{App, Error, Result};
use aaatw_platform::{GameEvent, Key, Platform};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::renderer::Sdl2Renderer;

pub struct Sdl2App {
    _sdl: sdl2::Sdl,
    _video: sdl2::VideoSubsystem,
    renderer: Sdl2Renderer,
    event_pump: sdl2::EventPump,
}

impl Sdl2App {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl = sdl2::init().map_err(|error| Error::Other(error.to_string()))?;
        let video = sdl
            .video()
            .map_err(|error| Error::Other(error.to_string()))?;
        let window = video
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|error| Error::Other(error.to_string()))?;
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|error| Error::Other(error.to_string()))?;
        let texture_creator = canvas.texture_creator();
        let renderer = Sdl2Renderer::new(canvas, texture_creator);
        let event_pump = sdl
            .event_pump()
            .map_err(|error| Error::Other(error.to_string()))?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            renderer,
            event_pump,
        })
    }

    pub fn run(mut self, app: &mut App) -> Result<()> {
        let frame_time = Duration::from_secs_f64(1.0 / 60.0);
        let mut last = Instant::now();

        loop {
            let now = Instant::now();
            let dt = now.duration_since(last).as_secs_f64();
            last = now;

            let mut quit = false;
            while let Some(event) = self.poll_event() {
                if event == GameEvent::Quit {
                    quit = true;
                }
            }
            if quit {
                break;
            }

            app.update(dt);
            app.draw(self.renderer())?;

            let elapsed = now.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }

        Ok(())
    }
}

impl Platform for Sdl2App {
    fn poll_event(&mut self) -> Option<GameEvent> {
        self.event_pump.poll_event().map(map_event)
    }

    fn renderer(&mut self) -> &mut dyn aaatw_core::Renderer {
        &mut self.renderer
    }
}

fn map_event(event: Event) -> GameEvent {
    match event {
        Event::Quit { .. } => GameEvent::Quit,
        Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => GameEvent::KeyDown(map_key(keycode)),
        Event::KeyUp {
            keycode: Some(keycode),
            ..
        } => GameEvent::KeyUp(map_key(keycode)),
        Event::MouseMotion { x, y, .. } => GameEvent::MouseMove { x, y },
        Event::MouseButtonDown {
            x,
            y,
            mouse_btn: sdl2::mouse::MouseButton::Left,
            ..
        } => GameEvent::MouseButton { x, y, left: true },
        Event::MouseButtonUp {
            x,
            y,
            mouse_btn: sdl2::mouse::MouseButton::Left,
            ..
        } => GameEvent::MouseButton { x, y, left: true },
        _ => GameEvent::Unknown,
    }
}

fn map_key(keycode: Keycode) -> Key {
    match keycode {
        Keycode::Escape => Key::Escape,
        Keycode::Return | Keycode::KpEnter => Key::Enter,
        Keycode::Space => Key::Space,
        _ => Key::Unknown,
    }
}
