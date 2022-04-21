// TODO: Eventually we would want these fonts to be specified externally

use std::{collections::HashMap, rc::Rc};

use bytes::Bytes;

use crate::{
    dom::FontInfo,
    error::BadPdfLayout,
    resource_cache::ResourceCache,
    rich_text::{FontStyle, FontWeight},
};

type FamilyName = String;

#[derive(Debug)]
pub struct FontData {
    pub bytes: Rc<Bytes>,
    weight: FontWeight,
    style: FontStyle,
}

impl FontData {
    pub async fn from_font_info(
        resource_cache: &mut ResourceCache,
        font_info: &FontInfo,
    ) -> Result<Self, BadPdfLayout> {
        let font_bytes = resource_cache.get(&font_info.source).await?;

        Ok(Self {
            bytes: font_bytes,
            weight: font_info.weight,
            style: font_info.style,
        })
    }
}

#[derive(Debug)]
pub struct FontFamily {
    pub family_name: FamilyName,
    pub fonts: Vec<FontData>,
}

#[derive(Debug)]
pub struct FontManager {
    pub families: HashMap<FamilyName, FontFamily>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            families: HashMap::new(),
        }
    }
}

impl FontFamily {
    pub fn with_font_stack<T: Into<String>>(family_name: T, fonts: Vec<FontData>) -> FontFamily {
        Self {
            family_name: family_name.into(),
            fonts,
        }
    }
}

pub struct FontLookup<'a> {
    pub family_name: &'a str,
    pub weight: FontWeight,
    pub style: FontStyle,
}
