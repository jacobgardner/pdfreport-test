use thiserror::Error;

use crate::fonts::FontAttributes;

#[derive(Error, Debug)]
pub enum InternalServerError {
    #[error("Write PDF Error")]
    WritePdfError(#[from] std::io::Error),

    #[error("Error loading font: {family_name} w/ attributes: {attributes:?}")]
    LoadFontError {
        source: Box<dyn std::error::Error>,
        family_name: String,
        attributes: FontAttributes,
    },

    #[error("FontId does not match any loaded font.")]
    FontIdNotLoaded,

    #[error("Unable to associate font data with skia typeface for family, {family_name} w/ attributes: {attributes:?}")]
    SkiaTypefaceFailure {
        family_name: String,
        attributes: FontAttributes,
    },

    #[error("Font face not loaded into text layout engine: {family_name}")]
    FontFamilyNotRegisteredForLayoutEngine { family_name: String },
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
    NonUniqueFontAttribute {
        family_name: String,
        attributes: FontAttributes,
    },

    #[error("Font family, {family_name}, was registered more than once")]
    NonUniqueFontFamily { family_name: String },

    #[error("Cannot parse font-color: {source}")]
    ColorParseError {
        #[from]
        source: color_processing::ParseError,
    },

    #[error("Style name does not exist on stylesheet: {style_name}")]
    StyleDoesNotExist { style_name: String },
}

#[derive(Error, Debug)]
pub enum DocumentGenerationError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] InternalServerError),

    #[error("User input error")]
    UserInputError(#[from] UserInputError),
}
