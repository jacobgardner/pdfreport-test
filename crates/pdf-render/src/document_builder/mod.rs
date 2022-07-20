//! This is a generic interface to build up an unstructured
//!  document (like a PDF)

use crate::{
    block_layout::paginated_layout::PaginatedNode, error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock, stylesheet::Style,
};

pub use self::document_writer::UnstructuredDocumentWriter;

pub struct DocumentBuilder<Writer: UnstructuredDocumentWriter> {
    unstructured_doc_writer: Writer,
}

mod document_writer;

impl<Writer: UnstructuredDocumentWriter> DocumentBuilder<Writer> {
    pub fn new(raw_document_writer: Writer) -> Self {
        Self {
            unstructured_doc_writer: raw_document_writer,
        }
    }

    pub fn write_text_block(
        &mut self,
        node: &PaginatedNode,
        style: &Style::Unmergeable,
        text_block: &RenderedTextBlock,
    ) -> Result<&mut Self, DocumentGenerationError> {
        self.unstructured_doc_writer
            .draw_text_block(node, style, text_block)?;

        Ok(self)
    }

    pub fn draw_node(
        &mut self,
        node: &PaginatedNode,
    ) -> Result<&mut Self, DocumentGenerationError> {
        self.unstructured_doc_writer.draw_node(node)?;

        Ok(self)
    }

    pub fn into_inner(self) -> Writer {
        self.unstructured_doc_writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDocWriter {}

    impl MockDocWriter {
        fn new(_doc_title: &str) -> Self {
            Self {}
        }
    }

    impl UnstructuredDocumentWriter for MockDocWriter {
        fn draw_text_block(
            &mut self,
            _node: &PaginatedNode,
            _style: &Style::Unmergeable,
            _text_block: &RenderedTextBlock,
        ) -> Result<&mut Self, DocumentGenerationError> {
            todo!()
        }

        fn draw_node(
            &mut self,
            _node: &PaginatedNode,
        ) -> Result<&mut Self, DocumentGenerationError> {
            todo!()
        }
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn can_write_line() {
        let writer = MockDocWriter::new("Test Title");

        let mut _builder: DocumentBuilder<MockDocWriter> = DocumentBuilder::new(writer);

        // let f1 = FontId::new();

        // builder.write_line(f1, "Hello").unwrap();

        // let mut output: Vec<u8> = vec![];

        // builder.into_inner().save(output).unwrap();

        // let string_version = std::str::from_utf8(&output).unwrap();

        // assert_eq!(string_version, "Test Title - Hello");
    }
}
