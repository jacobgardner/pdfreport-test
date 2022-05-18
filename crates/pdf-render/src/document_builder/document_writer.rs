use crate::{error::DocumentGenerationError, rich_text::RichTextLine};

pub trait DocumentWriter {
    fn write_line(&mut self, line: RichTextLine)
        -> Result<&mut Self, DocumentGenerationError>;
}
