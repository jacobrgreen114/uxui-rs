use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::*;
use std::cell::Cell;

#[derive(Debug)]
struct ColumnChild {
    component: Box<dyn Component>,
    final_size: Cell<Size>,
}

#[derive(Debug)]
pub struct Column {
    children: Vec<ColumnChild>,
    sizing: Sizing,
    horiz_align: HorizontalAlignment,
    final_rect: Cell<Rect>,
}

impl Column {
    pub fn builder() -> ColumnBuilder {
        ColumnBuilder::default()
    }
}

impl Layout for Column {
    fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        let required = {
            let mut remaining_size = available_size;
            let mut max_width = 0.0f32;

            for child in &mut self.children {
                let child_size = child.component.measure(remaining_size);
                child.final_size.set(child_size);
                remaining_size.height -= child_size.height;
                max_width = max_width.max(child_size.width);
            }

            Size::new(max_width, available_size.height - remaining_size.height)
        };

        self.sizing.calc_final_size(available, required)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        let final_rect = match self.horiz_align {
            HorizontalAlignment::Left => self.arrange_left(final_rect),
            HorizontalAlignment::Center => self.arrange_center(final_rect),
            HorizontalAlignment::Right => self.arrange_right(final_rect),
        };
        self.final_rect.set(final_rect);
        final_rect
    }
}

impl Column {
    #[inline(always)]
    fn arrange_generic(&mut self, final_rect: Rect, func: impl Fn(Rect, Size) -> Point) -> Rect {
        let mut y = final_rect.pos.y;
        for child in &mut self.children {
            let child_size = child.final_size.get();
            let child_rect = child.component.arrange(Rect::new(
                func(
                    Rect::new(Point::new(final_rect.pos.x, y), final_rect.size),
                    child_size,
                ),
                child_size,
            ));
            y += child_rect.size.height;
        }
        final_rect
    }

    #[inline(always)]
    fn arrange_left(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, _child_size| rect.pos)
    }

    #[inline(always)]
    fn arrange_center(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, child_size| {
            Point::new(
                rect.pos.x + (rect.size.width - child_size.width) / 2.0,
                rect.pos.y,
            )
        })
    }

    #[inline(always)]
    fn arrange_right(&mut self, final_rect: Rect) -> Rect {
        self.arrange_generic(final_rect, |rect, child_size| {
            Point::new(rect.pos.x + rect.size.width - child_size.width, rect.pos.y)
        })
    }
}

impl Draw for Column {
    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        for child in &self.children {
            child.component.draw(context);
        }
    }
}

impl InputHandler for Column {}

impl PreviewInputHandler for Column {}

impl DispatchInput for Column {
    fn dispatch_key(&mut self, event: &KeyEvent) -> bool {
        self.on_key_preview(event)
            || self
                .children
                .iter_mut()
                .any(|c| c.component.dispatch_key(event))
            || self.on_key(event)
    }

    fn dispatch_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.on_mouse_button_preview(event)
            || self
                .children
                .iter_mut()
                .any(|c| c.component.dispatch_mouse_button(event))
            || self.on_mouse_button(event)
    }

    fn dispatch_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.on_mouse_wheel_preview(event)
            || self
                .children
                .iter_mut()
                .any(|c| c.component.dispatch_mouse_wheel(event))
            || self.on_mouse_wheel(event)
    }

    fn dispatch_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        if self.final_rect.get().contains(event.pos()) {
            self.on_cursor_moved_preview(event)
                || self
                    .children
                    .iter_mut()
                    .any(|c| c.component.dispatch_cursor_moved(event))
                || self.on_cursor_moved(event)
        } else {
            false
        }
    }
}

impl Component for Column {}

//
// Builder
//

#[derive(Default)]
pub struct ColumnBuilder {
    children: Vec<Box<dyn Component>>,
    sizing: Sizing,
    horiz_align: HorizontalAlignment,
}

impl ColumnBuilder {
    pub fn with_children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children = children;
        self
    }

    pub const fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }

    pub const fn with_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horiz_align = alignment;
        self
    }
}

impl Builder<Column> for ColumnBuilder {
    fn build(self) -> Column {
        Column {
            children: self
                .children
                .into_iter()
                .map(|c| ColumnChild {
                    component: c,
                    final_size: Cell::new(Size::zero()),
                })
                .collect(),
            sizing: self.sizing,
            horiz_align: self.horiz_align,
            final_rect: Cell::new(Rect::default()),
        }
    }
}
