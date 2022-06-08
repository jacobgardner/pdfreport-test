//! This is the raw representation of the DOM that will ultimately
//!  be used throughout the rest of the engine. This includes the stylesheet
//!  definition, the dom hierarchy, and the font definitions.
use serde::Deserialize;

mod dom_node;
mod fonts;
mod has_node_id;
mod image_node;
mod styled_node;
mod text_node;

pub use dom_node::DomNode;
pub use fonts::FontFamilyInfo;
pub use has_node_id::HasNodeId;
pub use image_node::ImageNode;
pub use styled_node::StyledNode;
pub use text_node::{TextChild, TextNode};
use ts_rs::TS;

use crate::{
    stylesheet::{EdgeStyle, Stylesheet},
    utils::unique_id::create_id,
};

#[derive(TS, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, rename_all = "camelCase")]
pub struct DocStructure {
    pub filename: String,
    pub document_title: String,
    pub page_size: String,
    #[serde(default)]
    #[ts(type="EdgeStyle")]
    pub page_margins: EdgeStyle::Unmergeable,
    pub fonts: Vec<FontFamilyInfo>,
    #[ts(type="Style")]
    pub stylesheet: Stylesheet,
    pub root: DomNode,
}

create_id!(NodeId);
