use std::collections::HashMap;

use bytes::Bytes;

use crate::error::{PdfGenerationError, UserInputError};

use super::{FontAttributes, FontData, FontId};

pub struct FontFamilyCollection {
    family_name: String,
    pub(crate) fonts_by_attribute: HashMap<FontAttributes, FontData>,
}

impl FontFamilyCollection {
    pub fn new(family_name: &str) -> Self {
        Self {
            family_name: family_name.to_owned(),
            fonts_by_attribute: HashMap::new(),
        }
    }

    pub fn get_font_by_attribute(
        &self,
        attributes: &FontAttributes,
    ) -> Result<&FontData, PdfGenerationError> {
        Ok(self.fonts_by_attribute.get(attributes).ok_or_else(|| {
            UserInputError::FontAttributesNotOnFamily {
                family_name: self.family_name.to_owned(),
                attributes: *attributes,
            }
        })?)
    }

    pub fn add_font(
        &mut self,
        attributes: FontAttributes,
        bytes: Bytes,
    ) -> Result<FontId, PdfGenerationError> {
        let new_font_id = FontId::new();

        if self
            .fonts_by_attribute
            .insert(attributes, FontData::new(new_font_id, bytes))
            .is_some()
        {
            Err(UserInputError::MultipleFontsWithSameAttribute {
                family_name: self.family_name.clone(),
                attributes,
            }
            .into())
        } else {
            Ok(new_font_id)
        }
    }
}
