use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::*;
use std::cell::Cell;

use crate::font::*;

#[cfg(windows)]
const DEFAULT_FONT_FAMILY: &str = "Segoe UI";

#[derive(Debug, Clone)]
pub struct FontInfo {
    pub family: Box<str>,
    pub weight: FontWeight,
    pub width: FontWidth,
    pub size: FontSize,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            family: DEFAULT_FONT_FAMILY.into(),
            size: FontSize::em(1.0),
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
        }
    }
}

pub struct TextBuilder {
    text: BindableString,
    font_info: FontInfo,
}

impl Builder<Text> for TextBuilder {
    fn build(self) -> Text {
        Text {
            text: self.text,
            font_info: self.font_info,
            formatted_text: None,
        }
    }
}

pub struct Text {
    text: BindableString,
    font_info: FontInfo,
    formatted_text: Option<VisualText>,
}

impl Text {
    pub fn builder(text: &str) -> TextBuilder {
        TextBuilder {
            text: BindableString::Static(text.into()),
            font_info: Default::default(),
        }
    }
}

impl Layout for Text {
    fn measure(&mut self, available_size: Size) -> Size {
        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(self.font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        let text = match &self.text {
            BindableString::Static(text) => text.as_str(),
            // BindableString::Binding(binding) => todo!(),
        };

        TextFormatter::calculate_bounding_box(text, font, self.font_info.size)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        let text = match &self.text {
            BindableString::Static(text) => text.as_str(),
            // BindableString::Binding(binding) => todo!(),
        };

        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(self.font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        self.formatted_text = Some(VisualText::new(text, final_rect, font, self.font_info.size));
        final_rect
    }
}

impl Draw for Text {
    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(self.formatted_text.as_ref().unwrap());
    }
}

impl InputHandler for Text {}

impl PreviewInputHandler for Text {}

impl DispatchInput for Text {}

impl Component for Text {}

struct TextFormatter {}

impl TextFormatter {
    fn calculate_bounding_box(text: &str, font: &Font, size: FontSize) -> Size {
        let font_scale = calculate_font_scale(size);

        let line_height = font.line_height() * font_scale;

        let mut height: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for line in text.lines() {
            height += line_height;

            for c in line.chars() {
                match c {
                    codepoint => {
                        let glyph = font.get_glyph(codepoint).unwrap();
                        width += glyph.advance() * font_scale;
                    }
                }
            }

            max_width = max_width.max(width);
            width = 0.0;
        }

        max_width = max_width.max(width);
        Size::new(max_width, height)
    }
}
