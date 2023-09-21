use crate::component::*;
use crate::drawing::*;
use crate::*;

#[derive(Default)]
pub struct ColumnBuilder {
    children: Option<Vec<Box<dyn Component>>>,
    sizing: Sizing,
    horiz_align: Option<HorizontalAlignment>,
}

impl ColumnBuilder {
    pub fn with_children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.replace(children);
        self
    }

    pub const fn with_width(mut self, width: Length) -> Self {
        self.sizing.width.desired = width;
        self
    }

    pub const fn with_height(mut self, height: Length) -> Self {
        self.sizing.height.desired = height;
        self
    }

    pub const fn with_horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horiz_align = Some(alignment);
        self
    }

    pub fn build(self) -> Box<Column> {
        Box::new(Column {
            children: self
                .children
                .unwrap_or_default()
                .into_iter()
                .map(|c| ColumnChild {
                    component: c,
                    final_size: Size::default(),
                    final_rect: Rect::default(),
                })
                .collect(),
            sizing: self.sizing,
            horiz_align: self.horiz_align.unwrap_or(HorizontalAlignment::Left),
        })
    }
}

struct ColumnChild {
    component: Box<dyn Component>,
    final_size: Size,
    final_rect: Rect,
}

pub struct Column {
    children: Vec<ColumnChild>,
    sizing: Sizing,
    horiz_align: HorizontalAlignment,
}

impl Column {
    pub fn build() -> ColumnBuilder {
        Default::default()
    }

    fn arrange_left(&mut self, final_rect: Rect) -> Rect {
        let mut y = final_rect.pos.y;
        for child in self.children.iter_mut() {
            child.final_rect = child
                .component
                .arrange(Rect::new(Point::new(final_rect.pos.x, y), child.final_size));
            y += child.final_size.height;
        }
        final_rect
    }

    fn arrange_center(&mut self, final_rect: Rect) -> Rect {
        let mut y = final_rect.pos.y;
        for child in self.children.iter_mut() {
            child.final_rect = child.component.arrange(Rect::new(
                Point::new(
                    final_rect.pos.x + (final_rect.size.width - child.final_size.width) / 2.0,
                    y,
                ),
                child.final_size,
            ));
            y += child.final_size.height;
        }
        final_rect
    }

    fn arrange_right(&mut self, final_rect: Rect) -> Rect {
        let mut y = final_rect.pos.y;
        for child in self.children.iter_mut() {
            child.final_rect = child.component.arrange(Rect::new(
                Point::new(
                    final_rect.pos.x + final_rect.size.width - child.final_size.width,
                    y,
                ),
                child.final_size,
            ));
            y += child.final_size.height;
        }
        final_rect
    }
}

impl Component for Column {
    fn is_layout_dirty(&self) -> bool {
        self.children.iter().any(|c| c.component.is_layout_dirty())
    }

    fn is_visually_dirty(&self) -> bool {
        self.children
            .iter()
            .any(|c| c.component.is_visually_dirty())
    }

    fn measure(&mut self, available_size: Size) -> Size {
        let mut remaining_size = self.sizing.calc_available_size(available_size);
        let mut max_width = 0.0f32;

        for child in self.children.iter_mut() {
            child.final_size = child.component.measure(remaining_size);
            remaining_size.height -= child.final_size.height;
            max_width = max_width.max(child.final_size.width);
        }

        let required_size = Size::new(max_width, available_size.height - remaining_size.height);
        self.sizing.calc_final_size(available_size, required_size)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        match self.horiz_align {
            HorizontalAlignment::Left => self.arrange_left(final_rect),
            HorizontalAlignment::Center => self.arrange_center(final_rect),
            HorizontalAlignment::Right => self.arrange_right(final_rect),
        }
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        for child in self.children.iter() {
            child.component.draw(context);
        }
    }
}
