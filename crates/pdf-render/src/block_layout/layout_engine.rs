use std::{rc::Rc, collections::HashMap};
use polyhorn_yoga as yoga;

use crate::{
    doc_structure::{DomNode, NodeId}, error::DocumentGenerationError, paragraph_layout::ParagraphLayout,
    stylesheet::Stylesheet,
};

pub struct LayoutNode {}

pub trait LayoutEngine {
    fn build_node_layout(
        &mut self,
        root_node: &DomNode,
        stylesheet: &Stylesheet,
        paragraph_layout: Rc<ParagraphLayout>,
    // TODO:
    // FIXME: DO NOT TIE TRAIT TO YOGA 
    ) -> Result<HashMap<NodeId, yoga::Node>, DocumentGenerationError>;
}
