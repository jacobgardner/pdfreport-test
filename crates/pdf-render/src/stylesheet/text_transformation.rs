use serde::Deserialize;
use ts_rs::TS;

// TODO: Move to its own file
#[derive(TS, Clone, Debug, PartialEq, Deserialize)]
#[ts(export)]
pub enum TextTransformation {
    None,
    Uppercase,
}

impl Default for TextTransformation {
    fn default() -> Self {
        Self::None
    }
}
