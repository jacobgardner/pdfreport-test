//! Some standard page sizes for the PDFs
use crate::values::{Mm, Size};

// Apparently this is different from letter
pub const A4: Size<Mm> = Size {
    width: Mm(210.),
    height: Mm(297.),
};

pub const LETTER: Size<Mm> = Size {
    width: Mm(215.9),
    height: Mm(279.4),
};
