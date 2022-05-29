use super::NodeId;

pub trait HasNodeId {
    fn node_id(&self) -> NodeId;
}
