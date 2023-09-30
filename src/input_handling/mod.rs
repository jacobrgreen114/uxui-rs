use crate::Delta;
use Point;

type Key = winit::event::VirtualKeyCode;
type KeyState = winit::event::ElementState;

type MouseButton = winit::event::MouseButton;
type ButtonState = winit::event::ElementState;

#[derive(Debug)]
pub struct KeyEvent {
    pub key: Key,
    pub state: KeyState,
    pub(crate) _phantom: (),
}

#[derive(Debug)]
pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub state: ButtonState,
    pub(crate) _phantom: (),
}

#[derive(Debug)]
pub struct MouseWheelEvent {
    pub delta: Delta,
    pub(crate) _phantom: (),
}

#[derive(Debug)]
pub struct CursorMovedEvent {
    pub pos: Point,
    pub(crate) _phantom: (),
}

pub trait InputHandler {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        false
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        false
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        false
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        false
    }
}

pub trait PreviewInputHandler: InputHandler {
    fn on_key_preview(&mut self, event: &KeyEvent) -> bool {
        false
    }

    fn on_mouse_button_preview(&mut self, event: &MouseButtonEvent) -> bool {
        false
    }

    fn on_mouse_wheel_preview(&mut self, event: &MouseWheelEvent) -> bool {
        false
    }

    fn on_cursor_moved_preview(&mut self, event: &CursorMovedEvent) -> bool {
        false
    }
}
