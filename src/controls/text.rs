use crate::component::*;
use crate::drawing::*;
use crate::*;

use freetype as ft;

pub struct Text {
    text: BindableString,
}

impl Text {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            text: BindableString::Static(text.into()),
        })
    }
}

impl Component for Text {
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
