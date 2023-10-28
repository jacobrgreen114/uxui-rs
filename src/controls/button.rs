use crate::drawing::*;
use crate::*;
use input_handling::*;
use std::cell::*;

use crate::component::*;

pub struct ButtonBuilder {
    content: Option<Box<dyn Component>>,
    sizing: Sizing,
    background: Color,
}

impl ButtonBuilder {
    pub fn with_content(mut self, content: impl Component + 'static) -> Self {
        self.content = Some(Box::new(content));
        self
    }

    pub fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
}

impl Builder<Button> for ButtonBuilder {
    fn build(self) -> Button {
        Button {
            content: self.content.map(|c| ButtonContent {
                content: c,
                final_size: Cell::new(Size::zero()),
            }),
            sizing: self.sizing,
            background_color: self.background,
            background: VisualRectangle::new(Rect::default(), self.background),
            final_rect: Cell::new(Rect::default()),
            cursor_in_bounds: Cell::new(false),
        }
    }
}

struct ButtonContent {
    content: Box<dyn Component>,
    final_size: Cell<Size>,
}

pub struct Button {
    content: Option<ButtonContent>,
    sizing: Sizing,
    background_color: Color,
    background: VisualRectangle,
    final_rect: Cell<Rect>,
    cursor_in_bounds: Cell<bool>,
}

impl Button {
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder {
            content: None,
            sizing: Sizing::fixed(Size::new(16.0, 16.0)),
            background: Color::new_grayscale(0.9),
        }
    }
}

impl Layout for Button {
    fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        let required = match &mut self.content {
            Some(content) => {
                let content_size = content.content.measure(available);
                content.final_size.set(content_size);
                content_size
            }
            None => Size::zero(),
        };
        self.sizing.calc_final_size(available, required)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        let _content_rect = match &mut self.content {
            Some(content) => {
                let child_rect = final_rect.align_center(content.final_size.get());
                content.content.arrange(child_rect)
            }
            None => Rect::default(),
        };

        self.background.update(final_rect, self.background_color);
        self.final_rect.set(final_rect);
        final_rect
    }
}

impl Draw for Button {
    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(&self.background);
    }
}

impl PreviewInputHandler for Button {}

impl InputHandler for Button {
    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
}

impl DispatchInput for Button {
    fn dispatch_key(&mut self, event: &KeyEvent) -> bool {
        self.on_key_preview(event)
            || self
                .content
                .as_mut()
                .map_or(false, |c| c.content.dispatch_key(event))
            || self.on_key(event)
    }

    fn dispatch_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        self.on_mouse_button_preview(event)
            || self
                .content
                .as_mut()
                .map_or(false, |c| c.content.dispatch_mouse_button(event))
            || self.on_mouse_button(event)
    }

    fn dispatch_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        self.on_mouse_wheel_preview(event)
            || self
                .content
                .as_mut()
                .map_or(false, |c| c.content.dispatch_mouse_wheel(event))
            || self.on_mouse_wheel(event)
    }

    fn dispatch_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        if self.final_rect.get().contains(event.pos()) {
            self.cursor_in_bounds.set(true);
            self.on_cursor_moved_preview(event)
                || self
                    .content
                    .as_mut()
                    .map_or(false, |c| c.content.dispatch_cursor_moved(event))
                || self.on_cursor_moved(event)
        } else {
            self.cursor_in_bounds.set(false);
            false
        }
    }
}

impl Component for Button {}
