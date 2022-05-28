use bytes::Bytes;

use super::{FontAttributes, FontId};

pub struct FontData {
    id: FontId,
    family_name: String,
    attributes: FontAttributes,
    data: Bytes,
}

impl FontData {
    pub(super) fn new(
        id: FontId,
        family_name: String,
        attributes: FontAttributes,
        data: Bytes,
    ) -> Self {
        Self {
            id,
            data,
            family_name,
            attributes,
        }
    }
    
    pub fn family_name(&self) -> &str {
        &self.family_name
    }
    
    pub fn attributes(&self) -> &FontAttributes {
        &self.attributes
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn font_id(&self) -> FontId {
        self.id
    }
}
