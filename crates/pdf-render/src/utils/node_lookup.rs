use crate::doc_structure::HasNodeId;
use std::collections::HashMap;

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    stylesheet::{Style, Stylesheet},
};

use super::parent_lookup::ParentLookup;

pub struct NodeLookup<'a> {
    dom_node_lookup: HashMap<NodeId, &'a DomNode>,
    style_lookup: HashMap<NodeId, Style::Unmergeable>,
    parent_lookup: ParentLookup,
}

impl<'a> NodeLookup<'a> {
    pub fn from_root_node(
        root_node: &'a DomNode,
        stylesheet: &Stylesheet,
    ) -> Result<Self, DocumentGenerationError> {
        let mut parent_lookup = ParentLookup::new();
        let mut dom_node_lookup = HashMap::new();
        let mut partially_computed_style: HashMap<NodeId, Style::Mergeable> = HashMap::new();
        let mut style_lookup = HashMap::new();

        for (node, parent) in root_node.block_iter() {
            dom_node_lookup.insert(node.node_id(), node);

            let parent_style = if let Some(parent) = parent {
                partially_computed_style
                    .get(&parent.node_id())
                    .unwrap()
                    .clone()
            } else {
                Default::default()
            };

            let node_style = stylesheet.compute_mergeable_style(&parent_style, node.styles())?;

            style_lookup.insert(
                node.node_id(),
                Style::Unmergeable::default().merge_style(&node_style),
            );
            partially_computed_style.insert(node.node_id(), node_style);

            if let Some(parent) = parent {
                parent_lookup.add_parent(node.node_id(), parent.node_id());
            }
        }

        Ok(Self {
            dom_node_lookup,
            parent_lookup,
            style_lookup,
        })
    }

    pub fn get_style(&self, node: impl Into<NodeId>) -> &Style::Unmergeable {
        self.style_lookup
            .get(&node.into())
            .expect("If it has a NodeId it should exist in the lookup")
    }

    pub fn get_dom_node(&self, node: impl Into<NodeId>) -> &DomNode {
        self.dom_node_lookup
            .get(&node.into())
            .expect("If it has a NodeId it should exist in the lookup")
    }

    pub fn get_parent(&self, node: impl Into<NodeId>) -> Option<&DomNode> {
        self.get_parent_id(node.into())
            .map(|node_id| self.get_dom_node(node_id))
    }

    pub fn get_parent_id(&self, node: impl Into<NodeId>) -> Option<NodeId> {
        self.parent_lookup.get_parent(node.into())
    }

    pub fn get_ancestor_ids(&self, node: impl Into<NodeId>) -> Vec<NodeId> {
        self.parent_lookup.get_ancestors(node.into())
    }

    pub fn get_ancestors(&self, node: impl Into<NodeId>) -> Vec<&DomNode> {
        self.get_ancestor_ids(node.into())
            .into_iter()
            .map(|node_id| self.get_dom_node(node_id))
            .collect()
    }
}
