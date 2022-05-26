use optional_merge_derive::mergeable;
use ts_rs::TS;

use super::{Direction, FlexWrap, FlexAlign};


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
    // pub basis: String,
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
            // basis: String::from("undefined"),
        }
    }
}