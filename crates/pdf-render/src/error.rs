use thiserror::Error;

use crate::fonts::FontAttributes;

// use crate::{
//     rich_text::{FontStyle, FontWeight},
//     units::MeasurementParseError,
// };

#[derive(Error, Debug)]
pub enum InternalServerError {
    #[error("Write Error")]
    WriteError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum UserInputError {
    #[error("Font family has not been loaded: {family_name}")]
    FontFamilyNotLoaded { family_name: String },

    #[error("Font family, {family_name}, does not have attributes: {attributes:?}")]
    FontAttributesNotOnFamily {
        family_name: String,
        attributes: FontAttributes,
    },

    #[error("Font family, {family_name}, was registered with the same attribute multiple times: {attributes:?}")]
    MultipleFontsWithSameAttribute {
        family_name: String,
        attributes: FontAttributes,
    },
}

#[derive(Error, Debug)]
pub enum PdfGenerationError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] InternalServerError),

    #[error("User input error")]
    UserInputError(#[from] UserInputError),
}
