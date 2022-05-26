use std::rc::Rc;

use crate::{
    doc_structure::NodeId,
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, RenderedTextBlock},
    rich_text::RichText,
};

pub(super) struct NodeContext {
    pub rich_text: RichText,
    pub paragraph_layout: Rc<ParagraphLayout>,
    pub text_block: Option<RenderedTextBlock>,
    pub node_id: NodeId,
    pub calculate_error: Option<DocumentGenerationError>,
}
