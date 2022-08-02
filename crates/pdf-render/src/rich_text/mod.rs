//! A representation of the style (size, color, etc.) of text
//!  throughout a paragraph.

use std::fmt::Display;

use crate::{
    error::DocumentGenerationError,
    fonts::FontAttributes,
    stylesheet::{Style, TextTransformation},
    values::{Color, Pt},
};

pub mod dom_node_conversion;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RichTextSpan {
    pub text: String,
    pub attributes: FontAttributes,
    // TODO: Probably not efficient to store a full string every time
    pub font_family: String,
    pub size: Pt,
    pub color: Color,
    pub letter_spacing: Pt,
    pub line_height: f64,
}

impl RichTextSpan {
    pub fn new(raw_str: &str, style: Style) -> Self {
        let line_height = if let Some(line_height) = style.line_height {
            line_height.0 / style.font.size.0
        } else {
            1.0
        };

        Self {
            text: if style.text_transform == TextTransformation::Uppercase {
                raw_str.to_uppercase()
            } else {
                raw_str.to_owned()
            },
            attributes: FontAttributes {
                weight: style.font.weight,
                style: style.font.style,
            },
            color: style.color,
            font_family: style.font.family,
            size: style.font.size,
            letter_spacing: style.font.letter_spacing,
            line_height,
        }
    }
}

impl From<&str> for RichTextSpan {
    fn from(raw_str: &str) -> Self {
        RichTextSpan {
            text: raw_str.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RichText(pub Vec<RichTextSpan>);

impl Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::from(""), |acc, span_str| acc + &span_str.text)
        )
    }
}

trait CharToByteIndex {
    fn get_byte_index_from_char(&self, char_index: usize) -> usize;
}

impl CharToByteIndex for String {
    fn get_byte_index_from_char(&self, char_index: usize) -> usize {
        debug_assert!(char_index <= self.chars().count());

        self.char_indices()
            .nth(char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.len())
    }
}

impl RichText {
    pub fn substr(
        &self,
        char_start_index: usize,
        char_end_index: usize,
    ) -> Result<RichText, DocumentGenerationError> {

        if char_start_index == char_end_index {
            return Ok(RichText(vec![]));
        }

        let span_data: Vec<(&RichTextSpan, usize, usize)> = self
            .0
            .iter()
            .scan(0, |current_line_index, span| {
                let line_start_index = *current_line_index;
                *current_line_index += span.text.chars().count();
                let line_end_index = *current_line_index;

                Some((span, line_start_index, line_end_index))
            })
            .collect();

        let start_span_index = span_data
            .iter()
            .position(|&(_, _, line_end_index)| line_end_index > char_start_index)
            .unwrap();

        let end_span_index = span_data
            .iter()
            .rposition(|&(_, line_start_index, _)| line_start_index < char_end_index)
            .unwrap();

        let rich = if start_span_index == end_span_index {
            // Same span!

            let (span, start, _) = span_data[start_span_index];

            let byte_start_index = span.text.get_byte_index_from_char(char_start_index - start);
            let byte_end_index = span.text.get_byte_index_from_char(char_end_index - start);

            RichText(vec![RichTextSpan {
                text: span.text[byte_start_index..byte_end_index].to_owned(),
                ..span.clone()
            }])
        } else {
            let mut rich_text = RichText(vec![]);

            let (start_span, start, _) = span_data[start_span_index];

            let byte_start_index = start_span
                .text
                .get_byte_index_from_char(char_start_index - start);

            rich_text.0.push(RichTextSpan {
                text: start_span.text[byte_start_index..].to_owned(),
                ..start_span.clone()
            });

            rich_text
                .0
                .extend(self.0[start_span_index + 1..end_span_index].iter().cloned());

            let (end_span, start, _) = span_data[end_span_index];

            let byte_end_index = end_span
                .text
                .get_byte_index_from_char(char_end_index - start);

            rich_text.0.push(RichTextSpan {
                text: end_span.text[0..byte_end_index].to_owned(),
                ..end_span.clone()
            });

            rich_text
        };

        Ok(rich)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_selection() {
        let line = RichText(vec![
            RichTextSpan {
                // 89 characters
                // 93 bytes
                size: Pt(32.),
                .."Your approach to work is one of the most visible parts of your professional “appearance”.".into()
            },
            RichTextSpan {
                // 23 characters
                // 51 bytes
                size: Pt(15.),
                .."🎅🎅🎅🎅🎅🎅🎅🎅 “appearance”.".into()
            },
            RichTextSpan {
                // 9 characters
                size: Pt(8.),
                .." lazy dog".into()
            },
        ]);

        assert_eq!(line.substr(0, 0).unwrap(), RichText(vec![]));
    }

    #[test]
    fn unicode_support() {
        let line = RichText(vec![
            RichTextSpan {
                // 89 characters
                // 93 bytes
                size: Pt(32.),
                .."Your approach to work is one of the most visible parts of your professional “appearance”.".into()
            },
            RichTextSpan {
                // 23 characters
                // 51 bytes
                size: Pt(15.),
                .."🎅🎅🎅🎅🎅🎅🎅🎅 “appearance”.".into()
            },
            RichTextSpan {
                // 9 characters
                size: Pt(8.),
                .." lazy dog".into()
            },
        ]);

        assert_eq!(line.substr(0, 89).unwrap().0, line.0[0..1]);
        // 4 santas
        assert_eq!(
            line.substr(0, 93).unwrap(),
            RichText(vec![
                line.0[0].clone(),
                RichTextSpan {
                    size: Pt(15.),
                    .."🎅🎅🎅🎅".into()
                }
            ])
        );

        assert_eq!(
            line.substr(76, 88).unwrap(),
            RichText(vec![
                // line.0[0].clone(),
                RichTextSpan {
                    size: Pt(32.),
                    .."“appearance”".into()
                }
            ])
        );

        assert_eq!(
            line.substr(76, 87).unwrap(),
            RichText(vec![
                // line.0[0].clone(),
                RichTextSpan {
                    size: Pt(32.),
                    .."“appearance".into()
                }
            ])
        );

        assert_eq!(
            line.substr(76, 89).unwrap(),
            RichText(vec![
                // line.0[0].clone(),
                RichTextSpan {
                    size: Pt(32.),
                    .."“appearance”.".into()
                }
            ])
        );
    }

    #[test]
    #[should_panic]
    fn bad_substr() {
        let line = RichText(vec![
            RichTextSpan {
                // 15 characters
                size: Pt(32.),
                .."The quick brown".into()
            },
            RichTextSpan {
                // 19 characters
                size: Pt(15.),
                .." fox jumps over the".into()
            },
            RichTextSpan {
                // 9 characters
                size: Pt(8.),
                .." lazy dog".into()
            },
        ]);

        line.substr(0, 44).unwrap();
    }

    #[test]
    fn substr_works() {
        let line = RichText(vec![
            RichTextSpan {
                // 15 characters
                size: Pt(32.),
                .."The quick brown".into()
            },
            RichTextSpan {
                // 19 characters
                size: Pt(15.),
                .." fox jumps over the".into()
            },
            RichTextSpan {
                // 9 characters
                size: Pt(8.),
                .." lazy dog".into()
            },
        ]);

        // Total: 43

        assert_eq!(line.substr(0, 15).unwrap().0, line.0[0..1]);
        assert_eq!(
            line.substr(0, 10).unwrap(),
            RichText(vec![RichTextSpan {
                size: Pt(32.),
                .."The quick ".into()
            }])
        );
        assert_eq!(
            line.substr(16, 20).unwrap(),
            RichText(vec![RichTextSpan {
                size: Pt(15.),
                .."fox ".into()
            }])
        );
        assert_eq!(
            line.substr(15, 20).unwrap(),
            RichText(vec![RichTextSpan {
                size: Pt(15.),
                .." fox ".into()
            }])
        );
        assert_eq!(
            line.substr(15, 34).unwrap(),
            RichText(vec![RichTextSpan {
                size: Pt(15.),
                .." fox jumps over the".into()
            }])
        );

        assert_eq!(line.substr(0, 43).unwrap(), line);

        assert_eq!(line.substr(10, 43).unwrap().0[1..], line.0[1..]);
        assert_eq!(
            line.substr(10, 43).unwrap().0[0],
            RichTextSpan {
                size: Pt(32.),
                .."brown".into()
            }
        );

        assert_eq!(line.substr(15, 43).unwrap().0, line.0[1..]);
        assert_eq!(line.substr(34, 43).unwrap().0, line.0[2..]);

        assert_eq!(line.substr(0, 15).unwrap().0, line.0[0..1]);
        assert_eq!(line.substr(0, 18).unwrap().0[0], line.0[0]);
        assert_eq!(
            line.substr(0, 18).unwrap().0[1],
            RichTextSpan {
                size: Pt(15.),
                .." fo".into()
            }
        );
    }
}
