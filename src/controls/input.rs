use crate::component::*;
use crate::drawing::*;
use crate::*;

pub struct InputBuilder {
    hint: Option<String>,
    text: Option<BindableString>,
}

impl InputBuilder {
    pub fn with_hint(mut self, text: &str) -> Self {
        self.hint.replace(text.into());
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text.replace(BindableString::Static(text.into()));
        self
    }

    pub fn with_binding(mut self, binding: StringPropertyBinding) -> Self {
        self.text.replace(BindableString::Binding(binding));
        self
    }

    pub fn build(self) -> Box<Input> {
        Box::new(Input {
            hint: self.hint,
            text: self.text.unwrap_or_default(),
        })
    }
}

pub struct Input {
    hint: Option<String>,
    text: BindableString,
}

impl Input {
    pub fn builder() -> InputBuilder {
        InputBuilder {
            hint: None,
            text: None,
        }
    }
}

impl Component for Input {
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
