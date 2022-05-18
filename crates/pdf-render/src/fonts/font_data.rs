use bytes::Bytes;

use super::FontId;

pub struct FontData {
    id: FontId,
    data: Bytes,
}

impl FontData {
    pub(super) fn new(id: FontId, data: Bytes) -> Self {
        Self { id, data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn font_id(&self) -> FontId {
        self.id
    }
}
