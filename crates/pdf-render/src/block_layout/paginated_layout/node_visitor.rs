use crate::{
    block_layout::layout_engine::NodeLayout,
    doc_structure::{DomNode, HasNodeId},
    error::DocumentGenerationError,
    stylesheet::Style,
    utils::tree_iter::{NodeVisitor, TreeNode},
    values::{Point, Pt},
};

use super::{DebugCursor, DrawCursor, PaginatedLayoutEngine};

pub struct PaginationVisitor<'a, 'b> {
    pub pagination_engine: &'a mut PaginatedLayoutEngine<'b>,
    pub draw_cursor: DrawCursor,
    pub prior_sibling_layout: NodeLayout,
    // This should probably be a reference instead so we're not cloning
    pub prior_sibling_style: Style::Unmergeable,
    pub depth: usize,
}

impl<'a, 'b> PaginationVisitor<'a, 'b> {
    pub fn new(pagination_engine: &'a mut PaginatedLayoutEngine<'b>) -> Self {
        Self {
            pagination_engine,
            draw_cursor: DrawCursor {
                y_offset: Pt(0.),
                page_index: 0,
                debt: Pt(0.)
            },
            prior_sibling_layout: Default::default(),
            prior_sibling_style: Default::default(),
            depth: 0,
        }
    }
}

impl<'a, 'b> NodeVisitor<DomNode> for PaginationVisitor<'a, 'b> {
    fn node_enter(
        &mut self,
        node: &DomNode,
        parent: Option<&DomNode>,
    ) -> Result<(), DocumentGenerationError> {
        let node_layout = self
            .pagination_engine
            .layout_engine
            .get_node_layout(node.node_id());

        let cursor_offset = node_layout.top - self.prior_sibling_layout.top - self.draw_cursor.debt;

        self.draw_cursor.y_offset += cursor_offset;

        let style = self.pagination_engine.node_lookup.get_style(node);

        if let Some(parent) = parent {
            let parent_layout = self
                .pagination_engine
                .layout_engine
                .get_node_layout(parent.node_id());

            let offset = if parent.first_child() == node {
                self.depth += 1;

                // Parent to child movement
                (node_layout.top - style.margin.top)
                    - (parent_layout.top - self.prior_sibling_style.margin.top)
            } else {
                (node_layout.top - style.margin.top)
                    - (self.prior_sibling_layout.bottom() + self.prior_sibling_style.margin.bottom)
            };
        }

        self.pagination_engine
            .draw_paginated_node(&mut self.draw_cursor, &node_layout, node)?;

        self.prior_sibling_layout = node_layout;
        self.prior_sibling_style = style.clone();

        Ok(())
    }

    fn node_visit(
        &mut self,
        _node: &DomNode,
        _parent: Option<&DomNode>,
    ) -> Result<(), DocumentGenerationError> {
        Ok(())
    }

    fn node_leave(
        &mut self,
        _node: &DomNode,
        _parent: Option<&DomNode>,
    ) -> Result<(), DocumentGenerationError> {
        Ok(())
    }
}
