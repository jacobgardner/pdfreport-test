use printpdf::Pt;

pub struct LineMetric {
    pub start_index: usize,
    pub end_index: usize,
    pub ascent: Pt,
    pub height: Pt,
    pub left: Pt,
}

pub struct ParagraphMetrics {
    pub height: Pt,
    pub line_metrics: Vec<LineMetric>,
}
