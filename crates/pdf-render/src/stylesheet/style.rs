use merges::Merges;
use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::values::Color;

use super::{BorderStyle, EdgeStyle, FlexStyle, FontStyles};

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct Style {
    #[mergeable(nested)]
    pub border: BorderStyle,
    #[mergeable(nested)]
    pub font: FontStyles,
    #[ts(type = "string")]
    pub color: Color,
    #[mergeable(nested)]
    pub margin: EdgeStyle,
    #[mergeable(nested)]
    pub padding: EdgeStyle,
    #[ts(type = "string")]
    pub background_color: Color,
    #[mergeable(nested)]
    pub flex: FlexStyle,
    pub width: String,
    pub height: String,
}

impl Default for Style::Unmergeable {
    fn default() -> Self {
        Self {
            color: Color::black(),
            background_color: Color::white(),
            width: String::from("auto"),
            height: String::from("auto"),
            border: Default::default(),
            font: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            flex: Default::default(),
        }
    }
}

impl Style::Unmergeable {
    pub fn merge_style(&self, rhs: &Style::Mergeable) -> Style::Unmergeable {
        let base = Style::Mergeable::from(self.clone());

        let merged: Style::Mergeable = base.merge(rhs);

        merged.into()
    }
}
