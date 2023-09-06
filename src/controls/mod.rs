use crate::*;

struct TextController {}

impl ComponentController for TextController {}

pub struct Text {
    component: Component<TextController>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            component: Component::new(TextController {}),
        }
    }
}

impl ComponentInterface for Text {
    fn measure(&self, available_size: Size) {
        self.component.measure(available_size)
    }

    fn arrange(&self, final_rect: Rect) {
        self.component.arrange(final_rect)
    }
}
