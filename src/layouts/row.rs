use crate::component::*;
use crate::drawing::*;
use crate::*;

pub struct Row {}

impl Component for Row {
    fn is_layout_dirty(&self) -> bool {
        todo!()
    }

    fn is_visually_dirty(&self) -> bool {
        todo!()
    }

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
