use crate::{error::PdfGenerationError, fonts::FontId};

use self::pdf_writer::PdfWriter;

pub struct PdfBuilder<Writer: PdfWriter> {
    raw_pdf_writer: Writer,
}

mod pdf_writer;
pub mod print_pdf_writer;

impl<Writer: PdfWriter> PdfBuilder<Writer> {
    pub fn new(raw_pdf_writer: Writer) -> Self {
        Self { raw_pdf_writer }
    }

    pub fn write_line(
        &mut self,
        font_id: FontId,
        pdf_line: &str,
    ) -> Result<&mut Self, PdfGenerationError> {
        self.raw_pdf_writer.write_line(font_id, pdf_line)?;

        Ok(self)
    }

    pub fn into_inner(self) -> Writer {
        self.raw_pdf_writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPdfWriter {
        title: String,
        lines: Vec<String>,
    }

    impl MockPdfWriter {
        fn new(doc_title: &str) -> Self {
            Self {
                title: doc_title.to_owned(),
                lines: vec![],
            }
        }
    }

    impl PdfWriter for MockPdfWriter {
        fn write_line(
            &mut self,
            font_id: FontId,
            pdf_line: &str,
        ) -> Result<&mut MockPdfWriter, PdfGenerationError> {
            self.lines.push(pdf_line.to_owned());

            Ok(self)
        }
    }

    #[test]
    fn can_write_line() {
        let writer = MockPdfWriter::new("Test Title");

        let mut builder: PdfBuilder<MockPdfWriter> = PdfBuilder::new(writer);

        let f1 = FontId::new();

        // builder.write_line(f1, "Hello").unwrap();

        // let mut output: Vec<u8> = vec![];

        // builder.into_inner().save(output).unwrap();

        // let string_version = std::str::from_utf8(&output).unwrap();

        // assert_eq!(string_version, "Test Title - Hello");
    }
}
