//! This is a generic interface to build up an unstructured
//!  document (like a PDF)

use crate::{
    block_layout::{layout_engine::NodeLayout, paginated_layout::{PaginatedLayout, DrawableNode, PaginatedNode}},
    doc_structure::{DomNode, HasNodeId},
    error::DocumentGenerationError,
    page_sizes,
    paragraph_layout::RenderedTextBlock,
    stylesheet::Style,
    utils::node_lookup::NodeLookup,
    values::{Color, Pt, Rect, Size},
};

pub use self::document_writer::UnstructuredDocumentWriter;

pub struct DocumentBuilder<Writer: UnstructuredDocumentWriter> {
    unstructured_doc_writer: Writer,
    page_size: Size<Pt>,
}

mod document_writer;

impl<Writer: UnstructuredDocumentWriter> DocumentBuilder<Writer> {
    pub fn new(raw_document_writer: Writer, page_size: impl Into<Size<Pt>>) -> Self {
        Self {
            unstructured_doc_writer: raw_document_writer,
            page_size: page_size.into(),
        }
    }

    // pub fn draw_dom_node(
    //     &mut self,
    //     dom_node: &DomNode,
    //     node_lookup: &NodeLookup,
    //     layout: &NodeLayout,
    // ) -> Result<&mut Self, DocumentGenerationError> {
    //     let style = node_lookup.get_style(dom_node.node_id());

    //     if style.debug {
    //         let mut margin_rect = Rect {
    //             left: layout.left - Pt(style.margin.left),
    //             top: layout.top - Pt(style.margin.top),
    //             width: layout.width + Pt(style.margin.right + style.margin.left),
    //             height: layout.height + Pt(style.margin.top + style.margin.bottom),
    //         };

    //         let mut border_rect = Rect {
    //             left: layout.left,
    //             top: layout.top,
    //             width: layout.width,
    //             height: layout.height,
    //         };

    //         let mut content_rect = Rect {
    //             left: border_rect.left + Pt(style.padding.left),
    //             top: border_rect.top + Pt(style.padding.top),
    //             width: border_rect.width - Pt(style.padding.right + style.padding.left),
    //             height: border_rect.height - Pt(style.padding.top + style.padding.bottom),
    //         };

    //         margin_rect.top = Pt::from(page_sizes::LETTER.height) - margin_rect.top;
    //         border_rect.top = Pt::from(page_sizes::LETTER.height) - border_rect.top;
    //         content_rect.top = Pt::from(page_sizes::LETTER.height) - content_rect.top;

    //         self.unstructured_doc_writer.draw_rect(
    //             margin_rect,
    //             Pt(1.),
    //             Some(Color::try_from("green").unwrap()),
    //             None,
    //             None,
    //         );

    //         self.unstructured_doc_writer.draw_rect(
    //             border_rect,
    //             Pt(1.),
    //             Some(Color::try_from("red").unwrap()),
    //             None,
    //             None,
    //         );

    //         self.unstructured_doc_writer.draw_rect(
    //             content_rect,
    //             Pt(1.),
    //             Some(Color::try_from("blue").unwrap()),
    //             None,
    //             None,
    //         );
    //     }

    //     Ok(self)
    // }

    pub fn write_text_block(
        &mut self,
        layout: &PaginatedLayout,
        style: &Style::Unmergeable,
        text_block: &RenderedTextBlock,
    ) -> Result<&mut Self, DocumentGenerationError> {
        self.unstructured_doc_writer
            .draw_text_block(layout, style, text_block)?;

        Ok(self)
    }
    
    pub fn draw_node(&mut self, node: &PaginatedNode) -> Result<&mut Self, DocumentGenerationError> {
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
        fn draw_text_block(
            &mut self,
            layout: &PaginatedLayout,
            style: &Style::Unmergeable,
            text_block: &RenderedTextBlock,
        ) -> Result<&mut Self, DocumentGenerationError> {
            todo!()
        }

        fn draw_node(&mut self, node: &PaginatedNode) -> Result<&mut Self, DocumentGenerationError> {
        todo!()
    }
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn can_write_line() {
        let writer = MockDocWriter::new("Test Title");

        let mut builder: DocumentBuilder<MockDocWriter> =
            DocumentBuilder::new(writer, page_sizes::LETTER);

        // let f1 = FontId::new();

        // builder.write_line(f1, "Hello").unwrap();

        // let mut output: Vec<u8> = vec![];

        // builder.into_inner().save(output).unwrap();

        // let string_version = std::str::from_utf8(&output).unwrap();

        // assert_eq!(string_version, "Test Title - Hello");
    }
}
