use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use ts_rs::TS;

use crate::fonts::{FontSlant, FontWeight};
use crate::values::Pt;

mergeable_fn! {
    source => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct FontStyles {
            pub family: String,
            pub size: Pt,
            pub style: FontSlant,
            pub weight: FontWeight,
            pub letter_spacing: Pt,
        }
    }
    mergeable => {
        #[derive(Deserialize, TS)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        #[ts(export, rename_all = "camelCase", rename = "FontStyles")]
        pub struct MergeableFontStyles;
    }
    unmergeable => {
        pub struct FontStyles;
    }
}

impl Default for FontStyles {
    fn default() -> Self {
        Self {
            family: String::from("sans-serif"),
            size: Pt(12.),
            style: FontSlant::Normal,
            weight: FontWeight::Regular,
            letter_spacing: Pt(0.),
        }
    }
}
