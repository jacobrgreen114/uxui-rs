use crate::component::*;
use crate::drawing::*;
use crate::*;

use num_traits::Zero;
use std::cell::{RefCell, UnsafeCell};
use std::ops::{Deref, DerefMut};

pub trait SceneInterface {
    fn update_layout(&mut self, canvas_size: Size);

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>);

    fn get_background_color(&self) -> Color;

    fn on_canvas_size_changed(&mut self, canvas_size: Size);
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
    layout_dirty: bool,
}

impl<C> Scene<C>
where
    C: SceneController,
{
    pub fn new(controller: C) -> Box<Self> {
        let this = Box::new(Self {
            controller: RefCell::new(controller),
            root: UnsafeCell::new(None),
            background_color: Color::new_rgb(1.0, 1.0, 1.0),
            layout_dirty: true,
        });
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
}

impl<C> SceneInterface for Scene<C>
where
    C: SceneController,
{
    fn update_layout(&mut self, canvas_size: Size) {
        if let Some(root) = unsafe { &mut *self.root.get() } {
            if self.layout_dirty || root.is_layout_dirty() {
                let measured_size = root.measure(canvas_size);
                root.arrange(Rect::new(Point::zero(), canvas_size).align_center(measured_size));
            }
        }
        self.layout_dirty = false;
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

    fn on_canvas_size_changed(&mut self, canvas_size: Size) {
        self.layout_dirty = true;
    }
}
