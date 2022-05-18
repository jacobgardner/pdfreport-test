use serde::Deserialize;

mod dom_node;
mod text_node;
mod styled_node;
mod image_node;
mod fonts;

pub use dom_node::DomNode;
pub use styled_node::StyledNode;
pub use image_node::ImageNode;
pub use text_node::{TextChild, TextNode};
pub use fonts::FontFamilyInfo;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocStructure {
    pub filename: String,
    pub document_title: String,

    pub fonts: Vec<FontFamilyInfo>,
    // pub styles: HashMap<String, MergeableStyle>,
    pub root: DomNode,
}
