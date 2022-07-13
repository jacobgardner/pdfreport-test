use crate::values::Pt;
use optional_merge_derive::mergeable_fn;
use ts_rs::TS;
use serde::Deserialize;


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
        #[derive(Deserialize)]
        pub struct EdgeStyle;
    }
}

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
