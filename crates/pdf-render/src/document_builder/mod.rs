//! This is a generic interface to build up an unstructured
//!  document (like a PDF)

use crate::{
    error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock,
    values::{Point, Pt},
};

pub use self::document_writer::DocumentWriter;

pub struct DocumentBuilder<Writer: DocumentWriter> {
    raw_document_writer: Writer,
    draw_debug_lines: bool,
}

mod document_writer;

impl<Writer: DocumentWriter> DocumentBuilder<Writer> {
    pub fn new(raw_document_writer: Writer) -> Self {
        Self {
            raw_document_writer,
            draw_debug_lines: false,
        }
    }

    pub fn enable_debug_lines(&mut self, value: bool) -> &mut Self {
        self.draw_debug_lines = value;

        self
    }

    pub fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError> {
        self.raw_document_writer
            .write_text_block(text_block, position)?;

        Ok(self)
    }

    pub fn into_inner(self) -> Writer {
        self.raw_document_writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDocWriter {
        title: String,
        lines: Vec<String>,
    }

    impl MockDocWriter {
        fn new(doc_title: &str) -> Self {
            Self {
                title: doc_title.to_owned(),
                lines: vec![],
            }
        }
    }

    impl DocumentWriter for MockDocWriter {
        fn write_text_block(
            &mut self,
            text_block: crate::paragraph_layout::RenderedTextBlock,
            position: crate::values::Point<crate::values::Pt>,
        ) -> Result<&mut Self, DocumentGenerationError> {
            todo!()
        }
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn can_write_line() {
        let writer = MockDocWriter::new("Test Title");

        let mut builder: DocumentBuilder<MockDocWriter> = DocumentBuilder::new(writer);

        // let f1 = FontId::new();

        // builder.write_line(f1, "Hello").unwrap();

        // let mut output: Vec<u8> = vec![];

        // builder.into_inner().save(output).unwrap();

        // let string_version = std::str::from_utf8(&output).unwrap();

        // assert_eq!(string_version, "Test Title - Hello");
    }
}
