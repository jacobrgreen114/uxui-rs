use crate::drawing::*;
use crate::*;
use input_handling::{InputHandler, KeyEvent, MouseButtonEvent, PreviewInputHandler};

use crate::component::*;

#[derive(Default)]
pub struct ButtonBuilder {
    content: Option<Component>,
    builder: ComponentBuilder,
    background: Option<Color>,
}

impl ButtonBuilder {
    pub fn with_content(mut self, content: Component) -> Self {
        debug_assert!(self.content.is_none(), "Content already set");
        self.content = Some(content);
        self
    }

    pub fn with_action(mut self, action: Box<dyn Fn() + 'static>) -> Self {
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        debug_assert!(self.background.is_none(), "Background already set");
        self.background = Some(color);
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

    pub fn build(self) -> Component {
        let background_color = self.background.unwrap_or_default();
        self.builder.build(Button {
            content: self.content,
            background_color,
            background: Rectangle::new(Rect::default(), background_color),
        })
    }
}

pub struct Button {
    content: Option<Component>,
    background_color: Color,
    background: Rectangle,
}

impl Button {
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

impl InputHandler for Button {
    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        true
    }
}

impl PreviewInputHandler for Button {}

impl ComponentController for Button {
    fn measure(&mut self, available_size: Size) -> Size {
        match &mut self.content {
            Some(content) => content.measure(available_size),
            None => Size::zero(),
        }
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        let _content_rect = match &mut self.content {
            Some(content) => content.arrange(final_rect),
            None => Rect::default(),
        };
        self.background.update(final_rect, self.background_color);
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(&self.background);
        if let Some(content) = &self.content {
            content.draw(context);
        }
    }
}
