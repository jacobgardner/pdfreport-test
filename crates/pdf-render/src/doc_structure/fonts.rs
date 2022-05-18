use serde::Deserialize;

use crate::fonts::{FontWeight, FontStyle, FontAttributes};


#[derive(Debug, Deserialize)]
pub struct FontInfo {
    pub source: String,
  
    #[serde(flatten)]
    pub attributes: FontAttributes,

    // #[serde(default)]
    // pub weight: FontWeight,
    // #[serde(default)]
    // pub style: FontStyle,
}

#[derive(Debug, Deserialize)]
pub struct FontFamilyInfo {
    pub family_name: String,
    pub fonts: Vec<FontInfo>,
}