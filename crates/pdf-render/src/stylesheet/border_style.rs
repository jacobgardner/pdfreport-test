use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::values::Color;

use super::BorderRadiusStyle;

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