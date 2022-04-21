use std::ops::Range;

use printpdf::Pt;

use num_derive::FromPrimitive;
use serde::Deserialize;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

impl From<&str> for FontStyle {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "italic" => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, FromPrimitive, Deserialize)]
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

impl From<&str> for FontWeight {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "lighter" => FontWeight::Light,
            "bold" => FontWeight::Bold,
            "bolder" => FontWeight::ExtraBold,
            "normal" => FontWeight::Regular,
            other => {
                if let Ok(num) = other.parse::<u32>() {
                    num::FromPrimitive::from_u32(num).unwrap_or_default()
                } else {
                    FontWeight::Regular
                }
            }
        }
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Default, Clone)]
pub struct RichTextStyleChanges {
    pub font_family: Option<String>,
    pub font_size: Option<Pt>,
    pub weight: Option<FontWeight>,
    pub style: Option<FontStyle>,
    pub color: Option<(f32, f32, f32)>,
}

impl RichTextStyleChanges {
    #[allow(dead_code)]
    pub fn font_size(size: Pt) -> Self {
        Self {
            font_size: Some(size),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn color(color: (f32, f32, f32)) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RichTextStyle {
    pub font_family: String,
    pub font_size: Pt,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub color: (f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct RichText<'a> {
    prev_start: usize,
    pub(crate) paragraph: &'a str,
    pub(crate) default_style: RichTextStyle,
    pub(crate) style_ranges: Vec<(Range<usize>, RichTextStyleChanges)>,
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

    pub fn push_style(&mut self, style: RichTextStyleChanges, range: Range<usize>) -> &mut Self {
        assert!(
            range.start >= self.prev_start,
            "Expected styles to be presented in monotonically increasing order"
        );
        // TODO: We should also assert nested ranges are subsets

        self.prev_start = range.start;
        self.style_ranges.push((range, style));

        self
    }

    pub fn style_range_iter(&'a self) -> StyleIterator<'a> {
        StyleIterator::new(self)
    }
}

pub struct StyleIterator<'a> {
    rich_text: &'a RichText<'a>,
    style_stack: Vec<RichTextStyle>,
    range_stack: Vec<Range<usize>>,
    current_position: usize,
    next_range_index: usize,
}

impl<'a> StyleIterator<'a> {
    fn new(rich_text: &'a RichText<'a>) -> Self {
        Self {
            rich_text,
            style_stack: vec![rich_text.default_style.clone()],
            range_stack: vec![0..rich_text.paragraph.len()],
            current_position: 0,
            next_range_index: 0,
        }
    }
}

impl<'a> Iterator for StyleIterator<'a> {
    type Item = (Range<usize>, RichTextStyle);

    fn next(&mut self) -> Option<Self::Item> {
        if self.range_stack.is_empty() {
            return None;
        }

        let current_range = self
            .range_stack
            .last()
            .expect("We just checked if the stack was empty")
            .clone();

        let current_style = self.style_stack.last().expect("Same...").clone();

        let end_position =
            if let Some((range, _)) = self.rich_text.style_ranges.get(self.next_range_index) {
                if range.start < current_range.end {
                    range.start
                } else {
                    self.style_stack.pop();
                    self.range_stack.pop();
                    // Finish up the current range
                    current_range.end
                }
            } else {
                self.style_stack.pop();
                self.range_stack.pop();
                current_range.end
            };

        debug_assert!(self.current_position < end_position);

        let r_value: Self::Item = (self.current_position..end_position, current_style.clone());

        self.current_position = end_position;

        if let Some((range, style)) = self.rich_text.style_ranges.get(self.next_range_index) {
            self.range_stack.push(range.clone());

            let mut next_style = current_style;
            // We could probably create a macros or something to merge this so
            // we don't have to keep this up to date
            if let Some(font_size) = style.font_size {
                next_style.font_size = font_size;
            }

            if let Some(weight) = style.weight {
                next_style.weight = weight;
            }

            if let Some(font_style) = style.style {
                next_style.style = font_style;
            }

            if let Some(color) = style.color {
                next_style.color = color;
            }

            // Create new style based on prev_style
            self.style_stack.push(next_style);
            self.next_range_index += 1;
        }

        Some(r_value)
    }
}
