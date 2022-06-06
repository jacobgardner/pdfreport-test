use crate::values::Pt;
use skia_safe::textlayout as skia_layout;

#[derive(Clone, Debug)]
pub struct LineMetrics {
    pub ascent: Pt,
    pub descent: Pt,
    pub baseline: Pt,
    pub height: Pt,
    pub width: Pt,
    pub left: Pt,
    // pub leading: Pt,
}

impl<'a> From<&skia_layout::LineMetrics<'a>> for LineMetrics {
    fn from(metrics: &skia_layout::LineMetrics) -> Self {
        Self {
            ascent: metrics.ascent.into(),
            descent: metrics.descent.into(),
            baseline: metrics.baseline.into(),
            height: metrics.height.into(),
            width: metrics.width.into(),
            left: metrics.left.into(),
        }
    }
}
