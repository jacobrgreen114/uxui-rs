use crate::*;

use component::ComponentSizingExt;
use std::any::Any;
use std::cell::Cell;

/*
   Component
*/

#[derive(Default)]
pub struct ComponentBuilder {
    sizing: Sizing,
    children: Vec<Component>,
}

impl ComponentBuilder {
    const fn default() -> Self {
        ComponentBuilder {
            sizing: Sizing::default(),
            children: Vec::new(),
        }
    }

    pub const fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }

    pub fn with_children(mut self, children: Vec<Component>) -> Self {
        self.children = children;
        self
    }

    #[inline(always)]
    pub fn build(self, controller: impl ComponentController + 'static) -> Component {
        self.build_boxed(Box::new(controller))
    }

    fn build_boxed(self, controller: Box<dyn ComponentController>) -> Component {
        Component {
            controller,
            sizing: self.sizing,
            final_size: Cell::new(Size::zero()),
            final_rect: Cell::new(Rect::default()),
            children: self.children,
            parent: None,
            parent_data: None,
        }
    }
}

pub struct Component {
    controller: Box<dyn ComponentController>,
    sizing: Sizing,
    final_size: Cell<Size>,
    final_rect: Cell<Rect>,
    children: Vec<Component>,
    parent: Option<*const Component>,
    parent_data: Option<Box<dyn Any>>,
}

impl Component {
    pub const fn builder() -> ComponentBuilder {
        ComponentBuilder::default()
    }

    #[inline]
    pub fn final_size(&self) -> Size {
        self.final_size.get()
    }

    #[inline]
    pub fn final_rect(&self) -> Rect {
        self.final_rect.get()
    }

    #[inline]
    pub fn children(&self) -> &[Component] {
        &self.children
    }

    pub fn measure(&self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        let required = self.controller.measure(self, available);
        let requested = self.sizing.calc_final_size(available, required);
        self.final_size.set(requested);
        requested
    }

    pub fn arrange(&self, requested_rect: Rect) -> Rect {
        let final_rect = self.controller.arrange(self, requested_rect);
        self.final_rect.set(final_rect);
        final_rect
    }
}

/*
   Component Controller
*/

pub trait ComponentController: Layout + Draw + InputHandler {}

pub trait Layout {
    fn measure(&self, component: &Component, available_size: Size) -> Size;
    fn arrange(&self, component: &Component, requested_rect: Rect) -> Rect;
}

pub trait Draw {
    fn draw(&self);
}

pub trait InputHandler {}

/*
   Content Layout
*/

struct ContentLayout {}

impl Layout for ContentLayout {
    fn measure(&self, component: &Component, available_size: Size) -> Size {
        let children = component.children();
        debug_assert_eq!(children.len(), 1);
        children.first().unwrap().measure(available_size)
    }

    fn arrange(&self, component: &Component, requested_rect: Rect) -> Rect {
        let children = component.children();
        debug_assert_eq!(children.len(), 1);
        children.first().unwrap().arrange(requested_rect)
    }
}

/*
   Content Draw
*/

struct ContentDraw {}

impl Draw for ContentDraw {
    fn draw(&self) {
        todo!()
    }
}

/*
   Button
*/

struct ButtonBuilder {
    component: ComponentBuilder,
}

impl ButtonBuilder {
    pub fn build_component(self) -> Component {
        self.component.build(Button {
            layout: ContentLayout {},
            draw: ContentDraw {},
        })
    }

    pub fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.component = self.component.with_sizing(sizing);
        self
    }

    pub fn with_content(mut self, content: Component) -> Self {
        self.component = self.component.with_children(vec![content]);
        self
    }
}

struct Button {
    layout: ContentLayout,
    draw: ContentDraw,
}

impl Button {
    pub const fn builder() -> ButtonBuilder {
        ButtonBuilder {
            component: Component::builder(),
        }
    }
}

impl Layout for Button {
    fn measure(&self, component: &Component, available_size: Size) -> Size {
        self.layout.measure(component, available_size)
    }

    fn arrange(&self, component: &Component, requested_rect: Rect) -> Rect {
        self.layout.arrange(component, requested_rect)
    }
}

impl Draw for Button {
    fn draw(&self) {
        self.draw.draw()
    }
}

impl InputHandler for Button {}

impl ComponentController for Button {}

/*
   Custom Button
*/

struct MyButtonBuilder {}

impl MyButtonBuilder {
    pub fn build(self) -> Component {
        todo!()
    }
}

struct MyButton {
    button: Button,
}

impl MyButton {
    pub const fn builder() -> MyButtonBuilder {
        MyButtonBuilder {}
    }
}

impl Layout for MyButton {
    fn measure(&self, component: &Component, available_size: Size) -> Size {
        self.button.measure(component, available_size)
    }

    fn arrange(&self, component: &Component, requested_rect: Rect) -> Rect {
        self.button.arrange(component, requested_rect)
    }
}

impl Draw for MyButton {
    fn draw(&self) {
        self.button.draw()
    }
}

impl InputHandler for MyButton {}

impl ComponentController for MyButton {}

fn create_layout() -> Component {
    todo!()
}
