use pdf_builder::{print_pdf_writer::PrintPdfWriter, PdfBuilder};
use std::io::Write;

pub mod dom;
pub mod error;
mod fonts;
pub mod geometry;
pub mod page_sizes;
pub mod pdf_builder;

use error::PdfGenerationError;

pub fn build_pdf_from_dom<W: Write>(
    pdf_dom: &dom::PdfDom,
    pdf_doc_writer: W,
) -> Result<W, PdfGenerationError> {
    let pdf_writer = PrintPdfWriter::new(&pdf_dom.document_title, page_sizes::LETTER);

    let mut pdf_builder = PdfBuilder::new(pdf_writer);

    // pdf_writer.write_line("asdbd").write_line("aosdifjasfodij");

    pdf_builder.into_inner().save(pdf_doc_writer)
}
