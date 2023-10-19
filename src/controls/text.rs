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

pub struct Text {
    text: BindableString,
    font_info: FontInfo,
    cached_bb: Size,
    formatted_text: Option<FormattedText>,
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
            cached_bb: Size::default(),
            formatted_text: None,
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

        self.cached_bb = TextFormatter::calculate_bounding_box(text, font, self.font_info.size);
        self.cached_bb
    }

    fn arrange(&mut self, final_rect: Rect, children: &[Component]) -> Rect {
        let text = match &self.text {
            BindableString::Static(text) => text.as_str(),
            // BindableString::Binding(binding) => todo!(),
        };

        let font = find_best_font(&BestFontQuery {
            query: FontQuery::FamilyName(self.font_info.family.as_ref()),
            style: Default::default(),
        })
        .unwrap();

        self.formatted_text = Some(FormattedText::new(
            text,
            final_rect,
            font,
            self.font_info.size,
        ));
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(self.formatted_text.as_ref().unwrap());
    }
}

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
