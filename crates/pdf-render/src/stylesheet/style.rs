use merges::Merges;
use optional_merge_derive::mergeable;

use ts_rs::TS;

use crate::values::{Color, Pt};

use super::{BorderStyle, EdgeStyle, FlexStyle, FontStyles, PageBreakRule, TextTransformation};

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export, rename_all = "camelCase")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
    pub background_color: Option<Color>,
    #[mergeable(nested)]
    pub flex: FlexStyle,
    pub width: String,
    pub height: String,
    pub debug: bool,
    pub break_before: PageBreakRule,
    pub break_after: PageBreakRule,
    pub break_inside: PageBreakRule,
    pub text_transform: TextTransformation,
    #[ts(type = "number | string")]
    pub line_height: Option<Pt>,
}

impl Default for Style::Unmergeable {
    fn default() -> Self {
        Self {
            color: Color::black(),
            background_color: None,
            width: String::from("auto"),
            height: String::from("auto"),
            border: Default::default(),
            font: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            flex: Default::default(),
            text_transform: Default::default(),
            break_before: Default::default(),
            break_after: Default::default(),
            break_inside: Default::default(),
            line_height: None,
            debug: false,
        }
    }
}

impl Style::Mergeable {
    /// This is meant to emulate how if you set a color on a parent, the child
    /// gets that color by default unless overridden
    ///
    /// With the exception of inherited styles, the target node, self, should win in all cases,
    /// even if the parent has a style where the target node does not.
    /// For inherited styles, inherited styles should only "win" where the
    /// target node does not have any style set.
    pub fn merge_inherited_styles(&self, parent_style: &Style::Mergeable) -> Style::Mergeable {
        let mut style = self.clone();

        style.font = if let Some(font) = &parent_style.font {
            font.merge_optional(&style.font)
        } else {
            style.font.clone()
        };

        if style.color.is_none() {
            style.color = parent_style.color.clone();
        }

        if style.debug.is_none() {
            style.debug = parent_style.debug;
        }

        style
    }
}

impl Style::Unmergeable {
    pub fn merge_style(&self, rhs: &Style::Mergeable) -> Style::Unmergeable {
        let base = Style::Mergeable::from(self.clone());

        let merged: Style::Mergeable = base.merge(rhs);

        merged.into()
    }
}
