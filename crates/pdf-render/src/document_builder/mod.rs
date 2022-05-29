//! This is a generic interface to build up an unstructured
//!  document (like a PDF)

use crate::{
    block_layout::layout_engine::NodeLayout,
    doc_structure::{DomNode, HasNodeId},
    error::DocumentGenerationError,
    page_sizes,
    paragraph_layout::RenderedTextBlock,
    utils::dom_lookup::NodeLookup,
    values::{Color, Point, Pt, Rect},
};

pub use self::document_writer::UnstructuredDocumentWriter;

pub struct DocumentBuilder<Writer: UnstructuredDocumentWriter> {
    unstructured_doc_writer: Writer,
    draw_debug_lines: bool,
}

mod document_writer;

impl<Writer: UnstructuredDocumentWriter> DocumentBuilder<Writer> {
    pub fn new(raw_document_writer: Writer) -> Self {
        Self {
            unstructured_doc_writer: raw_document_writer,
            draw_debug_lines: false,
        }
    }

    pub fn enable_debug_lines(&mut self, value: bool) -> &mut Self {
        self.draw_debug_lines = value;

        self
    }

    pub fn draw_dom_node(
        &mut self,
        dom_node: &DomNode,
        node_lookup: &NodeLookup,
        layout: &NodeLayout,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let style = node_lookup.get_style(dom_node.node_id());

        if style.debug {
            let mut margin_rect = Rect {
                left: layout.left - Pt(style.margin.left),
                top: layout.top - Pt(style.margin.top),
                width: layout.width + Pt(style.margin.right + style.margin.left),
                height: layout.height + Pt(style.margin.top + style.margin.bottom),
            };

            let mut border_rect = Rect {
                left: layout.left,
                top: layout.top,
                width: layout.width,
                height: layout.height,
            };

            let mut content_rect = Rect {
                left: border_rect.left + Pt(style.padding.left),
                top: border_rect.top + Pt(style.padding.top),
                width: border_rect.width - Pt(style.padding.right + style.padding.left),
                height: border_rect.height - Pt(style.padding.top + style.padding.bottom),
            };

            margin_rect.top = Pt::from(page_sizes::LETTER.height) - margin_rect.top;
            border_rect.top = Pt::from(page_sizes::LETTER.height) - border_rect.top;
            content_rect.top = Pt::from(page_sizes::LETTER.height) - content_rect.top;

            self.unstructured_doc_writer.draw_rect(
                margin_rect,
                Pt(1.),
                Some(Color::try_from("green").unwrap()),
                None,
                None,
            );

            self.unstructured_doc_writer.draw_rect(
                border_rect,
                Pt(1.),
                Some(Color::try_from("red").unwrap()),
                None,
                None,
            );

            self.unstructured_doc_writer.draw_rect(
                content_rect,
                Pt(1.),
                Some(Color::try_from("blue").unwrap()),
                None,
                None,
            );
        }

        Ok(self)
    }

    pub fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError> {
        self.unstructured_doc_writer
            .write_text_block(text_block, position)?;

        Ok(self)
    }

    pub fn into_inner(self) -> Writer {
        self.unstructured_doc_writer
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

    impl UnstructuredDocumentWriter for MockDocWriter {
        fn write_text_block(
            &mut self,
            text_block: crate::paragraph_layout::RenderedTextBlock,
            position: crate::values::Point<crate::values::Pt>,
        ) -> Result<&mut Self, DocumentGenerationError> {
            todo!()
        }

        fn draw_rect(
            &mut self,
            rect: crate::values::Rect<Pt>,
            border_width: Pt,
            border_color: Option<crate::values::Color>,
            background_color: Option<crate::values::Color>,
            border_radius: Option<crate::stylesheet::BorderRadiusStyle::Unmergeable>,
        ) {
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
