use skia_safe::{
    textlayout::{FontCollection, TypefaceFontProvider},
    Data, FontMgr, Typeface,
};
use tracing::instrument;

use crate::fonts::FONTS;

pub(crate) struct TextLayout {
    // TODO: Remove pub once everything is encapsulated
    pub font_collection: FontCollection,
    pub typeface: Typeface,
}

impl TextLayout {
    #[instrument(name = "Initialize text layout engine")]
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();

        let mut tfp = TypefaceFontProvider::new();

        let mut typeface: Option<Typeface> = None;

        for font in FONTS {
            // Safe because all the font data is 'static
            // They probably could have enforced this with a type to be safe...
            unsafe {
                let d = Data::new_bytes(font);
                let t = Typeface::from_data(d, None);
                typeface = Some(t.clone().unwrap());
                tfp.register_typeface(t.unwrap(), Some("Inter"));
            }
        }

        let manager = FontMgr::from(tfp);

        font_collection.set_asset_font_manager(manager);
        font_collection.disable_font_fallback();

        Self { font_collection, typeface: typeface.unwrap() }
    }
}
