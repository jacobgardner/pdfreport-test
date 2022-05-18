use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use printpdf::{IndirectFontRef, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};

use crate::{
    document_builder::DocumentWriter,
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontCollection, FontId},
    geometry::{Mm, Size},
    rich_text::RichTextLine,
};

pub struct PrintPdfWriter {
    raw_pdf_doc: PdfDocumentReference,
    fonts: HashMap<FontId, IndirectFontRef>,
    page_layer_indices: Vec<(PdfPageIndex, Vec<PdfLayerIndex>)>,
}

impl PrintPdfWriter {
    pub fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self {
        let dimensions = page_size.into();

        let (doc, page_index, layer_index) = PdfDocument::new(
            doc_title,
            dimensions.width.into(),
            dimensions.height.into(),
            "Layer 1",
        );

        Self {
            raw_pdf_doc: doc,
            fonts: HashMap::new(),
            page_layer_indices: vec![(page_index, vec![layer_index])],
        }
    }

    pub fn load_fonts(
        &mut self,
        font_collection: &FontCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        // TODO: Lazily add fonts as they are used so we don't end up embedding
        // fonts we don't actually need
        for (family_name, font_family) in font_collection.as_ref().iter() {
            for (attributes, data) in font_family.as_ref().iter() {
                let indirect_font_ref = self
                    .raw_pdf_doc
                    .add_external_font(data.as_bytes())
                    .map_err(|e| InternalServerError::LoadFontError {
                        source: Box::new(e),
                        family_name: family_name.clone(),
                        attributes: *attributes,
                    })?;

                self.fonts.insert(data.font_id(), indirect_font_ref);
            }
        }

        Ok(self)
    }

    pub fn save<W: Write>(
        self,
        pdf_doc_writer: W,
    ) -> Result<W, crate::error::DocumentGenerationError> {
        let mut buf_writer = BufWriter::new(pdf_doc_writer);

        self.raw_pdf_doc.save(&mut buf_writer).unwrap();

        let write_result = buf_writer
            .into_inner()
            .map_err(|e| InternalServerError::WritePdfError(e.into()));

        Ok(write_result?)
    }
}

impl DocumentWriter for PrintPdfWriter {
    fn write_line(&mut self, pdf_line: RichTextLine) -> Result<&mut Self, DocumentGenerationError> {
        let (page_index, layers) = &self.page_layer_indices[0];
        let first_layer = layers[0];

        let page = self.raw_pdf_doc.get_page(*page_index);
        let layer = page.get_layer(first_layer);

        layer.begin_text_section();
        layer.set_text_cursor(printpdf::Mm(0.), printpdf::Mm(100.));
        for span in pdf_line.0.iter() {
            let font = self
                .fonts
                .get(&span.font_id)
                .ok_or(InternalServerError::FontIdNotLoaded)?;

            // TODO: I believe every time we set this, it adds more data to
            // the PDF, so we should probably optimize to only update the
            // styles when something has changed (keep track of last state)
            layer.set_font(font, span.size);
            layer.write_text(span.text.clone(), font);
        }

        layer.end_text_section();

        Ok(self)
    }
}
