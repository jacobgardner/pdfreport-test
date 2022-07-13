use crate::values::Pt;
use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use serde_with::skip_serializing_none;
use ts_rs::TS;

mergeable_fn! {
    source => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct BorderRadiusStyle {
            pub top_right: Pt,
            pub bottom_right: Pt,
            pub bottom_left: Pt,
            pub top_left: Pt,
        }
    },
    mergeable => {
        #[derive(Deserialize, TS)]
        #[skip_serializing_none]
        #[ts(export, rename_all = "camelCase", rename = "BorderRadiusStyle")]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct MergeableBorderRadiusStyle;

    },
    unmergeable => {
        pub struct BorderRadiusStyle;
    }
}

impl Default for BorderRadiusStyle {
    fn default() -> Self {
        Self {
            top_right: Pt(0.),
            bottom_right: Pt(0.),
            bottom_left: Pt(0.),
            top_left: Pt(0.),
        }
    }
}

impl BorderRadiusStyle {
    pub fn new(radius: Pt) -> Self {
        Self {
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
            top_left: radius,
        }
    }
}
