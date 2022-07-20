use std::collections::HashMap;

use crate::doc_structure::NodeId;

#[derive(Default)]
pub struct ParentLookup(HashMap<NodeId, NodeId>);

impl ParentLookup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_parent(&mut self, child: NodeId, parent: NodeId) {
        assert!(
            self.0.insert(child, parent).is_none(),
            "This child already has an associated parent."
        );
    }

    pub fn get_parent(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(&node).cloned()
    }

    /// Ancestor ids are returned from deepest node to root node.
    pub fn get_ancestors(&self, mut node: NodeId) -> Vec<NodeId> {
        let mut ancestors = vec![];

        while let Some(parent) = self.get_parent(node) {
            ancestors.push(parent);
            node = parent;
        }

        ancestors
    }
}
