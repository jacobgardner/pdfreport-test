use serde::Deserialize;

use super::NodeId;

#[derive(Clone, Deserialize, Debug)]
pub struct ImageNode {
    #[serde(skip)]
    pub node_id: NodeId,
    #[serde(default)]
    pub styles: Vec<String>,
    pub content: String,
}
