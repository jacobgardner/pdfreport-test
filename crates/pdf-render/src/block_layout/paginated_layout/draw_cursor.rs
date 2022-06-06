use std::fmt::Display;

use crate::values::Pt;


pub(super) struct DrawCursor {
    pub y_offset: Pt,
    pub page_index: usize,
    /// I don't have a better name for this atm. This is meant to represent
    /// the amount we have to offset the next drawing node to compensate for
    /// the prior node that is broken across one or more pages. Basically, if
    /// only 25% of the prior node is visible on the current page, then we only
    /// need to offset the current node by 25% of the previous node where 100%
    /// is already built in.
    pub page_break_debt: Pt,
}

impl Display for DrawCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {}, {} -{}",
            self.page_index, self.y_offset, self.page_break_debt
        )
    }
}