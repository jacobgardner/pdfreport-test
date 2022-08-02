use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use ts_rs::TS;

use super::{Direction, FlexAlign, FlexWrap};

mergeable_fn! {
    source => {
        #[derive(Clone, Debug, PartialEq)]
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
    }
    mergeable => {
        #[derive(Deserialize, TS)]
        #[ts(export, rename = "FlexStyle")]
        pub struct MergeableFlexStyle;
    }
    unmergeable => {
        pub struct FlexStyle;
    }
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction: Direction::Column,
            wrap: FlexWrap::NoWrap,
            align_items: FlexAlign::Stretch,
            align_self: FlexAlign::Auto,
            grow: 0.,
            shrink: 1.,
            // basis: String::from("undefined"),
        }
    }
}
