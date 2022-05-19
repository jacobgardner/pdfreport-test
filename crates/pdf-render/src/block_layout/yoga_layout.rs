use crate::doc_structure::DomNode;

use super::layout_engine::LayoutEngine;

use polyhorn_yoga as yoga;

pub struct YogaLayout {}

impl LayoutEngine for YogaLayout {
    fn build_node_layout(&mut self, root_node: &DomNode) -> () {
        for (node, parent) in root_node.block_iter() {
            let layout_node = yoga::Node::from(node);
        }

        todo!()
    }
}

impl From<&DomNode> for yoga::Node {
    fn from(node: &DomNode) -> Self {
        let layout_node = yoga::Node::new();

        layout_node
    }
}
