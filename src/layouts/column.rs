use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::{
    CursorMovedEvent, InputHandler, KeyEvent, MouseButtonEvent, MouseWheelEvent,
    PreviewInputHandler,
};

#[derive(Default)]
pub struct ColumnBuilder {
    builder: ComponentBuilder,
    children: Option<Vec<Component>>,
    horiz_align: Option<HorizontalAlignment>,
}

impl ColumnBuilder {
    pub fn with_children(mut self, children: Vec<Component>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn with_width(mut self, width: Length) -> Self {
        self.builder = self.builder.with_width(width);
        self
    }

    pub fn with_height(mut self, height: Length) -> Self {
        self.builder = self.builder.with_height(height);
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horiz_align = Some(alignment);
        self
    }

    pub fn build(self) -> Component {
        self.builder.build(Column {
            children: self.children.unwrap_or_default(),
            horiz_align: self.horiz_align.unwrap_or(HorizontalAlignment::Left),
        })
    }
}

// struct ColumnChild {
//     component: Box<dyn ComponentController>,
//     final_size: Size,
//     final_rect: Rect,
// }

pub struct Column {
    children: Vec<Component>,
    // sizing: Sizing,
    horiz_align: HorizontalAlignment,
}

impl Column {
    pub fn build() -> ColumnBuilder {
        Default::default()
    }

    fn arrange_generic(&mut self, final_rect: Rect, f: impl Fn(Rect, Size) -> Point) -> Rect {
        let mut y = final_rect.pos.y;
        for child in &mut self.children {
            let child_rect = child.arrange_from(f(
                Rect::new(Point::new(final_rect.pos.x, y), final_rect.size),
                child.final_size(),
            ));
            y += child_rect.size.height;
        }
        final_rect
    }

    fn arrange_left(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, _child_size| rect.pos)
    }

    fn arrange_center(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, child_size| {
            Point::new(
                rect.pos.x + (rect.size.width - child_size.width) / 2.0,
                rect.pos.y,
            )
        })
    }

    fn arrange_right(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, child_size| {
            Point::new(rect.pos.x + rect.size.width - child_size.width, rect.pos.y)
        })
    }
}

impl InputHandler for Column {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        for child in &mut self.children {
            if child.on_key(event) {
                return true;
            }
        }

        false
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        for child in &mut self.children {
            if child.on_mouse_button(event) {
                return true;
            }
        }

        false
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        for child in &mut self.children {
            if child.on_mouse_wheel(event) {
                return true;
            }
        }

        false
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        for child in &mut self.children {
            if child.on_cursor_moved(event) {
                return true;
            }
        }

        false
    }
}

impl PreviewInputHandler for Column {}

impl ComponentController for Column {
    fn measure(&mut self, available_size: Size) -> Size {
        let mut remaining_size = available_size;
        let mut max_width = 0.0f32;

        for child in &mut self.children {
            let child_size = child.measure(remaining_size);
            remaining_size.height -= child_size.height;
            max_width = max_width.max(child_size.width);
        }

        let required_size = Size::new(max_width, available_size.height - remaining_size.height);
        required_size
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        match self.horiz_align {
            HorizontalAlignment::Left => self.arrange_left(final_rect),
            HorizontalAlignment::Center => self.arrange_center(final_rect),
            HorizontalAlignment::Right => self.arrange_right(final_rect),
        }
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        for child in &self.children {
            child.draw(context);
        }
    }
}
