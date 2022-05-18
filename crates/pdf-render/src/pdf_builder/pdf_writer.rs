use crate::{error::PdfGenerationError, fonts::FontId};

pub trait PdfWriter {
    fn write_line(&mut self, font: FontId, pdf_line: &str)
        -> Result<&mut Self, PdfGenerationError>;
}
