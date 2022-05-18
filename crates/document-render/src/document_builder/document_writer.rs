use crate::{error::DocumentGenerationError, fonts::FontId};

pub trait DocumentWriter {
    fn write_line(
        &mut self,
        font: FontId,
        pdf_line: &str,
    ) -> Result<&mut Self, DocumentGenerationError>;
}
