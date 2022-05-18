// use document_builder::{print_pdf_writer::PrintPdfWriter, PdfBuilder};
use std::io::Write;

pub mod dom;
pub mod error;
pub mod fonts;
pub mod geometry;
pub mod page_sizes;
pub mod document_builder;

// pub fn build_pdf_from_dom<W: Write>(
//     pdf_dom: &dom::PdfDom,
//     pdf_doc_writer: W,
// ) -> Result<W, DocumentGenerationError> {
//     let pdf_writer = PrintPdfWriter::new(&pdf_dom.document_title, page_sizes::LETTER);

//     let mut pdf_builder = PdfBuilder::new(pdf_writer);

//     // pdf_writer.write_line("asdbd").write_line("aosdifjasfodij");

//     pdf_builder.into_inner().save(pdf_doc_writer)
// }
