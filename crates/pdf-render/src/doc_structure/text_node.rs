use serde::Deserialize;
use ts_rs::TS;

use crate::utils::tree_iter::{TreeIterator, TreeNode};

use super::{has_node_id::HasNodeId, NodeId};

#[derive(TS, Default, Deserialize, Debug, Clone)]
#[ts(export)]
pub struct TextNode {
    #[serde(skip)]
    pub node_id: NodeId,
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

#[derive(TS, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[ts(export)]
pub enum TextChild {
    Content(String),
    TextNode(TextNode),
}

impl TextNode {
    pub fn with_children(children: Vec<TextChild>, styles: &[&str]) -> Self {
        Self {
            children,
            styles: styles.iter().map(|&s| s.to_owned()).collect(),
            ..Default::default()
        }
    }

    pub fn styles(&self) -> &[String] {
        &self.styles[..]
    }
}

impl TextChild {
    pub fn iter(&self) -> TreeIterator<Self> {
        TreeIterator::new(self)
    }
}

impl TreeNode for TextChild {
    fn children(&self) -> &[Self] {
        match self {
            TextChild::Content(_) => &[],
            TextChild::TextNode(node) => &node.children,
        }
    }
}

impl HasNodeId for TextNode {
    fn node_id(&self) -> NodeId {
        self.node_id
    }
}
