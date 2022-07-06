use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::stylesheet::EdgeStyle;
use crate::values::Color;

use super::BorderRadiusStyle;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BorderStyle {
    #[mergeable(nested)]
    pub width: EdgeStyle,
    pub color: Color,
    #[mergeable(nested)]
    pub radius: BorderRadiusStyle,
}

#[allow(clippy::derivable_impls)]
impl Default for BorderStyle::Unmergeable {
    fn default() -> Self {
        Self {
            width: Default::default(),
            color: Default::default(),
            radius: Default::default(),
        }
    }
}
