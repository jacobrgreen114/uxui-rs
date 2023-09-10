use crate::drawing::*;
use crate::*;

use std::cell::{RefCell, UnsafeCell};
use std::ops::{Deref, DerefMut};

pub trait SceneInterface {
    fn update_layout(&self, canvas_size: Size);

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);

    fn get_background_color(&self) -> Color;
}

pub trait SceneController: Sized + 'static {
    fn on_init(&mut self, _scene: &Scene<Self>) {}
    fn on_active_preview(&mut self, _scene: &Scene<Self>) {}
    fn on_active(&mut self, _scene: &Scene<Self>) {}
    fn on_inactive_preview(&mut self, _scene: &Scene<Self>) {}
    fn on_inactive(&mut self, _scene: &Scene<Self>) {}
}

pub struct Scene<C>
where
    C: SceneController,
{
    controller: RefCell<C>,
    root: UnsafeCell<Option<Box<dyn Component>>>,
    background_color: Color,
}

impl<C> Scene<C>
where
    C: SceneController,
{
    pub fn new(controller: C) -> Box<Self> {
        let mut s = Self {
            controller: RefCell::new(controller),
            root: UnsafeCell::new(None),
            background_color: Color::new_rgb(1.0, 1.0, 1.0),
        };
        s.controller.borrow_mut().on_init(&s);
        Box::new(s)
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
}

impl<C> SceneInterface for Scene<C>
where
    C: SceneController,
{
    fn update_layout(&self, canvas_size: Size) {
        if let Some(root) = unsafe { &mut *self.root.get() } {
            let measured_size = root.measure(canvas_size);
            root.arrange(Rect::new(Point::new(0.0, 0.0), canvas_size));
        }
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        // todo : figure out a fix for the UnsafeCell
        if let Some(root) = unsafe { &*self.root.get() } {
            root.draw(context);
        }
    }

    fn get_background_color(&self) -> Color {
        self.background_color
    }
}

fn center_rect(rect: Rect, size: Size) -> Rect {
    let x = (rect.size.width - size.width) / 2.0;
    let y = (rect.size.height - size.height) / 2.0;
    Rect::new(Point::new(x, y), size)
}
