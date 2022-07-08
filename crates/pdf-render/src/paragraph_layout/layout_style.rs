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

impl TextAlign {
    pub fn from_anchor(anchor: &str) -> Self {
        match anchor.to_lowercase().as_ref() {
            "start" => Self::Left,
            "middle" => Self::Center,
            "end" => Self::Right,
            _ => Self::Left,
        }
    }
}

#[derive(Default)]
pub struct ParagraphStyle {
    pub align: TextAlign,
}

impl ParagraphStyle {
    pub fn left() -> Self {
        Self {
            align: TextAlign::Left,
        }
    }

    pub fn right() -> Self {
        Self {
            align: TextAlign::Right,
        }
    }

    pub fn center() -> Self {
        Self {
            align: TextAlign::Center,
        }
    }
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
