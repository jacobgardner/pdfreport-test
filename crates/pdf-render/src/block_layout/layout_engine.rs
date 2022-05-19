use crate::doc_structure::DomNode;




pub struct LayoutNode {}

pub trait LayoutEngine {
  fn build_node_layout(&mut self, root_node: &DomNode) -> ();
}