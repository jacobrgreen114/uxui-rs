use crate::component::*;
use crate::drawing::*;
use crate::*;
use input_handling::{InputHandler, PreviewInputHandler};

use crate::font::*;

#[cfg(windows)]
const DEFAULT_FONT_FAMILY: &str = "Segoe UI";

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
            family: DEFAULT_FONT_FAMILY.into(),
            size: 12.0,
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
        }
    }
}

pub struct Text {
    text: BindableString,
    font_info: FontInfo,
    final_rect: Rect,
    formatted_text: FormattedText,
}

impl Text {
    pub fn new(text: &str) -> Component {
        let font_info = FontInfo::default();

        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        ComponentBuilder::default().build(Text {
            text: BindableString::Static(text.into()),
            font_info,
            final_rect: Rect::default(),
            formatted_text: FormattedText::new(text, Point::new(0.0, 0.0), font),
        })
    }
}

impl InputHandler for Text {}

impl PreviewInputHandler for Text {}

impl ComponentController for Text {
    fn measure(&mut self, available_size: Size, children: &[Component]) -> Size {
        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(self.font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        let text = match &self.text {
            BindableString::Static(text) => text.as_str(),
            // BindableString::Binding(binding) => todo!(),
        };

        TextFormatter::calculate_bounding_box(text, font)
    }

    fn arrange(&mut self, final_rect: Rect, children: &[Component]) -> Rect {
        self.final_rect = final_rect;
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(&self.formatted_text);
    }
}

struct TextFormatter {}

impl TextFormatter {
    fn calculate_bounding_box(text: &str, font: &Font) -> Size {
        let line_height = font.line_height();

        let mut height: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for line in text.lines() {
            height += line_height;

            for c in line.chars() {
                match c {
                    codepoint => {
                        let glyph = font.get_glyph(codepoint).unwrap();
                        width += glyph.advance();
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
