use document_render::{
    error::{DocumentGenerationError, InternalServerError},
    fonts::FontAttributes,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfAssembleError {
    #[error("Write PDF Error")]
    WritePdfError(#[from] std::io::Error),

    #[error("Error loading font: {family_name} w/ attributes: {attributes:?}")]
    LoadFontError {
        source: Box<dyn std::error::Error>,
        family_name: String,
        attributes: FontAttributes,
    },
}

impl From<PdfAssembleError> for DocumentGenerationError {
    fn from(pdf_error: PdfAssembleError) -> Self {
        DocumentGenerationError::InternalServerError(InternalServerError::DocumentWriterError {
            source: Box::new(pdf_error),
            document_type: "PDF".to_owned(),
        })
    }
}
