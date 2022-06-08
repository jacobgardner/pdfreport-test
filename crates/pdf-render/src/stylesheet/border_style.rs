use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::values::Color;
use crate::stylesheet::EdgeStyle;

use super::BorderRadiusStyle;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BorderStyle {
    #[mergeable(nested)]
    pub width: EdgeStyle,
    #[ts(type = "string")]
    pub color: Color,
    #[mergeable(nested)]
    pub radius: BorderRadiusStyle,
}

impl Default for BorderStyle::Unmergeable {
    fn default() -> Self {
        Self {
            width: Default::default(),
            color: Default::default(),
            radius: Default::default(),
        }
    }
}
