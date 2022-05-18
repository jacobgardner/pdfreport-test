use printpdf::Mm as PrintPdfMm;

use super::Mm;

impl From<Mm> for PrintPdfMm {
    fn from(ours: Mm) -> Self {
        PrintPdfMm(ours.0)
    }
}
