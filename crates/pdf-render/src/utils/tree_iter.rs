use crate::error::DocumentGenerationError;

pub trait TreeNode: Sized {
    fn children(&self) -> &[Self];
    fn has_children(&self) -> bool {
        !self.children().is_empty()
    }

    fn first_child(&self) -> &Self {
        &self.children()[0]
    }

    fn last_child(&self) -> &Self {
        &self.children()[self.children().len() - 1]
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

    fn visit_nodes(
        &self,
        visitor: &mut dyn NodeVisitor<Self>,
        parent: Option<&Self>,
    ) -> Result<(), DocumentGenerationError> {
        visitor.node_enter(self, parent)?;
        visitor.node_visit(self, parent)?;

        for child in self.children() {
            child.visit_nodes(visitor, Some(self))?;
        }

        visitor.node_leave(self, parent)?;

        Ok(())
    }
}

//type VisitorFn<T> = Fn(&T, Option<&T>) -> Result<(), DocumentGenerationError>;

pub trait NodeVisitor<T> {
    fn node_enter(
        &mut self,
        _node: &T,
        _parent: Option<&T>,
    ) -> Result<(), DocumentGenerationError> {
        Ok(())
    }
    fn node_visit(
        &mut self,
        _node: &T,
        _parent: Option<&T>,
    ) -> Result<(), DocumentGenerationError> {
        Ok(())
    }
    fn node_leave(
        &mut self,
        _node: &T,
        _parent: Option<&T>,
    ) -> Result<(), DocumentGenerationError> {
        Ok(())
    }
    // pub node_enter: Option<Enter>,
    // pub node_visit: Option<Visit>,
    // pub node_leave: Option<Leave>,
    // pub _node_type: PhantomData<T>,
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
