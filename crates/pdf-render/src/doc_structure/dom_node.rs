use serde::Deserialize;

use super::{ImageNode, NodeId, StyledNode, TextNode};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DomNode {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}

pub struct BlockDomNodeIterator<'a> {
    current_stack: Vec<&'a DomNode>,
    // current_node: &'a DomNode
}

impl DomNode {
    pub fn styles(&self) -> &[String] {
        match self {
            DomNode::Styled(node) => &node.styles[..],
            DomNode::Text(node) => &node.styles[..],
            DomNode::Image(node) => &node.styles[..],
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            DomNode::Styled(node) => node.node_id,
            DomNode::Text(node) => node.node_id,
            DomNode::Image(node) => node.node_id,
        }
    }

    // This does not return children that are not considered block elements
    //  for the layout engine TextNodes, Strings, etc.
    pub fn block_children(&self) -> &[DomNode] {
        match self {
            DomNode::Styled(node) => &node.children,
            _ => &[],
        }
    }

    pub fn sibling(&self, target_node: &DomNode) -> Option<&DomNode> {
        // This should be safe from panic as well because the
        // current_node MUST have come from the parent
        let current_index = self
            .block_children()
            .iter()
            .position(|node| std::ptr::eq(node, target_node))
            .unwrap();

        self.block_children().get(current_index + 1)
    }

    pub fn has_block_children(&self) -> bool {
        !self.block_children().is_empty()
    }

    pub fn block_iter(&self) -> BlockDomNodeIterator {
        BlockDomNodeIterator {
            current_stack: vec![&self],
        }
    }
}

impl<'a> Iterator for BlockDomNodeIterator<'a> {
    type Item = (&'a DomNode, Option<&'a DomNode>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_stack.is_empty() {
            return None;
        }

        let current_index = self.current_stack.len() - 1;

        let current_node = self.current_stack[current_index];

        let parent_node = if current_index > 0 {
            Some(self.current_stack[current_index - 1])
        } else {
            None
        };

        if current_node.has_block_children() {
            self.current_stack.push(&current_node.block_children()[0]);
        } else {
            let mut found_node = false;

            while self.current_stack.len() > 1 {
                // These two lines are safe from panic because we just checked
                // the stack has at least 2 elements
                let current_node = self.current_stack.pop().unwrap();
                let parent_node = *self.current_stack.last().unwrap();

                let sibling_node = parent_node.sibling(current_node);

                if let Some(sibling_node) = sibling_node {
                    self.current_stack.push(sibling_node);
                    found_node = true;
                    break;
                }
            }

            if !found_node {
                self.current_stack.clear();
            }
        }

        /*
              A
             / \
            B   C
          / | \   \
         D  E  F   G
        */

        Some((current_node, parent_node))
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
