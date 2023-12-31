/*
  Copyright 2023 Jacob Green

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

use crate::Delta;
use Point;

type Key = winit::keyboard::PhysicalKey;
type KeyState = winit::event::ElementState;

type MouseButton = winit::event::MouseButton;
type ButtonState = winit::event::ElementState;

#[derive(Debug)]
pub struct KeyEvent {
    key: Key,
    state: KeyState,
}

impl KeyEvent {
    pub(crate) fn new(key: Key, state: KeyState) -> Self {
        Self { key, state }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn state(&self) -> KeyState {
        self.state
    }
}

#[derive(Debug)]
pub struct MouseButtonEvent {
    button: MouseButton,
    state: ButtonState,
}

impl MouseButtonEvent {
    pub(crate) fn new(button: MouseButton, state: ButtonState) -> Self {
        Self { button, state }
    }

    pub fn button(&self) -> MouseButton {
        self.button
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }
}

#[derive(Debug)]
pub struct MouseWheelEvent {
    delta: Delta,
}

impl MouseWheelEvent {
    pub(crate) fn new(delta: Delta) -> Self {
        Self { delta }
    }

    pub fn delta(&self) -> Delta {
        self.delta
    }
}

#[derive(Debug)]
pub struct CursorMovedEvent {
    pos: Point,
}

impl CursorMovedEvent {
    pub(crate) fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn pos(&self) -> Point {
        self.pos
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

pub trait DispatchInput: PreviewInputHandler + InputHandler {
    fn dispatch_key(&mut self, event: &KeyEvent) -> bool {
        self.on_key_preview(event) || self.on_key(event)
    }
    fn dispatch_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.on_mouse_button_preview(event) || self.on_mouse_button(event)
    }
    fn dispatch_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.on_mouse_wheel_preview(event) || self.on_mouse_wheel(event)
    }
    fn dispatch_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        self.on_cursor_moved_preview(event) || self.on_cursor_moved(event)
    }
}
