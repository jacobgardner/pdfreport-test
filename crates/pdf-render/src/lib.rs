use error::InternalServerError;
use printpdf::*;
use std::io::{BufWriter, Write};

pub mod dom;
pub mod error;

use error::PdfGenerationError;

pub fn build_pdf_from_dom<W: Write>(
    pdf_dom: &dom::PdfDom,
    pdf_doc_writer: W,
) -> Result<W, PdfGenerationError> {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");

    let mut buf_writer = BufWriter::new(pdf_doc_writer);

    doc.save(&mut buf_writer).unwrap();

    Ok(buf_writer
        .into_inner()
        .map_err(|e| InternalServerError::WriteError(e.into()))?)
}
