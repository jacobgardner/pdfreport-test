use std::collections::HashMap;

use crate::error::{DocumentGenerationError, UserInputError};

use super::{FontAttributes, FontData, FontFamilyCollection, FontId};

#[derive(Default)]
pub struct FontCollection {
    families: HashMap<String, FontFamilyCollection>,
    default_font: Option<FontId>,
}

impl AsRef<HashMap<String, FontFamilyCollection>> for FontCollection {
    fn as_ref(&self) -> &HashMap<String, FontFamilyCollection> {
        &self.families
    }
}

impl FontCollection {
    pub fn new() -> Self {
        Self {
            families: HashMap::new(),
            default_font: None,
        }
    }

    pub fn default_font(&self) -> Option<&FontData> {
        self.default_font.map(|font_id| self.get_font(font_id))
    }

    pub fn set_default_font(&mut self, default_font_id: FontId) {
        self.default_font = Some(default_font_id);
    }

    pub fn get_font(&self, font_id: FontId) -> &FontData {
        self.families
            .iter()
            .flat_map(|(_, font_family)| font_family.as_ref().iter())
            .find(|&(_, font)| font.font_id() == font_id)
            .map(|(_, data)| data)
            .expect("If we have a font_id it should be guaranteed to exist")
    }

    pub fn add_family(
        &mut self,
        family_collection: FontFamilyCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let family_name = family_collection.family_name().clone();

        if self.default_font.is_none() {
            let font_data = family_collection.get_font_by_attribute(&FontAttributes::default());

            if let Ok(font_data) = font_data {
                self.set_default_font(font_data.font_id());
            }
        }

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
    use bytes::Bytes;

    use crate::fonts::FontSlant;

    use super::*;

    #[test]
    fn test_font_lookup() {
        let mut font_collection = FontCollection::new();
        let mut family1 = FontFamilyCollection::new("Inter");

        let fid1 = family1
            .add_font(FontAttributes::default(), Bytes::from("1"))
            .unwrap();
        let fid2 = family1
            .add_font(
                FontAttributes {
                    style: FontSlant::Italic,
                    ..Default::default()
                },
                Bytes::from("2"),
            )
            .unwrap();

        font_collection.add_family(family1).unwrap();

        assert_eq!(
            font_collection
                .lookup_font("Inter", &FontAttributes::default())
                .unwrap()
                .font_id(),
            fid1
        );

        assert_eq!(
            font_collection
                .lookup_font(
                    "Inter",
                    &FontAttributes {
                        style: FontSlant::Italic,
                        ..Default::default()
                    }
                )
                .unwrap()
                .font_id(),
            fid2
        );
    }

    #[test]
    fn lookup_font_by_key() {
        let mut font_collection = FontCollection::new();
        let mut family1 = FontFamilyCollection::new("Inter");

        let fid1 = family1
            .add_font(FontAttributes::default(), Bytes::from("1"))
            .unwrap();
        let fid2 = family1
            .add_font(
                FontAttributes {
                    style: FontSlant::Italic,
                    ..Default::default()
                },
                Bytes::from("2"),
            )
            .unwrap();

        font_collection.add_family(family1).unwrap();

        assert_eq!(font_collection.get_font(fid2).as_bytes(), b"2");
        assert_eq!(font_collection.get_font(fid1).as_bytes(), b"1");
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
