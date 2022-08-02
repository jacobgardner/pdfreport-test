use serde::Deserialize;
use ts_rs::TS;

use crate::values::Pt;

use super::{has_node_id::HasNodeId, NodeId};

#[derive(TS, Clone, Deserialize, Debug)]
#[ts(export)]
pub struct ImageNode {
    #[serde(skip)]
    pub node_id: NodeId,
    #[serde(default)]
    pub styles: Vec<String>,
    pub content: String,
    pub width: Pt,
    pub height: Pt,
}

impl HasNodeId for ImageNode {
    fn node_id(&self) -> NodeId {
        self.node_id
    }
}
