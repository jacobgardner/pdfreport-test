use pdf_writer::{PdfBuilder, print_pdf_writer::PrintPdfWriter};
use printpdf::*;
use std::io::Write;

pub mod dom;
pub mod error;
pub mod pdf_writer;
pub mod geometry;
pub mod page_sizes;
mod fonts;

use error::PdfGenerationError;

pub fn build_pdf_from_dom<W: Write>(
    pdf_dom: &dom::PdfDom,
    pdf_doc_writer: W,
) -> Result<W, PdfGenerationError> {
    let mut pdf_writer: PdfBuilder<PrintPdfWriter> = PdfBuilder::new(&pdf_dom.document_title, page_sizes::LETTER);

    // pdf_writer.write_line("asdbd").write_line("aosdifjasfodij");

    pdf_writer.save(pdf_doc_writer)

}
