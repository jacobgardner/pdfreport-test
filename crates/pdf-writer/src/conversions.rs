
use document_render::geometry::Mm;
use printpdf::Mm as Ppmm;

pub(crate) trait AsPrintPdfMm {
  fn as_mm(&self) -> Ppmm;
}

impl AsPrintPdfMm for Mm {
    fn as_mm(&self) -> Ppmm {
        Ppmm(self.0)
    }
}

