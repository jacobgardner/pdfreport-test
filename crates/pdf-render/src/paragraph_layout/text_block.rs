use crate::{rich_text::RichText, values::Pt};

use super::LineMetrics;

#[derive(Debug, Clone)]
pub struct RenderedTextLine {
    pub rich_text: RichText,
    pub line_metrics: LineMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct RenderedTextBlock {
    // block_metrics: BlockMetrics
    pub lines: Vec<RenderedTextLine>,
}

impl RenderedTextBlock {
    pub fn height(&self) -> Pt {
        self.lines
            .iter()
            .fold(Pt(0.), |acc, line| acc + line.line_metrics.height)
    }

    pub fn width(&self) -> Pt {
        self.lines
            .iter()
            .max_by(|&x, &y| {
                x.line_metrics
                    .width
                    .partial_cmp(&y.line_metrics.width)
                    .expect("Lines should always have a non-inf size")
            })
            .map(|line| line.line_metrics.width)
            .unwrap_or(Pt(0.))
    }
}
