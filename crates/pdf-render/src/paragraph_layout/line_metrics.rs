use std::fmt::Display;

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

impl LineMetrics {
    pub fn top_edge(&self) -> Pt {
        self.baseline - self.ascent
    }

    pub fn bottom_edge(&self) -> Pt {
        self.baseline + self.descent
    }
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

impl Display for LineMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
__________________________________________________________________ _       ^   ___ {} (top edge)
  ______                                                           ___ {} (ascent)
    /                                                    /                 | 
---/---------------__----__----__---)__----__------__---/__------- -       |
  /      /   /   /   ) /   ) /   ) /   ) /   )   /   ) /   ) /   /         | --- {} (height)
_/______(___/___/___/_(___/_(___/_/_____(___(___/___/_/___/_(___/_ ___ {} (baseline from top of paragraph)
           /   /               /               /               /           |
       (_ /   /            (_ /               /            (_ /    ___ {} (descent)
                                                                           V   ___ {} (bottom edge)

"#,
            self.top_edge(),
            self.ascent,
            self.height,
            self.baseline,
            self.descent,
            self.bottom_edge()
        )
    }
}
