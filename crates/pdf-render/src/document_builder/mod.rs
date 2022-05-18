use crate::{error::DocumentGenerationError, rich_text::RichTextLine};

pub use self::document_writer::DocumentWriter;

pub struct DocumentBuilder<Writer: DocumentWriter> {
    raw_document_writer: Writer,
}

mod document_writer;

impl<Writer: DocumentWriter> DocumentBuilder<Writer> {
    pub fn new(raw_document_writer: Writer) -> Self {
        Self {
            raw_document_writer,
        }
    }

    pub fn write_line(&mut self, line: RichTextLine) -> Result<&mut Self, DocumentGenerationError> {
        self.raw_document_writer.write_line(line)?;

        Ok(self)
    }

    pub fn into_inner(self) -> Writer {
        self.raw_document_writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDocWriter {
        title: String,
        lines: Vec<String>,
    }

    impl MockDocWriter {
        fn new(doc_title: &str) -> Self {
            Self {
                title: doc_title.to_owned(),
                lines: vec![],
            }
        }
    }

    impl DocumentWriter for MockDocWriter {
        fn write_line(
            &mut self,
            line: RichTextLine,
        ) -> Result<&mut MockDocWriter, DocumentGenerationError> {
            self.lines.push(
                line.0
                    .iter()
                    .map(|span| span.text.clone())
                    .collect::<Vec<String>>()
                    .join(" "),
            );

            Ok(self)
        }
    }

    #[test]
    fn can_write_line() {
        let writer = MockDocWriter::new("Test Title");

        let mut builder: DocumentBuilder<MockDocWriter> = DocumentBuilder::new(writer);

        let f1 = FontId::new();

        // builder.write_line(f1, "Hello").unwrap();

        // let mut output: Vec<u8> = vec![];

        // builder.into_inner().save(output).unwrap();

        // let string_version = std::str::from_utf8(&output).unwrap();

        // assert_eq!(string_version, "Test Title - Hello");
    }
}
