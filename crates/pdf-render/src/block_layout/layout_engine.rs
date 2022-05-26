use std::rc::Rc;

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::ParagraphLayout,
    stylesheet::Stylesheet, values::Pt,
};

/// The absolute position of the node relative to 
/// the top of the PDF document. (This does NOT include pagination)
#[derive(Default)]
pub struct NodeLayout {
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


    fn get_node_layout(&self, node_id: NodeId) -> NodeLayout;
}
