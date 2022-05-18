use std::io::Write;

use crate::{geometry::{Size, Mm}, error::PdfGenerationError, fonts::{FontId, FontCollection}};

pub trait PdfWriter {
    fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self;
    fn write_line(&mut self, font: &FontId, pdf_line: &str) -> &mut Self;
    fn save<W: Write>(self, pdf_doc_writer: W) -> Result<W, PdfGenerationError>;
    

    fn load_fonts(&mut self, font_collection: &FontCollection) -> &mut Self;
}


