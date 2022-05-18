use crate::{error::DocumentGenerationError, fonts::FontId, geometry::Pt};

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextSpan {
    pub text: String,
    pub font_id: FontId,
    pub size: Pt,
}

#[derive(Debug, PartialEq)]
pub struct RichText(pub Vec<RichTextSpan>);

impl RichText {
    pub fn from_str(raw_string: &str, font_id: FontId, size: impl Into<Pt>) -> Self {
        Self(vec![RichTextSpan {
            text: raw_string.to_owned(),
            font_id,
            size: size.into(),
        }])
    }

    pub fn substr(
        &self,
        line_start_index: usize,
        line_end_index: usize,
    ) -> Result<RichText, DocumentGenerationError> {
        let mut current_span_offset = 0;

        let span_data: Vec<(&RichTextSpan, usize, usize)> = self
            .0
            .iter()
            .scan(0, |current_line_index, span| {
                let line_start_index = *current_line_index;
                *current_line_index += span.text.len();
                let line_end_index = *current_line_index;

                Some((span, line_start_index, line_end_index))
            })
            .collect();

        let start_span_index = span_data
            .iter()
            .position(|&(_, _, line_end_index)| line_end_index > line_start_index)
            .unwrap();

        let end_span_index = span_data
            .iter()
            .rposition(|&(_, line_start_index, _)| line_start_index < line_end_index)
            .unwrap();

        let rich = if start_span_index == end_span_index {
            // Same span!

            let (span, start, end) = span_data[start_span_index];

            RichText(vec![RichTextSpan {
                text: span.text[line_start_index - start..line_end_index - start].to_owned(),
                ..span.clone()
            }])
        } else {
            let mut rich_text = RichText(vec![]);

            let (start_span, start, end) = span_data[start_span_index];

            rich_text.0.push(RichTextSpan {
                text: start_span.text[line_start_index - start..].to_owned(),
                ..start_span.clone()
            });

            rich_text
                .0
                .extend(self.0[start_span_index+1..end_span_index].iter().cloned());

            let (end_span, start, end) = span_data[end_span_index];
            rich_text.0.push(RichTextSpan {
                text: end_span.text[0..line_end_index - start].to_owned(),
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
    fn substr_works() {
        let font_id = FontId::new();

        let line = RichText(vec![
            RichTextSpan {
                // 15 characters
                text: "The quick brown".to_owned(),
                font_id,
                size: Pt(32.),
            },
            RichTextSpan {
                // 19 characters
                text: " fox jumps over the".to_owned(),
                font_id,
                size: Pt(15.),
            },
            RichTextSpan {
                // 9 characters
                text: " lazy dog".to_owned(),
                font_id,
                size: Pt(8.),
            },
        ]);

        // Total: 43

        assert_eq!(line.substr(0, 15).unwrap().0, line.0[0..1]);
        assert_eq!(
            line.substr(0, 10).unwrap(),
            RichText(vec![RichTextSpan {
                text: "The quick ".to_owned(),
                font_id,
                size: Pt(32.)
            }])
        );
        assert_eq!(
            line.substr(16, 20).unwrap(),
            RichText(vec![RichTextSpan {
                text: "fox ".to_owned(),
                font_id,
                size: Pt(15.)
            }])
        );
        assert_eq!(
            line.substr(15, 20).unwrap(),
            RichText(vec![RichTextSpan {
                text: " fox ".to_owned(),
                font_id,
                size: Pt(15.)
            }])
        );
        assert_eq!(
            line.substr(15, 34).unwrap(),
            RichText(vec![RichTextSpan {
                text: " fox jumps over the".to_owned(),
                font_id,
                size: Pt(15.)
            }])
        );

        assert_eq!(line.substr(0, 43).unwrap(), line);

        assert_eq!(line.substr(10, 43).unwrap().0[1..], line.0[1..]);
        assert_eq!(
            line.substr(10, 43).unwrap().0[0],
            RichTextSpan {
                text: "brown".to_owned(),
                font_id,
                size: Pt(32.),
            }
        );

        assert_eq!(line.substr(15, 43).unwrap().0, line.0[1..]);
        assert_eq!(line.substr(34, 43).unwrap().0, line.0[2..]);

        assert_eq!(line.substr(0, 15).unwrap().0, line.0[0..1]);
        assert_eq!(line.substr(0, 18).unwrap().0[0], line.0[0]);
        assert_eq!(
            line.substr(0, 18).unwrap().0[1],
            RichTextSpan {
                text: " fo".to_owned(),
                font_id,
                size: Pt(15.),
            }
        );
    }
}
