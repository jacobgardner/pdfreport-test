use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use document_render::{
    document_builder::DocumentWriter,
    error::DocumentGenerationError,
    fonts::{FontCollection, FontId},
    geometry::{Mm, Size},
};
use error::PdfAssembleError;
use printpdf::{IndirectFontRef, PdfDocument, PdfDocumentReference};

mod conversions;
mod error;

mod prelude {
    pub(crate) use crate::conversions::AsPrintPdfMm;
}

use prelude::*;

pub struct PrintPdfWriter {
    raw_pdf_doc: PdfDocumentReference,
    font_families: HashMap<FontId, IndirectFontRef>,
}

impl PrintPdfWriter {
    pub fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self {
        let dimensions = page_size.into();

        let (doc, _, _) = PdfDocument::new(
            doc_title,
            dimensions.width.as_mm(),
            dimensions.height.as_mm(),
            "Layer 1",
        );

        Self {
            raw_pdf_doc: doc,
            font_families: HashMap::new(),
        }
    }

    fn load_fonts(
        &mut self,
        font_collection: &FontCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        for (family_name, font_family) in font_collection.families.iter() {
            for (attributes, data) in font_family.fonts_by_attribute.iter() {
                let indirect_font_ref = self
                    .raw_pdf_doc
                    .add_external_font(data.as_bytes())
                    .map_err(|e| PdfAssembleError::LoadFontError {
                        source: Box::new(e),
                        family_name: family_name.clone(),
                        attributes: *attributes,
                    })?;

                self.font_families.insert(data.font_id(), indirect_font_ref);
            }
        }

        Ok(self)
    }

    pub fn save<W: Write>(self, pdf_doc_writer: W) -> Result<W, DocumentGenerationError> {
        let mut buf_writer = BufWriter::new(pdf_doc_writer);

        self.raw_pdf_doc.save(&mut buf_writer).unwrap();

        let write_result = buf_writer
            .into_inner()
            .map_err(|e| PdfAssembleError::WritePdfError(e.into()));

        Ok(write_result?)
    }
}

impl DocumentWriter for PrintPdfWriter {
    fn write_line(
        &mut self,
        font_id: FontId,
        pdf_line: &str,
    ) -> Result<&mut Self, DocumentGenerationError> {
        // self.font_families.get(font_id)

        Ok(self)
    }
}
