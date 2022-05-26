//! Handles the logic of merging various styles together based on an
//!  array of classnames

use std::collections::HashMap;

use serde::Deserialize;

mod border_radius;
mod border_style;
mod edge_style;
mod flex_style;
mod flex_values;
mod font_styles;
mod style;

pub use border_radius::BorderRadiusStyle;
pub use border_style::BorderStyle;
pub use edge_style::EdgeStyle;
pub use flex_style::FlexStyle;
pub use flex_values::*;
pub use font_styles::FontStyles;
pub use style::Style;

use crate::error::{DocumentGenerationError, UserInputError};

#[derive(Deserialize, Debug, Default)]
pub struct Stylesheet {
    #[serde(flatten)]
    style_lookup: HashMap<String, Style::Mergeable>,
}

impl Stylesheet {
    pub fn get_style(
        &self,
        base_style: Style::Unmergeable,
        class_names: &[String],
    ) -> Result<Style::Unmergeable, DocumentGenerationError> {
        class_names
            .iter()
            .map(|class_name| (class_name, self.style_lookup.get(class_name)))
            .try_fold(base_style, |acc, (class_name, style)| {
                Ok(
                    acc.merge_style(style.ok_or_else(|| UserInputError::StyleDoesNotExist {
                        style_name: class_name.to_owned(),
                    })?),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::values::Color;

    use super::*;

    #[test]
    fn style_lookup() {
        let stylesheet = Stylesheet {
            style_lookup: [
                (
                    "a".to_owned(),
                    Style::Mergeable {
                        color: Some(Color::white()),
                        width: Some("a".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "b".to_owned(),
                    Style::Mergeable {
                        height: Some("b".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "c".to_owned(),
                    Style::Mergeable {
                        width: Some("c".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "d".to_owned(),
                    Style::Mergeable {
                        height: Some("d".to_owned()),
                        width: Some("d".to_owned()),
                        color: Some(Color::white()),
                        ..Default::default()
                    },
                ),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        assert_eq!(
            stylesheet.get_style(Default::default(), &[]).unwrap(),
            Default::default()
        );
        assert_eq!(
            stylesheet
                .get_style(Default::default(), &["a".to_owned()])
                .unwrap(),
            Style::Unmergeable {
                color: Color::white(),
                width: String::from("a"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(Default::default(), &["a".to_owned(), "b".to_owned()])
                .unwrap(),
            Style::Unmergeable {
                color: Color::white(),
                height: String::from("b"),
                width: String::from("a"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(
                    Default::default(),
                    &["a".to_owned(), "b".to_owned(), "c".to_owned()]
                )
                .unwrap(),
            Style::Unmergeable {
                color: Color::white(),
                height: String::from("b"),
                width: String::from("c"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(
                    Default::default(),
                    &[
                        "a".to_owned(),
                        "b".to_owned(),
                        "c".to_owned(),
                        "d".to_owned()
                    ]
                )
                .unwrap(),
            Style::Unmergeable {
                color: Color::white(),
                height: String::from("d"),
                width: String::from("d"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(Default::default(), &["b".to_owned()])
                .unwrap(),
            Style::Unmergeable {
                height: String::from("b"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(Default::default(), &["b".to_owned(), "c".to_owned()])
                .unwrap(),
            Style::Unmergeable {
                height: String::from("b"),
                width: String::from("c"),
                ..Default::default()
            }
        );

        assert_eq!(
            stylesheet
                .get_style(
                    Default::default(),
                    &["b".to_owned(), "c".to_owned(), "d".to_owned()]
                )
                .unwrap(),
            Style::Unmergeable {
                color: Color::white(),
                height: String::from("d"),
                width: String::from("d"),
                ..Default::default()
            }
        );
    }
}
