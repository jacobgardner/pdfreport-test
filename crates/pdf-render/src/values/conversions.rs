use printpdf::Pt as PrintPdfPt;
use printpdf::Mm as PrintPdfMm;

use super::{Mm, Pt};

impl From<Mm> for PrintPdfMm {
    fn from(ours: Mm) -> Self {
        PrintPdfMm(ours.0)
    }
}

impl From<Pt> for PrintPdfPt {
    fn from(ours: Pt) -> Self {
        PrintPdfPt(ours.0)
    }
}
