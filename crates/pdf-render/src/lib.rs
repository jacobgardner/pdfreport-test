use std::io::{BufWriter, Write};

pub mod dom;
mod error;

pub use error::BadPdfLayout;

pub fn build_pdf_from_dom<W: Write>(
    pdf_dom: &dom::PdfDom,
    writable_stream: &W,
) -> Result<(), BadPdfLayout> {
    Err(BadPdfLayout::UnknownError)
}
