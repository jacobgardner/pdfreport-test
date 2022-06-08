use serde::Deserialize;
use ts_rs::TS;

#[derive(TS, Clone, Debug, PartialEq, Deserialize)]
#[ts(export)]
pub enum PageBreakRule {
  Auto,
  Avoid,
  Always
}

impl Default for PageBreakRule {
    fn default() -> Self {
        Self::Auto
    }
}