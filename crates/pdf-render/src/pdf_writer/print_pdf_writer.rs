use std::{collections::HashMap, io::BufWriter};

use printpdf::{IndirectFontRef, PdfDocument, PdfDocumentReference};

use crate::{
    error::InternalServerError,
    fonts::{FontAttributes, FontCollection, FontId},
    geometry::{Mm, Size},
};

use super::pdf_writer::PdfWriter;

pub struct PrintPdfWriter {
    raw_pdf_doc: PdfDocumentReference,
    font_families: HashMap<FontId, IndirectFontRef>,
}

impl PrintPdfWriter {}

impl PdfWriter for PrintPdfWriter {
    fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self {
        let dimensions = page_size.into();

        let (doc, _, _) = PdfDocument::new(
            doc_title,
            dimensions.width.into(),
            dimensions.height.into(),
            "Layer 1",
        );

        Self {
            raw_pdf_doc: doc,
            font_families: HashMap::new(),
        }
    }

    fn write_line(&mut self, font_id: &FontId, pdf_line: &str) -> &mut Self {
        self
    }

    fn save<W: std::io::Write>(
        self,
        pdf_doc_writer: W,
    ) -> Result<W, crate::error::PdfGenerationError> {
        let mut buf_writer = BufWriter::new(pdf_doc_writer);

        self.raw_pdf_doc.save(&mut buf_writer).unwrap();

        let write_result = buf_writer
            .into_inner()
            .map_err(|e| InternalServerError::WriteError(e.into()));

        Ok(write_result?)
    }

    fn load_fonts(&mut self, font_collection: &FontCollection) -> &mut Self {
        for (_, font_family) in font_collection.families.iter() {
            // let mut font_family_fonts = HashMap::new();

            for (_, data) in font_family.fonts_by_attribute.iter() {
                let indirect_font_ref =
                    self.raw_pdf_doc.add_external_font(data.as_bytes()).unwrap();

                self.font_families.insert(data.font_id, indirect_font_ref);
                // font_family_fonts.insert(attribute, indirect_font_ref);
            }

            // self.font_families
            //     .insert(family_name.clone(), font_family_fonts);
        }

        self
    }
}
