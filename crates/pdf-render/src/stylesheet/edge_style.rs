use crate::values::Pt;
use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use ts_rs::TS;

mergeable_fn! {
    source => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct EdgeStyle {
            pub top: Pt,
            pub right: Pt,
            pub bottom: Pt,
            pub left: Pt,
        }
    }
    mergeable => {
        #[derive(Deserialize, TS)]
        #[ts(export, rename = "EdgeStyle")]
        pub struct MergeableEdgeStyle;
    },
    unmergeable => {
        #[derive(Deserialize, TS)]
        #[ts(export, rename = "RequiredEdgeStyle")]
        pub struct EdgeStyle;
    }
}

// TODO: Add custom deserialize to support "top right bottom left" "vertical
// horizontal", "all".
// impl<'de> Deserialize<'de> for MergeableEdgeStyle {

// }

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            top: Pt(0.),
            right: Pt(0.),
            bottom: Pt(0.),
            left: Pt(0.),
        }
    }
}

impl EdgeStyle {
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
