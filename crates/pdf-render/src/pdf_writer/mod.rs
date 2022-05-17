// use printpdf::*;
use std::io::{BufWriter, Write};

use printpdf::{PdfDocument, PdfDocumentReference};

use crate::{
    error::{InternalServerError, PdfGenerationError},
    geometry::{Mm, Size},
};

pub struct PdfWriter {
    raw_pdf_doc: PdfDocumentReference,
}

impl PdfWriter {
    pub fn new(doc_title: &str, page_size: Size<Mm>) -> Self {
        let (doc, _, _) = PdfDocument::new(
            doc_title,
            page_size.width.into(),
            page_size.height.into(),
            "Layer 1",
        );

        Self { raw_pdf_doc: doc }
    }

    pub fn write_line(&self, pdf_line: &str) -> &Self {
        self
    }

    pub fn save<W: Write>(self, pdf_doc_writer: W) -> Result<W, PdfGenerationError> {
        let mut buf_writer = BufWriter::new(pdf_doc_writer);

        self.raw_pdf_doc.save(&mut buf_writer).unwrap();

        Ok(buf_writer
            .into_inner()
            .map_err(|e| InternalServerError::WriteError(e.into()))?)
    }
}
