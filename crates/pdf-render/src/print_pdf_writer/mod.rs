//! This is ultimately what takes nodes that have been styled
//!  and laid out and writes them to a PDF.
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use printpdf::{
    IndirectFontRef, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex, TextMatrix,
};

use crate::{
    document_builder::DocumentWriter,
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontCollection, FontId},
    paragraph_layout::RenderedTextBlock,
    rich_text::RichText,
    values::{Mm, Point, Pt, Size},
};

pub struct PrintPdfWriter<'a> {
    raw_pdf_doc: PdfDocumentReference,
    fonts: HashMap<FontId, IndirectFontRef>,
    page_layer_indices: Vec<(PdfPageIndex, Vec<PdfLayerIndex>)>,
    font_collection: &'a FontCollection,
}

impl<'a> PrintPdfWriter<'a> {
    pub fn new(
        doc_title: &str,
        page_size: impl Into<Size<Mm>>,
        font_collection: &'a FontCollection,
    ) -> Self {
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
            font_collection,
        }
    }

    pub fn get_font(
        &mut self,
        font_id: FontId,
    ) -> Result<&IndirectFontRef, DocumentGenerationError> {
        // We have to do this since NLL are not yet implemented in Rust yet
        if self.fonts.contains_key(&font_id) {
            Ok(self
                .fonts
                .get(&font_id)
                .expect("We just checked for its existence"))
        } else {
            let font_data = self
                .font_collection
                .get_font(font_id)
                .ok_or(InternalServerError::FontIdNotLoaded)?;

            let font_ref = self
                .raw_pdf_doc
                .add_external_font(font_data.as_bytes())
                .map_err(|e| InternalServerError::LoadFontError {
                    source: Box::new(e),
                    family_name: font_data.family_name().to_owned(),
                    attributes: *font_data.attributes(),
                })?;

            self.fonts.insert(font_data.font_id(), font_ref);

            Ok(self
                .fonts
                .get(&font_data.font_id())
                .expect("We just inserted it so it has to exist"))
        }
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

impl<'a> DocumentWriter for PrintPdfWriter<'a> {
    fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let (page_index, layers) = &self.page_layer_indices[0];
        let first_layer = layers[0];

        let page = self.raw_pdf_doc.get_page(*page_index);
        let layer = page.get_layer(first_layer);

        layer.begin_text_section();

        let x = printpdf::Pt::from(position.x);
        let y = printpdf::Pt::from(position.y);

        let mut current_y = y;
        for line in text_block.lines.iter() {
            layer.set_text_matrix(TextMatrix::Translate(
                x + line.line_metrics.left.into(),
                current_y - line.line_metrics.ascent.into(),
            ));

            for span in line.rich_text.0.iter() {
                let font = self
                    .font_collection
                    .lookup_font(&span.font_family, &span.attributes)?;

                let font = self.get_font(font.font_id())?;

                // TODO: I believe every time we set this, it adds more data to
                // the PDF, so we should probably optimize to only update the
                // styles when something has changed (keep track of last state)
                layer.set_font(font, span.size.0);
                layer.set_fill_color(span.color.clone().into());
                layer.write_text(span.text.clone(), font);
            }

            current_y -= line.line_metrics.height.into();
        }

        layer.end_text_section();

        Ok(self)
    }
}
