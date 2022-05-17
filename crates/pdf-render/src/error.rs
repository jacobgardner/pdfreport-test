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
}

#[derive(Error, Debug)]
pub enum PdfGenerationError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] InternalServerError),

    #[error("User input error")]
    UserInputError(#[from] UserInputError), // #[error("Could not find style, {style_name}, in stylesheet. Style names are case-sensitive.")]
                                            // UnmatchedStyle { style_name: String },

                                            // #[error("Unable to parse underlying pdf: {source}")]
                                            // MeasurementParseError {
                                            //     #[from]
                                            //     source: MeasurementParseError,
                                            // },

                                            // #[error("Error computing the flex layout: {source}")]
                                            // LayoutComputationError {
                                            //     #[from]
                                            //     source: stretch2::Error,
                                            // },

                                            // #[error("Unable to fetch resource: {source}")]
                                            // ResourceNotFound {
                                            //     #[from]
                                            //     source: reqwest::Error,
                                            // },

                                            // #[error("Referenced unloaded font-family, {font_family}.")]
                                            // FontFamilyNotFound { font_family: String },

                                            // #[error("Referenced unloaded font-style, weight: {font_weight:?} & style: {font_style:?}, for supported font-family, {font_family}.")]
                                            // FontStyleNotFoundForFamily {
                                            //     font_family: String,
                                            //     font_weight: FontWeight,
                                            //     font_style: FontStyle,
                                            // },

                                            // #[error("Cannot parse font-color: {source}")]
                                            // ColorParseError {
                                            //     #[from]
                                            //     source: color_processing::ParseError,
                                            // }
}
