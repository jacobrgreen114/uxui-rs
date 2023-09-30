use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::{InputHandler, PreviewInputHandler};

use crate::font::*;

#[derive(Debug, Clone)]
pub struct FontInfo {
    pub family: Box<str>,
    pub weight: FontWeight,
    pub width: FontWidth,
    pub size: f32,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            family: "Segoe UI".into(),
            size: 12.0,
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
        }
    }
}

pub struct Text {
    text: BindableString,
    font_info: FontInfo,
}

impl Text {
    pub fn new(text: &str) -> Component {
        ComponentBuilder::default().build(Text {
            text: BindableString::Static(text.into()),
            font_info: Default::default(),
        })
    }
}

impl InputHandler for Text {}

impl PreviewInputHandler for Text {}

impl ComponentController for Text {
    fn measure(&mut self, available_size: Size) -> Size {
        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(self.font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        let text = match &self.text {
            BindableString::Static(text) => text.as_str(),
            BindableString::Binding(binding) => todo!(),
        };

        let line_height = font.line_height();

        let mut height: f32 = line_height;
        let mut width: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for character in text.chars() {
            if character == '\n' {
                height += line_height;
                max_width = max_width.max(width);
                width = 0.0;
                continue;
            }

            let glyph = font.get_glyph(character).unwrap();
            width += glyph.advance();
        }

        max_width = max_width.max(width);
        Size::new(max_width, height)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        final_rect
    }

    fn draw(&self, context: &mut DrawingContext) {}
}
