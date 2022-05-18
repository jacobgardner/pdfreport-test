use std::io::Write;

use crate::{
    error::PdfGenerationError,
    fonts::FontId,
    geometry::{Mm, Size},
};

use self::pdf_writer::PdfWriter;

pub struct PdfBuilder<Writer: PdfWriter> {
    raw_pdf_writer: Writer,
}

mod pdf_writer;
pub mod print_pdf_writer;

impl<Writer: PdfWriter> PdfBuilder<Writer> {
    pub fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self {
        let writer = Writer::new(doc_title, page_size);

        Self {
            raw_pdf_writer: writer,
        }
    }

    pub fn write_line(&mut self, font_id: &FontId, pdf_line: &str) -> &mut Self {
        self.raw_pdf_writer.write_line(font_id, pdf_line);
        self
    }

    pub fn save<W: Write>(self, pdf_doc_writer: W) -> Result<W, PdfGenerationError> {
        self.raw_pdf_writer.save(pdf_doc_writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPdfWriter {
        title: String,
        lines: Vec<String>,
    }

    impl PdfWriter for MockPdfWriter {
        fn new(doc_title: &str, page_size: impl Into<Size<Mm>>) -> Self {
            Self {
                title: doc_title.to_owned(),
                lines: vec![],
            }
        }

        fn write_line(&mut self, font_id: &FontId, pdf_line: &str) -> &mut Self {
            self.lines.push(pdf_line.to_owned());

            self
        }

        fn save<W: Write>(self, mut pdf_doc_writer: W) -> Result<W, PdfGenerationError> {
            let lines = self.lines.join(" ");
            let title = self.title;

            let output_string = format!("{title} - {lines}");

            pdf_doc_writer.write(&output_string.as_bytes()).unwrap();
            Ok(pdf_doc_writer)
        }

        fn load_fonts(&mut self, font_collection: &crate::fonts::FontCollection) -> &mut Self {
            todo!()
        }
    }

    #[test]
    fn can_write_line() {
        let mut builder: PdfBuilder<MockPdfWriter> =
            PdfBuilder::new("Test Title", (Mm(10.), Mm(10.)));

        // builder.write_line("Hello");

        let mut output: Vec<u8> = vec![];

        output = builder.save(output).unwrap();

        let string_version = std::str::from_utf8(&output).unwrap();

        assert_eq!(string_version, "Test Title - Hello");
    }
}
