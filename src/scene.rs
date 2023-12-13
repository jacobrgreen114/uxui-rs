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

use crate::component::*;
use crate::drawing::*;
use crate::input_handling::*;
use crate::*;

use num_traits::Zero;
use std::cell::{RefCell, UnsafeCell};
use std::ops::{Deref, DerefMut};
use wgpu::core::instance::AdapterInputs;

// pub trait SceneInterface {
//     fn update_layout(&mut self, canvas_size: Size);
//     fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);
//     fn get_background_color(&self) -> Color;
//     fn on_canvas_size_changed(&mut self, canvas_size: Size);
//
//     //fn on_active_preview(&mut self);
//     fn on_active(&mut self);
//     //fn on_inactive_preview(&mut self);
//     fn on_inactive(&mut self);
// }

pub trait SceneController {
    fn on_init(&mut self, _scene: &Scene) {}
    //fn on_active_preview(&mut self, _scene: &Scene<Self>) {}
    fn on_active(&mut self, _scene: &Scene) {}
    //fn on_inactive_preview(&mut self, _scene: &Scene<Self>) {}
    fn on_inactive(&mut self, _scene: &Scene) {}
}

pub struct Scene {
    controller: RefCell<Box<dyn SceneController>>,
    root: UnsafeCell<Option<Box<dyn Component>>>,
    background_color: Color,
    layout_dirty: bool,
}

impl Scene {
    pub fn new(controller: impl SceneController + 'static) -> Self {
        let this = Self {
            controller: RefCell::new(Box::new(controller)),
            root: UnsafeCell::new(None),
            background_color: Color::rgb(1.0, 1.0, 1.0),
            layout_dirty: true,
        };
        this.controller.borrow_mut().on_init(&this);
        this
    }

    pub fn swap_root(&self, new: Option<Box<dyn Component>>) -> Option<Box<dyn Component>> {
        let mut r = unsafe { &mut *self.root.get() };
        let old = r.take();
        *r.deref_mut() = new;
        old
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub(crate) fn update_layout(&mut self, canvas_size: Size) {
        if let Some(root) = unsafe { &mut *self.root.get() } {
            // if self.layout_dirty || root.is_layout_dirty() {
            let measured_size = root.measure(canvas_size);
            root.arrange(Rect::new(Point::zero(), canvas_size).align_center(measured_size));
            //}
        }
        self.layout_dirty = false;
    }
    pub(crate) fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        // todo : figure out a lifetime fix for the UnsafeCell
        if let Some(root) = unsafe { &*self.root.get() } {
            root.draw(context);
        }
    }
    pub(crate) fn get_background_color(&self) -> Color {
        self.background_color
    }
    pub(crate) fn on_canvas_size_changed(&mut self, canvas_size: Size) {
        self.layout_dirty = true;
    }
    pub(crate) fn on_active(&mut self) {
        self.controller.borrow_mut().on_active(self);
    }
    pub(crate) fn on_inactive(&mut self) {
        self.controller.borrow_mut().on_inactive(self);
    }
}

impl InputHandler for Scene {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        match unsafe { &mut *self.root.get() } {
            Some(root) => root.dispatch_key(event),
            None => false,
        }
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        match unsafe { &mut *self.root.get() } {
            Some(root) => root.dispatch_mouse_button(event),
            None => false,
        }
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        match unsafe { &mut *self.root.get() } {
            Some(root) => root.dispatch_mouse_wheel(event),
            None => false,
        }
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        match unsafe { &mut *self.root.get() } {
            Some(root) => root.dispatch_cursor_moved(event),
            None => false,
        }
    }
}
