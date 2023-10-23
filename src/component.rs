use crate::drawing::*;
use crate::*;
use input_handling::{
    CursorMovedEvent, InputHandler, KeyEvent, MouseButtonEvent, MouseWheelEvent,
    PreviewInputHandler,
};
use std::cell::{RefCell, UnsafeCell};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Visibility {
    Visible,
    Hidden,
    Collapsed,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Visible
    }
}

pub trait ComponentController: PreviewInputHandler {
    // fn is_layout_dirty(&self) -> bool;
    // fn is_visually_dirty(&self) -> bool;

    fn measure(&mut self, available_size: Size, children: &[Component]) -> Size;
    fn arrange(&mut self, final_rect: Rect, children: &[Component]) -> Rect;

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

    #[inline(always)]
    pub fn build(self, controller: impl ComponentController + 'static) -> Component {
        self.build_boxed(Box::new(controller))
    }

    fn build_boxed(self, controller: Box<dyn ComponentController + 'static>) -> Component {
        Component {
            sizing: self.sizing,
            controller,
            children: ComponentChildren::None,
            visually_dirty: true,
            layout_dirty: true,
            final_size: Default::default(),
            final_rect: Default::default(),
            visibility: Default::default(),
        }
    }
}

enum ComponentChildren {
    None,
    Single(Box<Component>),
    Static(&'static [Component]),
    Owned(Vec<Component>),
}

impl ComponentChildren {
    fn as_slice(&self) -> &[Component] {
        match self {
            ComponentChildren::None => &[],
            ComponentChildren::Single(child) => std::slice::from_ref(child),
            ComponentChildren::Static(children) => children,
            ComponentChildren::Owned(children) => children,
        }
    }
}

pub struct Component {
    controller: Box<dyn ComponentController>,
    sizing: Sizing,
    children: ComponentChildren,
    final_size: Option<Size>,
    final_rect: Option<Rect>,
    visually_dirty: bool,
    layout_dirty: bool,
    visibility: Visibility,
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
        match self.visibility {
            Visibility::Collapsed => Size::zero(),
            _ => {
                let available = self.sizing.calc_available_size(available_size);
                let required = self.controller.measure(available, self.children.as_slice());
                let final_size = self.sizing.calc_final_size(available, required);
                self.final_size = Some(final_size);
                final_size
            }
        }
    }

    pub fn arrange(&mut self, final_rect: Rect) -> Rect {
        match self.visibility {
            Visibility::Collapsed => Rect::new(final_rect.pos, Size::zero()),
            _ => {
                let final_rect = self
                    .controller
                    .arrange(final_rect, self.children.as_slice());
                self.final_rect = Some(final_rect);
                self.visually_dirty = false;
                final_rect
            }
        }
    }

    pub fn arrange_from(&mut self, point: Point) -> Rect {
        self.arrange(Rect::new(point, self.final_size.unwrap()))
    }

    pub fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        match self.visibility {
            Visibility::Visible => {
                self.controller.draw(context);
            }
            Visibility::Hidden => {}
            Visibility::Collapsed => {}
        }

        self.controller.draw(context);
    }
}

impl InputHandler for Component {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        return if self.controller.on_key_preview(event) {
            true
        } else {
            self.controller.on_key(event)
        };
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.controller.on_mouse_button(event)
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.controller.on_mouse_wheel(event)
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        self.controller.on_cursor_moved(event)
    }
}

/*
   Sizing Extensions
*/

pub(crate) trait ComponentSizingExt {
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
