use serde::Deserialize;

use crate::utils::tree_iter::{TreeIterator, TreeNode};

use super::NodeId;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct TextNode {
    #[serde(skip)]
    pub node_id: NodeId,
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
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
    pub fn iter<'a>(&'a self) -> TreeIterator<'a, Self> {
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
