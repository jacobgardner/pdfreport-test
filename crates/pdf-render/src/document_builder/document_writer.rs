use crate::{error::DocumentGenerationError, fonts::FontId, rich_text::RichTextLine};

pub trait DocumentWriter {
    fn write_line(&mut self, line: RichTextLine)
        -> Result<&mut Self, DocumentGenerationError>;
}
