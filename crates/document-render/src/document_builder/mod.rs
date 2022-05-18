use crate::{error::DocumentGenerationError, fonts::FontId};

pub use self::document_writer::DocumentWriter;

pub struct PdfBuilder<Writer: DocumentWriter> {
    raw_pdf_writer: Writer,
}

mod document_writer;

impl<Writer: DocumentWriter> PdfBuilder<Writer> {
    pub fn new(raw_pdf_writer: Writer) -> Self {
        Self { raw_pdf_writer }
    }

    pub fn write_line(
        &mut self,
        font_id: FontId,
        pdf_line: &str,
    ) -> Result<&mut Self, DocumentGenerationError> {
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

    impl DocumentWriter for MockPdfWriter {
        fn write_line(
            &mut self,
            font_id: FontId,
            pdf_line: &str,
        ) -> Result<&mut MockPdfWriter, DocumentGenerationError> {
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
