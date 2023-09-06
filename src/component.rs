use crate::drawing::*;
use crate::*;

pub trait ComponentInterface {
    fn measure(&self, available_size: Size);
    fn arrange(&self, final_rect: Rect);

    fn draw(&self, context: &mut DrawingContext) {
        todo!()
    }
}

pub trait ComponentController {}

pub struct Component<C>
where
    C: ComponentController,
{
    controller: C,
}

impl<C> Component<C>
where
    C: ComponentController,
{
    pub fn new(controller: C) -> Self {
        Self { controller }
    }
}

impl<C> ComponentInterface for Component<C>
where
    C: ComponentController,
{
    fn measure(&self, available_size: Size) {
        todo!()
    }

    fn arrange(&self, final_rect: Rect) {
        todo!()
    }
}
