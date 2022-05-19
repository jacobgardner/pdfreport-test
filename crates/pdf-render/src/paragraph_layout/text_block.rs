use crate::rich_text::RichText;

use super::LineMetrics;

pub struct RenderedTextLine {
    pub rich_text: RichText,
    pub line_metrics: LineMetrics,
}

#[derive(Default)]
pub struct RenderedTextBlock {
    // block_metrics: BlockMetrics
    pub lines: Vec<RenderedTextLine>,
}
