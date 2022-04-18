use super::dom_node::DomNode;


struct TextNode {

}

impl DomNode for TextNode {
    fn add_child(&mut self, _child: Box<dyn DomNode>) {
      panic!("Cannot add a child to TextNode")
    }

    fn children(&self) -> &[Box<dyn DomNode>] {
        &[]
    }
}