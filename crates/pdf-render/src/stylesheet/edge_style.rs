use optional_merge_derive::mergeable;
use ts_rs::TS;

#[mergeable]
#[derive(TS, Clone, Debug, PartialEq)]
#[ts(export)]
pub struct EdgeStyle {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for EdgeStyle::Unmergeable {
    fn default() -> Self {
        Self {
            top: 0.,
            right: 0.,
            bottom: 0.,
            left: 0.,
        }
    }
}