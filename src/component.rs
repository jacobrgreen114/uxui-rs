use crate::drawing::*;
use crate::*;
use input_handling::{
    CursorMovedEvent, InputHandler, KeyEvent, MouseButtonEvent, MouseWheelEvent,
    PreviewInputHandler,
};
use std::cell::{RefCell, UnsafeCell};

pub trait ComponentController: PreviewInputHandler {
    // fn is_layout_dirty(&self) -> bool;
    // fn is_visually_dirty(&self) -> bool;

    fn measure(&mut self, available_size: Size) -> Size;
    fn arrange(&mut self, final_rect: Rect) -> Rect;

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);
}

#[derive(Default)]
pub struct ComponentBuilder {
    sizing: Sizing,
    // children: Option<Vec<Component>>,
}

impl ComponentBuilder {
    pub const fn with_width(mut self, width: Length) -> Self {
        self.sizing.width.desired = width;
        self
    }

    pub const fn with_height(mut self, height: Length) -> Self {
        self.sizing.height.desired = height;
        self
    }

    // pub fn with_children(mut self, children: Vec<Component>) -> Self {
    //     self.children = Some(children);
    //     self
    // }

    pub fn build(self, controller: impl ComponentController + 'static) -> Component {
        Component {
            sizing: self.sizing,
            component: Box::new(controller),
            // children: Vec::new(),
            visually_dirty: true,
            layout_dirty: true,
            final_size: None,
            final_rect: None,
        }
    }
}

pub struct Component {
    component: Box<dyn ComponentController>,
    sizing: Sizing,
    // children: Vec<Component>,
    final_size: Option<Size>,
    final_rect: Option<Rect>,
    visually_dirty: bool,
    layout_dirty: bool,
}

impl Component {
    pub fn is_layout_dirty(&self) -> bool {
        self.layout_dirty
    }

    pub fn is_visually_dirty(&self) -> bool {
        self.visually_dirty
    }

    pub fn final_size(&self) -> Size {
        self.final_size.unwrap()
    }

    pub fn final_rect(&self) -> Rect {
        self.final_rect.unwrap()
    }

    pub fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        let required = self.component.measure(available);
        let final_size = self.sizing.calc_final_size(available, required);
        self.final_size = Some(final_size);
        final_size
    }

    pub fn arrange(&mut self, final_rect: Rect) -> Rect {
        self.final_rect = Some(self.component.arrange(final_rect));
        self.visually_dirty = false;
        self.final_rect.unwrap()
    }

    pub fn arrange_from(&mut self, point: Point) -> Rect {
        self.arrange(Rect::new(point, self.final_size.unwrap()))
    }

    pub fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        self.component.draw(context);
    }
}

impl InputHandler for Component {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        return if self.component.on_key_preview(event) {
            true
        } else {
            self.component.on_key(event)
        };
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.component.on_mouse_button(event)
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.component.on_mouse_wheel(event)
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        self.component.on_cursor_moved(event)
    }
}

trait ComponentSizingExt {
    fn calc_available_size(&self, available_size: Size) -> Size;
    fn calc_final_size(&self, available_size: Size, required_size: Size) -> Size;
}

impl ComponentSizingExt for Sizing {
    fn calc_available_size(&self, available_size: Size) -> Size {
        Size {
            width: match self.width.desired {
                Length::Fit | Length::Fill => {
                    available_size.width.max(self.width.min).min(self.width.max)
                }
                Length::Fixed(pixels) => pixels,
            },
            height: match self.height.desired {
                Length::Fit | Length::Fill => available_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fixed(pixels) => pixels,
            },
        }
    }

    fn calc_final_size(&self, available_size: Size, required_size: Size) -> Size {
        Size {
            width: match self.width.desired {
                Length::Fit => required_size.width.max(self.width.min).min(self.width.max),
                Length::Fill => available_size.width.max(self.width.min).min(self.width.max),
                Length::Fixed(pixels) => pixels,
            },
            height: match self.height.desired {
                Length::Fit => required_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fill => available_size
                    .height
                    .max(self.height.min)
                    .min(self.height.max),
                Length::Fixed(pixels) => pixels,
            },
        }
    }
}
