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

use crate::*;
use component::*;
use drawing::DrawingContext;
use input_handling::*;

use std::cell::Cell;
use num_traits::Zero;

#[derive(Debug, Default)]
pub struct DockBuilder {
    sizing: Sizing,
    center: Option<Box<dyn Component>>,
    top: Option<Box<dyn Component>>,
    bottom: Option<Box<dyn Component>>,
    left: Option<Box<dyn Component>>,
    right: Option<Box<dyn Component>>,
}

impl DockBuilder {
    pub fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }

    pub fn with_center(mut self, center: Box<dyn Component>) -> Self {
        self.center = Some(center);
        self
    }

    pub fn with_top(mut self, top: Box<dyn Component>) -> Self {
        self.top = Some(top);
        self
    }

    pub fn with_bottom(mut self, bottom: Box<dyn Component>) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn with_left(mut self, left: Box<dyn Component>) -> Self {
        self.left = Some(left);
        self
    }

    pub fn with_right(mut self, right: Box<dyn Component>) -> Self {
        self.right = Some(right);
        self
    }
}

impl Builder<Dock> for DockBuilder {
    fn build(self) -> Dock {
        Dock {
            sizing: self.sizing,
            center: self.center.map(|c| c.into()),
            top: self.top.map(|c| c.into()),
            bottom: self.bottom.map(|c| c.into()),
            left: self.left.map(|c| c.into()),
            right: self.right.map(|c| c.into()),
            final_size: Cell::new(Size::zero()),
            final_rect: Cell::new(Rect::default()),
        }
    }
}

#[derive(Debug)]
struct DockChild {
    component: Box<dyn Component>,
    final_size: Cell<Size>,
}

impl From<Box<dyn Component>> for DockChild {
    fn from(component: Box<dyn Component>) -> Self {
        Self {
            component,
            final_size: Cell::new(Size::zero()),
        }
    }
}

#[derive(Debug)]
pub struct Dock {
    sizing: Sizing,
    center: Option<DockChild>,
    top: Option<DockChild>,
    bottom: Option<DockChild>,
    left: Option<DockChild>,
    right: Option<DockChild>,

    final_size: Cell<Size>,
    final_rect: Cell<Rect>,
}

impl Dock {
    pub fn builder() -> DockBuilder {
        DockBuilder::default()
    }
}

impl Layout for Dock {
    fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        let required = {
            let mut remaining = available;

            let t = self
                .top
                .as_mut()
                .map(|c| {
                    let s = c.component.measure(remaining);
                    c.final_size.set(s);
                    s
                })
                .unwrap_or_default();
            remaining.height -= t.height;

            let b = self
                .bottom
                .as_mut()
                .map(|c| {
                    let s = c.component.measure(remaining);
                    c.final_size.set(s);
                    s
                })
                .unwrap_or_default();
            remaining.height -= b.height;

            let l = self
                .left
                .as_mut()
                .map(|c| {
                    let s = c.component.measure(remaining);
                    c.final_size.set(s);
                    s
                })
                .unwrap_or_default();
            remaining.width -= l.width;

            let r = self
                .right
                .as_mut()
                .map(|c| {
                    let s = c.component.measure(remaining);
                    c.final_size.set(s);
                    s
                })
                .unwrap_or_default();
            remaining.width -= r.width;

            let c = self
                .center
                .as_mut()
                .map(|c| {
                    let s = c.component.measure(remaining);
                    c.final_size.set(s);
                    s
                })
                .unwrap_or_default();

            let width = [t.width, b.width, l.width + c.width + r.width]
                .iter()
                .cloned()
                .reduce(f32::max)
                .unwrap();

            let height = [l.height, r.height, t.height + c.height + b.height]
                .iter()
                .cloned()
                .reduce(f32::max)
                .unwrap();

            Size::new(width, height)
        };
        let final_size = self.sizing.calc_final_size(available, required);
        self.final_size.set(final_size);
        final_size
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        let t = self
            .top
            .as_ref()
            .map(|c| c.final_size.get())
            .unwrap_or_default();
        let b = self
            .bottom
            .as_ref()
            .map(|c| c.final_size.get())
            .unwrap_or_default();
        let l = self
            .left
            .as_ref()
            .map(|c| c.final_size.get())
            .unwrap_or_default();
        let r = self
            .right
            .as_ref()
            .map(|c| c.final_size.get())
            .unwrap_or_default();
        let c = self
            .center
            .as_ref()
            .map(|c| c.final_size.get())
            .unwrap_or_default();

        let tr = self
            .top
            .as_mut()
            .map(|c| {
                c.component.arrange(Rect::new(
                    final_rect.pos,
                    Size::new(final_rect.size.width, t.height),
                ))
            })
            .unwrap_or_default();

        let br = self
            .bottom
            .as_mut()
            .map(|c| {
                c.component.arrange(Rect::new(
                    Point::new(
                        final_rect.pos.x,
                        final_rect.pos.y + final_rect.size.height - b.height,
                    ),
                    Size::new(final_rect.size.width, b.height),
                ))
            })
            .unwrap_or_default();

        let lr = self
            .left
            .as_mut()
            .map(|c| {
                c.component.arrange(Rect::new(
                    Point::new(final_rect.pos.x, final_rect.pos.y + t.height),
                    Size::new(l.width, final_rect.size.height - t.height - b.height),
                ))
            })
            .unwrap_or_default();

        let rr = self
            .right
            .as_mut()
            .map(|c| {
                c.component.arrange(Rect::new(
                    Point::new(
                        final_rect.pos.x + final_rect.size.width - r.width,
                        final_rect.pos.y + t.height,
                    ),
                    Size::new(r.width, final_rect.size.height - t.height - b.height),
                ))
            })
            .unwrap_or_default();

        let cr = self
            .center
            .as_mut()
            .map(|c| {
                c.component.arrange(Rect::new(
                    Point::new(final_rect.pos.x + l.width, final_rect.pos.y + t.height),
                    Size::new(
                        final_rect.size.width - l.width - r.width,
                        final_rect.size.height - t.height - b.height,
                    ),
                ))
            })
            .unwrap_or_default();

        self.final_rect.set(final_rect);
        final_rect
    }
}

impl Draw for Dock {
    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        self.center.as_ref().map(|c| c.component.draw(context));
        self.right.as_ref().map(|c| c.component.draw(context));
        self.left.as_ref().map(|c| c.component.draw(context));
        self.bottom.as_ref().map(|c| c.component.draw(context));
        self.top.as_ref().map(|c| c.component.draw(context));
    }
}

impl PreviewInputHandler for Dock {}

impl InputHandler for Dock {}

impl DispatchInput for Dock {
    fn dispatch_key(&mut self, event: &KeyEvent) -> bool {
        self.on_key_preview(event)
            || self
                .center
                .as_mut()
                .map_or(false, |c| c.component.dispatch_key(event))
            || self
                .top
                .as_mut()
                .map_or(false, |c| c.component.dispatch_key(event))
            || self
                .left
                .as_mut()
                .map_or(false, |c| c.component.dispatch_key(event))
            || self
                .right
                .as_mut()
                .map_or(false, |c| c.component.dispatch_key(event))
            || self
                .bottom
                .as_mut()
                .map_or(false, |c| c.component.dispatch_key(event))
            || self.on_key(event)
    }

    fn dispatch_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.on_mouse_button_preview(event)
            || self
                .center
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_button(event))
            || self
                .top
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_button(event))
            || self
                .left
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_button(event))
            || self
                .right
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_button(event))
            || self
                .bottom
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_button(event))
            || self.on_mouse_button(event)
    }

    fn dispatch_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.on_mouse_wheel_preview(event)
            || self
                .center
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_wheel(event))
            || self
                .top
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_wheel(event))
            || self
                .left
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_wheel(event))
            || self
                .right
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_wheel(event))
            || self
                .bottom
                .as_mut()
                .map_or(false, |c| c.component.dispatch_mouse_wheel(event))
            || self.on_mouse_wheel(event)
    }

    fn dispatch_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        if self.final_rect.get().contains(event.pos()) {
            self.on_cursor_moved_preview(event)
                || self
                    .center
                    .as_mut()
                    .map_or(false, |c| c.component.dispatch_cursor_moved(event))
                || self
                    .top
                    .as_mut()
                    .map_or(false, |c| c.component.dispatch_cursor_moved(event))
                || self
                    .left
                    .as_mut()
                    .map_or(false, |c| c.component.dispatch_cursor_moved(event))
                || self
                    .right
                    .as_mut()
                    .map_or(false, |c| c.component.dispatch_cursor_moved(event))
                || self
                    .bottom
                    .as_mut()
                    .map_or(false, |c| c.component.dispatch_cursor_moved(event))
                || self.on_cursor_moved(event)
        } else {
            false
        }
    }
}

impl Component for Dock {}
