use crate::fonts::FontId;

pub struct RichTextSpan {
    pub text: String,
    pub font_id: FontId,
    pub size: f64,
}

pub struct RichTextLine(pub Vec<RichTextSpan>);

impl RichTextLine {
    pub fn from_str(raw_string: &str, font_id: FontId, size: f64) -> Self {
        Self(vec![RichTextSpan {
            text: raw_string.to_owned(),
            font_id,
            size,
        }])
    }
}
