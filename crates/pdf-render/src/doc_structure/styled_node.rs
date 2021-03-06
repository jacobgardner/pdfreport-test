use serde::Deserialize;
use ts_rs::TS;

use super::{has_node_id::HasNodeId, DomNode, NodeId};

#[derive(TS, Clone, Default, Deserialize, Debug)]
#[ts(export)]
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

impl HasNodeId for StyledNode {
    fn node_id(&self) -> NodeId {
        self.node_id
    }
}
