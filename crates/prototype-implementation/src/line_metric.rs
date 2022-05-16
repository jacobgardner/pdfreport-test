use printpdf::Pt;

#[derive(Debug)]
pub struct LineMetric {
    pub start_index: usize,
    pub end_index: usize,
    pub ascent: Pt,
    pub descent: Pt,
    pub baseline: Pt,
    pub height: Pt,
    pub width: Pt,
    pub left: Pt,
}

#[derive(Debug)]
pub struct ParagraphMetrics {
    pub height: Pt,
    pub line_metrics: Vec<LineMetric>,
}
