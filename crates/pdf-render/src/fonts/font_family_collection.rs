use std::collections::HashMap;

use bytes::Bytes;

use crate::error::{DocumentGenerationError, UserInputError};

use super::{FontAttributes, FontData, FontId};

pub struct FontFamilyCollection {
    family_name: String,
    fonts_by_attribute: HashMap<FontAttributes, FontData>,
}

impl AsRef<HashMap<FontAttributes, FontData>> for FontFamilyCollection {
    fn as_ref(&self) -> &HashMap<FontAttributes, FontData> {
        &self.fonts_by_attribute
    }
}

impl FontFamilyCollection {
    pub fn new(family_name: &str) -> Self {
        Self {
            family_name: family_name.to_owned(),
            fonts_by_attribute: HashMap::new(),
        }
    }

    pub fn family_name(&self) -> &String {
        &self.family_name
    }

    pub fn get_font(&self, font_id: FontId) -> Option<&FontData> {
        self.fonts_by_attribute
            .iter()
            .find(|&(_, font)| font.font_id() == font_id)
            .map(|(_, data)| data)
    }

    pub fn get_font_by_attribute(
        &self,
        attributes: &FontAttributes,
    ) -> Result<&FontData, DocumentGenerationError> {
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
    ) -> Result<FontId, DocumentGenerationError> {
        let new_font_id = FontId::new();

        if self
            .fonts_by_attribute
            .insert(attributes, FontData::new(new_font_id, bytes))
            .is_some()
        {
            Err(UserInputError::NonUniqueFontAttribute {
                family_name: self.family_name.clone(),
                attributes,
            }
            .into())
        } else {
            Ok(new_font_id)
        }
    }
}
