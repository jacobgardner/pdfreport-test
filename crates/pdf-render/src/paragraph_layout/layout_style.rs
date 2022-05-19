use skia_safe::textlayout as skia_layout;

pub enum TextAlign {
    Left,
    Right,
    Center,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

#[derive(Default)]
pub struct ParagraphStyle {
    pub align: TextAlign,
}

impl From<TextAlign> for skia_layout::TextAlign {
    fn from(align: TextAlign) -> Self {
        match align {
            TextAlign::Left => skia_layout::TextAlign::Left,
            TextAlign::Center => skia_layout::TextAlign::Center,
            TextAlign::Right => skia_layout::TextAlign::Right,
        }
    }
}
