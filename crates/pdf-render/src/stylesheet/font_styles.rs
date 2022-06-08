use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::values::Pt;
use crate::fonts::{FontSlant, FontWeight};

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct FontStyles {
    pub family: String,
    #[ts(type = "string | number")]
    pub size: Pt,
    pub style: FontSlant,
    pub weight: FontWeight,
}

impl Default for FontStyles::Unmergeable {
    fn default() -> Self {
        Self {
            family: String::from("sans-serif"),
            size: Pt(12.),
            style: FontSlant::Normal,
            weight: FontWeight::Regular,
        }
    }
}
