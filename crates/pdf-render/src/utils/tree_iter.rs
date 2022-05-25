pub trait TreeNode: Sized {
    fn children(&self) -> &[Self];
    fn has_children(&self) -> bool {
        !self.children().is_empty()
    }
    fn sibling(&self, target_node: &Self) -> Option<&Self> {
        // This should be safe from panic as well because the
        // current_node MUST have come from the parent
        let current_index = self
            .children()
            .iter()
            .position(|node| std::ptr::eq(node, target_node))
            .unwrap();

        self.children().get(current_index + 1)
    }
}

pub struct TreeIterator<'a, T: TreeNode> {
    current_stack: Vec<&'a T>,
}

impl<'a, T: TreeNode> TreeIterator<'a, T> {
    pub fn new(root_node: &'a T) -> Self {
        Self {
            current_stack: vec![root_node],
        }
    }
}

impl<'a, T: TreeNode> Iterator for TreeIterator<'a, T> {
    type Item = (&'a T, Option<&'a T>);

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

        if current_node.has_children() {
            self.current_stack.push(&current_node.children()[0]);
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
