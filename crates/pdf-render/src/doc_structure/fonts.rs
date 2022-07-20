use serde::Deserialize;
use ts_rs::TS;

use crate::fonts::FontAttributes;

#[derive(TS, Debug, Deserialize)]
#[ts(export, rename_all = "camelCase")]
pub struct FontInfo {
    pub source: String,

    #[serde(flatten)]
    pub attributes: FontAttributes,
}

#[derive(TS, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[ts(export, rename_all = "camelCase")]
pub struct FontFamilyInfo {
    pub family_name: String,
    pub fonts: Vec<FontInfo>,
}
