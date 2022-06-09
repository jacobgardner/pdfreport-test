use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::fonts::{FontSlant, FontWeight};
use crate::values::Pt;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export)]
pub struct FontStyles {
    pub family: String,
    #[ts(type = "string | number")]
    pub size: Pt,
    pub style: FontSlant,
    pub weight: FontWeight,
    #[ts(type = "string | number")]
    pub letter_spacing: Pt,
}

impl Default for FontStyles::Unmergeable {
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
