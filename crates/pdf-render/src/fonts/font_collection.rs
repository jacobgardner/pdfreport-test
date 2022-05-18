use std::collections::HashMap;

use crate::error::{DocumentGenerationError, UserInputError};

use super::{FontAttributes, FontData, FontFamilyCollection};

#[derive(Default)]
pub struct FontCollection {
    pub families: HashMap<String, FontFamilyCollection>,
}

impl FontCollection {
    pub fn new() -> Self {
        Self {
            families: HashMap::new(),
        }
    }

    pub fn add_family(
        &mut self,
        family_collection: FontFamilyCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let family_name = family_collection.family_name.clone();

        if self
            .families
            .insert(family_name.clone(), family_collection)
            .is_some()
        {
            Err(UserInputError::NonUniqueFontFamily { family_name }.into())
        } else {
            Ok(self)
        }
    }

    pub fn lookup_font(
        &self,
        family_name: &str,
        attributes: &FontAttributes,
    ) -> Result<&FontData, DocumentGenerationError> {
        let family_collection =
            self.families
                .get(family_name)
                .ok_or_else(|| UserInputError::FontFamilyNotLoaded {
                    family_name: family_name.to_owned(),
                })?;

        family_collection.get_font_by_attribute(attributes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "TODO: Not yet working"]
    fn test_font_lookup() {
        let font_collection = FontCollection::new();

        let fid = font_collection
            .lookup_font("Inter", &FontAttributes::default())
            .unwrap()
            .font_id();
    }

    #[test]
    fn double_load_error() {
        let font_family1 = FontFamilyCollection::new("Inter");
        let font_family2 = FontFamilyCollection::new("Inter");

        let mut font_collection = FontCollection::new();
        let double_add_result = font_collection
            .add_family(font_family1)
            .unwrap()
            .add_family(font_family2);

        assert!(matches!(
            double_add_result,
            Err(DocumentGenerationError::UserInputError(
                UserInputError::NonUniqueFontFamily { .. }
            ))
        ));
    }
}
