use crate::font::Font;

pub struct FormattedText<'a> {
    text: String,
    font: &'a Font,
}
