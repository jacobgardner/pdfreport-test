use crate::{rich_text::RichText, values::Pt};

use super::LineMetrics;

#[derive(Clone)]
pub struct RenderedTextLine {
    pub rich_text: RichText,
    pub line_metrics: LineMetrics,
}

#[derive(Clone, Default)]
pub struct RenderedTextBlock {
    // block_metrics: BlockMetrics
    pub lines: Vec<RenderedTextLine>,
}

impl RenderedTextBlock {
    pub fn height(&self) -> Pt {
        Pt(self
            .lines
            .iter()
            .fold(0., |acc, line| acc + line.line_metrics.height.0))
    }
}
