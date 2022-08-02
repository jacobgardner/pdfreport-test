use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use ts_rs::TS;

use crate::stylesheet::EdgeStyle;
use crate::values::Color;

use super::BorderRadiusStyle;

mergeable_fn! {
    source => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct BorderStyle {
            #[mergeable(nested)]
            pub width: EdgeStyle,
            pub color: Color,
            #[mergeable(nested)]
            pub radius: BorderRadiusStyle,
        }
    }
    mergeable => {
        #[derive(Deserialize, TS)]
        #[ts(export, rename = "BorderStyle")]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct MergeableBorderStyle;
    }
    unmergeable => {
        pub struct BorderStyle;
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            width: Default::default(),
            color: Default::default(),
            radius: Default::default(),
        }
    }
}
