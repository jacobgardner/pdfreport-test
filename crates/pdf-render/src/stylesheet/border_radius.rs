use crate::values::Pt;
use optional_merge_derive::mergeable;
use ts_rs::TS;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BorderRadiusStyle {
    #[ts(type = "string | number")]
    pub top_right: Pt,
    #[ts(type = "string | number")]
    pub bottom_right: Pt,
    #[ts(type = "string | number")]
    pub bottom_left: Pt,
    #[ts(type = "string | number")]
    pub top_left: Pt,
}

impl Default for BorderRadiusStyle::Unmergeable {
    fn default() -> Self {
        Self {
            top_right: Pt(0.),
            bottom_right: Pt(0.),
            bottom_left: Pt(0.),
            top_left: Pt(0.),
        }
    }
}

impl BorderRadiusStyle::Unmergeable {
    pub fn new(radius: Pt) -> Self {
        Self {
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
            top_left: radius,
        }
    }
}
