use std::ops::Range;

use printpdf::Pt;

#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Default, Clone)]
pub struct RichTextStyle {
    pub font_size: Option<Pt>,
    pub weight: Option<FontWeight>,
    pub italic: Option<bool>,
    pub color: Option<(f32, f32, f32)>,
}

impl RichTextStyle {
    pub fn font_size(size: Pt) -> Self {
        Self {
            font_size: Some(size),
            ..Default::default()
        }
    }

    pub fn color(color: (f32, f32, f32)) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct RichText<'a> {
    prev_start: usize,
    pub(crate) paragraph: &'a str,
    pub(crate) default_style: RichTextStyle,
    pub(crate) style_ranges: Vec<(Range<usize>, RichTextStyle)>,
}

impl<'a> RichText<'a> {
    pub fn new(paragraph: &'a str, default_style: RichTextStyle) -> Self {
        Self {
            prev_start: 0,
            paragraph,
            default_style,
            style_ranges: Vec::new(),
        }
    }

    pub fn push_style(&mut self, style: RichTextStyle, range: Range<usize>) -> &mut Self {
        assert!(
            range.start >= self.prev_start,
            "Expected styles to be presented in monotonically increasing order"
        );
        // TODO: We should also assert nested ranges are subsets

        self.prev_start = range.start;
        self.style_ranges.push((range, style));

        self
    }
}
