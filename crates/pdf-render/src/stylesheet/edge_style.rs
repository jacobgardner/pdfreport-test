use crate::values::Pt;
use optional_merge_derive::mergeable;
use ts_rs::TS;
use serde::Deserialize;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct EdgeStyle {
    pub top: Pt,
    pub right: Pt,
    pub bottom: Pt,
    pub left: Pt,
}

// impl<'de> Deserialize<'de> for EdgeStyle::Mergeable {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         todo!()
//     }
// }

impl Default for EdgeStyle::Unmergeable {
    fn default() -> Self {
        Self {
            top: Pt(0.),
            right: Pt(0.),
            bottom: Pt(0.),
            left: Pt(0.),
        }
    }
}

impl EdgeStyle::Unmergeable {
    pub fn new(size: Pt) -> Self {
        Self {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }

    pub fn horizontal(&self) -> Pt {
        self.left + self.right
    }

    pub fn vertical(&self) -> Pt {
        self.top + self.bottom
    }
}
