use std::collections::HashMap;

use crate::error::{PdfGenerationError, UserInputError};

use self::attributes::{FontStyle, FontWeight};

pub mod attributes;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontAttributes {
    weight: FontWeight,
    style: FontStyle, // Italic/Normal/Oblique
}

pub struct FontData {
    pub font_id: FontId,
}

impl FontData {
    pub fn as_bytes(&self) -> &[u8] {
        todo!();
    }
}

pub struct FontFamilyCollection {
    pub fonts_by_attribute: HashMap<FontAttributes, FontData>,
}

pub struct FontCollection {
    pub families: HashMap<String, FontFamilyCollection>,
}

impl FontCollection {
    // TODO: Rename
    pub fn lookup_id_by_family_and_attributes(
        &self,
        family_name: &str,
        attributes: &FontAttributes,
    ) -> Result<FontId, PdfGenerationError> {
        let family_collection =
            self.families
                .get(family_name)
                .ok_or_else(|| UserInputError::FontFamilyNotLoaded {
                    family_name: family_name.to_owned(),
                })?;

        let font_data = family_collection
            .fonts_by_attribute
            .get(attributes)
            .ok_or_else(|| UserInputError::FontAttributesNotOnFamily {
                family_name: family_name.to_owned(),
                attributes: attributes.clone(),
            })?;

        Ok(font_data.font_id)
    }
}
