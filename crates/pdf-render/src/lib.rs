use pdf_writer::PdfWriter;
use printpdf::*;
use std::io::Write;

pub mod dom;
pub mod error;
pub mod pdf_writer;
pub mod geometry;
pub mod page_sizes;

use error::PdfGenerationError;

pub fn build_pdf_from_dom<W: Write>(
    pdf_dom: &dom::PdfDom,
    pdf_doc_writer: W,
) -> Result<W, PdfGenerationError> {
    let pdf_writer = PdfWriter::new(&pdf_dom.document_title, page_sizes::LETTER);

    pdf_writer.write_line("asdbd").write_line("aosdifjasfodij");

    pdf_writer.save(pdf_doc_writer)

}
