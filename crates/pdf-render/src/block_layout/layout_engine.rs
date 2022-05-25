use crate::{doc_structure::DomNode, stylesheet::Stylesheet, error::DocumentGenerationError};




pub struct LayoutNode {}

pub trait LayoutEngine {
  fn build_node_layout(&mut self, root_node: &DomNode, stylesheet: &Stylesheet) -> Result<(), DocumentGenerationError>;
}