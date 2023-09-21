use crate::drawing::*;
use crate::*;

use crate::component::*;

#[derive(Debug, Default)]
pub struct ButtonBuilder {
    sizing: Sizing,
    background: Color,
}

impl ButtonBuilder {
    pub fn with_label(mut self, text: &str) -> Self {
        // self.text.replace(text.into());
        self
    }

    pub fn with_action(mut self, action: Box<dyn FnMut() + 'static>) -> Self {
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn with_width(mut self, width: Length) -> Self {
        self.sizing.width.desired = width;
        self
    }

    pub fn with_height(mut self, height: Length) -> Self {
        self.sizing.height.desired = height;
        self
    }

    pub fn build(self) -> Box<Button> {
        Box::new(Button {
            sizing: self.sizing,
            background_color: self.background,
            background: Rectangle::new(Rect::default(), self.background),
            layout_dirty: true,
        })
    }
}

pub struct Button {
    sizing: Sizing,
    background_color: Color,
    background: Rectangle,
    layout_dirty: bool,
}

impl Button {
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

impl Component for Button {
    fn is_layout_dirty(&self) -> bool {
        self.layout_dirty
    }

    fn is_visually_dirty(&self) -> bool {
        todo!()
    }

    fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        self.sizing.calc_final_size(available, Size::default())
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        self.background.update(final_rect, self.background_color);
        self.layout_dirty = false;
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(&self.background)
    }
}
