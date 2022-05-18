use document_builder::DocumentBuilder;
use print_pdf_writer::{PrintPdfWriter};
use std::io::Write;

pub mod dom;
pub mod error;
mod fonts;
pub mod geometry;
pub mod page_sizes;
pub mod document_builder;
pub mod print_pdf_writer;

use error::DocumentGenerationError;

pub fn build_pdf_from_dom<W: Write>(
    doc_structure: &dom::DocStructure,
    pdf_doc_writer: W,
) -> Result<W, DocumentGenerationError> {
    let pdf_writer = PrintPdfWriter::new(&doc_structure.document_title, page_sizes::LETTER);

    let mut pdf_builder = DocumentBuilder::new(pdf_writer);

    // pdf_writer.write_line("asdbd").write_line("aosdifjasfodij");

    pdf_builder.into_inner().save(pdf_doc_writer)
}
