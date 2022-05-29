//! This is the raw representation of the DOM that will ultimately
//!  be used throughout the rest of the engine. This includes the stylesheet
//!  definition, the dom hierarchy, and the font definitions.
use serde::Deserialize;

mod dom_node;
mod fonts;
mod image_node;
mod styled_node;
mod text_node;
mod has_node_id;

pub use dom_node::DomNode;
pub use fonts::FontFamilyInfo;
pub use image_node::ImageNode;
pub use styled_node::StyledNode;
pub use text_node::{TextChild, TextNode};
pub use has_node_id::HasNodeId;

use crate::{stylesheet::Stylesheet, utils::unique_id::create_id};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocStructure {
    pub filename: String,
    pub document_title: String,

    pub fonts: Vec<FontFamilyInfo>,
    pub stylesheet: Stylesheet,
    pub root: DomNode,
}

create_id!(NodeId);
