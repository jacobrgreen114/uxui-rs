use crate::drawing::*;
use crate::*;

#[derive(Default)]
pub struct ColumnBuilder {
    children: Option<Vec<Box<dyn Component>>>,
    sizing: Sizing,
}

impl ColumnBuilder {
    pub fn with_children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.replace(children);
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

    pub fn build(self) -> Box<Column> {
        Box::new(Column {
            children: self.children.unwrap_or_default(),
            sizing: self.sizing,
        })
    }
}

pub struct Column {
    children: Vec<Box<dyn Component>>,
    sizing: Sizing,
}

impl Column {
    pub fn build() -> ColumnBuilder {
        Default::default()
    }
}

impl Component for Column {
    fn is_layout_dirty(&self) -> bool {
        todo!()
    }

    fn is_visually_dirty(&self) -> bool {
        todo!()
    }

    fn measure(&mut self, available_size: Size) -> Size {
        let mut remaining_size = calculate_available_size(&self.sizing, available_size);
        let mut max_width = 0.0f32;

        for child in self.children.iter_mut() {
            let child_size = child.measure(remaining_size);
            remaining_size.height -= child_size.height;
            max_width = max_width.max(child_size.width);
        }

        calculate_final_size(
            &self.sizing,
            available_size,
            Size::new(max_width, available_size.height - remaining_size.height),
        )
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        for child in self.children.iter() {
            child.draw(context);
        }
    }
}

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

    fn draw(&self, ctx: &mut DrawingContext) {
        todo!()
    }
}
