use serde::Deserialize;

use crate::fonts::FontAttributes;

#[derive(Debug, Deserialize)]
pub struct FontInfo {
    pub source: String,

    #[serde(flatten)]
    pub attributes: FontAttributes,
}

#[derive(Debug, Deserialize)]
pub struct FontFamilyInfo {
    pub family_name: String,
    pub fonts: Vec<FontInfo>,
}
