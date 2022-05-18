use std::collections::HashMap;

use crate::error::{PdfGenerationError, UserInputError};

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

    pub fn lookup_font(
        &self,
        family_name: &str,
        attributes: &FontAttributes,
    ) -> Result<&FontData, PdfGenerationError> {
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
    fn test_font_lookup() {
        let font_collection = FontCollection::new();

        let fid = font_collection
            .lookup_font("Inter", &FontAttributes::default())
            .unwrap()
            .font_id();
    }
}
