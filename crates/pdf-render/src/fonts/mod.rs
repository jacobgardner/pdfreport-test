//! Handles loading and prepping fonts for layout and embedding within the
//!  PDF

mod attributes;
mod font_collection;
mod font_data;
mod font_family_collection;
mod font_id;

pub use attributes::{FontAttributes, FontSlant, FontWeight};
pub use font_collection::FontCollection;
pub use font_data::FontData;
pub use font_family_collection::FontFamilyCollection;
pub use font_id::FontId;
