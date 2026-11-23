use aaatw_core::Renderer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape,
    Enter,
    Space,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    Quit,
    KeyDown(Key),
    KeyUp(Key),
    MouseMove { x: i32, y: i32 },
    MouseButton { x: i32, y: i32, left: bool },
    Unknown,
}

pub trait Platform {
    fn poll_event(&mut self) -> Option<GameEvent>;
    fn renderer(&mut self) -> &mut dyn Renderer;
}
