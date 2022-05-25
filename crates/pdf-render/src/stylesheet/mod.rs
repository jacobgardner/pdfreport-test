use std::collections::HashMap;

use serde::Deserialize;

mod style;

pub use style::{MergeableStyle, Style, Direction, FlexWrap};

use crate::error::{DocumentGenerationError, UserInputError};

#[derive(Deserialize, Debug, Default)]
pub struct Stylesheet {
    #[serde(flatten)]
    style_lookup: HashMap<String, MergeableStyle>,
    #[serde(skip)]
    default_style: Style,
}

impl Stylesheet {
    pub fn set_default_style(&mut self, value: Style) {
        self.default_style = value;
    }

    pub fn get_style(&self, class_names: &[&str]) -> Result<Style, DocumentGenerationError> {
        class_names
            .iter()
            .map(|&class_name| (class_name, self.style_lookup.get(class_name)))
            .try_fold(self.default_style.clone(), |acc, (class_name, style)| {
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
                    MergeableStyle {
                        color: Some(Color::white()),
                        width: Some("a".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "b".to_owned(),
                    MergeableStyle {
                        height: Some("b".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "c".to_owned(),
                    MergeableStyle {
                        width: Some("c".to_owned()),
                        ..Default::default()
                    },
                ),
                (
                    "d".to_owned(),
                    MergeableStyle {
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

        assert_eq!(stylesheet.get_style(&[]).unwrap(), Style::default());
        assert_eq!(
            stylesheet.get_style(&["a"]).unwrap(),
            Style {
                color: Color::white(),
                width: String::from("a"),
                ..Style::default()
            }
        );

        assert_eq!(
            stylesheet.get_style(&["a", "b"]).unwrap(),
            Style {
                color: Color::white(),
                height: String::from("b"),
                width: String::from("a"),
                ..Style::default()
            }
        );

        assert_eq!(
            stylesheet.get_style(&["a", "b", "c"]).unwrap(),
            Style {
                color: Color::white(),
                height: String::from("b"),
                width: String::from("c"),
                ..Style::default()
            }
        );

        assert_eq!(
            stylesheet.get_style(&["a", "b", "c", "d"]).unwrap(),
            Style {
                color: Color::white(),
                height: String::from("d"),
                width: String::from("d"),
                ..Style::default()
            }
        );
        
        
        assert_eq!(
            stylesheet.get_style(&["b"]).unwrap(),
            Style {
                height: String::from("b"),
                ..Style::default()
            }
        );

        assert_eq!(
            stylesheet.get_style(&["b", "c"]).unwrap(),
            Style {
                height: String::from("b"),
                width: String::from("c"),
                ..Style::default()
            }
        );

        assert_eq!(
            stylesheet.get_style(&["b", "c", "d"]).unwrap(),
            Style {
                color: Color::white(),
                height: String::from("d"),
                width: String::from("d"),
                ..Style::default()
            }
        );
    }
}
