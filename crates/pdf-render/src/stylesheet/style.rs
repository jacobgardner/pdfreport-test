use merges::Merges;
use optional_merge_derive::mergeable;
use serde::Deserialize;
use ts_rs::TS;

use crate::values::Color;

use super::FontStyles;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct BorderRadiusStyle {
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
    pub top_left: f32,
}

impl Default for BorderRadiusStyle::Unmergeable {
    fn default() -> Self {
        Self {
            top_right: 0.,
            bottom_right: 0.,
            bottom_left: 0.,
            top_left: 0.,
        }
    }
}

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct BorderStyle {
    pub width: f32,
    #[ts(type = "string")]
    pub color: Color,
    #[mergeable(nested)]
    pub radius: BorderRadiusStyle,
}

impl Default for BorderStyle::Unmergeable {
    fn default() -> Self {
        Self {
            width: 0.,
            color: Default::default(),
            radius: Default::default(),
        }
    }
}

#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum Direction {
    Column,
    Row,
}

#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum FlexAlign {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct FlexStyle {
    // Add other attributes as needed...
    pub direction: Direction,
    pub wrap: FlexWrap,
    pub align_items: FlexAlign,
    pub align_self: FlexAlign,
    pub grow: f32,
    pub shrink: f32,
    pub basis: String,
}

impl Default for FlexStyle::Unmergeable {
    fn default() -> Self {
        Self {
            direction: Direction::Column,
            wrap: FlexWrap::NoWrap,
            align_items: FlexAlign::Auto,
            align_self: FlexAlign::Auto,
            grow: 0.,
            shrink: 1.,
            basis: String::from("undefined"),
        }
    }
}

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct EdgeStyle {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for EdgeStyle::Unmergeable {
    fn default() -> Self {
        Self {
            top: 0.,
            right: 0.,
            bottom: 0.,
            left: 0.,
        }
    }
}

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
