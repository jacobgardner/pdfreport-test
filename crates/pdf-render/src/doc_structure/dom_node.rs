use serde::Deserialize;

use crate::utils::tree_iter::{TreeIterator, TreeNode};

use super::{ImageNode, NodeId, StyledNode, TextNode};

pub use super::HasNodeId;

#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DomNode {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}

impl PartialEq for DomNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_id() == other.node_id()
    }
}

impl HasNodeId for DomNode {
    fn node_id(&self) -> NodeId {
        match self {
            DomNode::Styled(node) => node.node_id(),
            DomNode::Text(node) => node.node_id(),
            DomNode::Image(node) => node.node_id(),
        }
    }
}

impl DomNode {
    pub fn styles(&self) -> &[String] {
        match self {
            DomNode::Styled(node) => &node.styles[..],
            DomNode::Text(node) => &node.styles[..],
            DomNode::Image(node) => &node.styles[..],
        }
    }

    pub fn block_iter(&self) -> TreeIterator<Self> {
        TreeIterator::new(self)
    }
}

impl<T: HasNodeId> From<&T> for NodeId {
    fn from(node: &T) -> Self {
        node.node_id()
    }
}

impl TreeNode for DomNode {
    // This does not return children that are not considered block elements
    //  for the layout engine TextNodes, Strings, etc.
    fn children(&self) -> &[Self] {
        match self {
            DomNode::Styled(node) => &node.children,
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::doc_structure::TextChild;

    use super::*;

    use DomNode::*;

    fn is_matching_node(actual: &DomNode, expected: &str) -> bool {
        actual.styles() == [expected]
    }

    fn is_matching_pair(
        (actual_node, actual_parent): (&DomNode, Option<&DomNode>),
        (expected_node, expected_parent): (&str, Option<&str>),
    ) -> bool {
        is_matching_node(actual_node, expected_node);

        match (actual_parent, expected_parent) {
            (Some(actual), Some(expected)) => is_matching_node(actual, expected),
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn traverses_in_order() {
        let root_node = Styled(StyledNode::with_children(
            vec![Styled(StyledNode::with_children(
                vec![
                    Styled(StyledNode::with_children(
                        vec![Styled(StyledNode::with_children(vec![], &["D"]))],
                        &["C"],
                    )),
                    Styled(StyledNode::with_children(
                        vec![Text(TextNode::with_children(
                            vec![
                                TextChild::Content("Test string".to_owned()),
                                TextChild::TextNode(TextNode::with_children(vec![], &["G"])),
                            ],
                            &["F"],
                        ))],
                        &["E"],
                    )),
                ],
                &["B"],
            ))],
            &["A"],
        ));

        let mut nodes = root_node.block_iter();

        assert!(is_matching_pair(nodes.next().unwrap(), ("A", None)));
        assert!(is_matching_pair(nodes.next().unwrap(), ("B", Some("A"))));
        assert!(is_matching_pair(nodes.next().unwrap(), ("C", Some("B"))));
        assert!(is_matching_pair(nodes.next().unwrap(), ("D", Some("C"))));
        assert!(is_matching_pair(nodes.next().unwrap(), ("E", Some("B"))));
        assert!(is_matching_pair(nodes.next().unwrap(), ("F", Some("E"))));
        assert!(matches!(nodes.next(), None));
    }
}
