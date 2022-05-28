use optional_merge_derive::mergeable;
use ts_rs::TS;

use crate::fonts::{FontSlant, FontWeight};

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct FontStyles {
    pub family: String,
    pub size: f32,
    pub style: FontSlant,
    pub weight: FontWeight,
}

impl Default for FontStyles::Unmergeable {
    fn default() -> Self {
        Self {
            // FIXME:
            // TODO: Don't use Inter
            family: String::from("Inter"),
            size: 12.,
            style: FontSlant::Normal,
            weight: FontWeight::Regular,
        }
    }
}
