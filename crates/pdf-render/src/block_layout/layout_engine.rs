use std::rc::Rc;

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::ParagraphLayout,
    stylesheet::Stylesheet, values::Pt,
};

#[derive(Default)]
pub struct LayoutNode {
    pub left: Pt,
    pub right: Pt,
    pub top: Pt,
    pub width: Pt,
    pub height: Pt,

}

pub trait LayoutEngine {
    fn build_node_layout(
        &mut self,
        page_width: Pt,
        root_node: &DomNode,
        stylesheet: &Stylesheet,
        paragraph_layout: Rc<ParagraphLayout>,
    ) -> Result<(), DocumentGenerationError>;


    /// Returns the absolute positioning of the node
    fn get_node_layout(&self, node_id: NodeId) -> LayoutNode;
}
