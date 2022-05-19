use serde::Deserialize;

use super::{DomNode, NodeId};

#[derive(Default, Deserialize, Debug)]
pub struct StyledNode {
    #[serde(skip)]
    pub node_id: NodeId,
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}

impl StyledNode {
    pub fn with_children(children: Vec<DomNode>, styles: &[&str]) -> Self {
        Self {
            children,
            styles: styles.iter().map(|&s| s.to_owned()).collect(),
            ..Default::default()
        }
    }
}
