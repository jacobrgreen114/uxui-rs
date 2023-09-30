use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::{InputHandler, PreviewInputHandler};

pub struct Row {}

impl InputHandler for Row {}

impl PreviewInputHandler for Row {}

impl ComponentController for Row {
    fn measure(&mut self, available_size: Size) -> Size {
        todo!()
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        todo!()
    }

    fn draw(&self, context: &mut DrawingContext) {
        todo!()
    }
}
