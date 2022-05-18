use crate::{error::DocumentGenerationError, rich_text::RichText};

pub trait DocumentWriter {
    fn write_line(&mut self, line: RichText)
        -> Result<&mut Self, DocumentGenerationError>;
}
