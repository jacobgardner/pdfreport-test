use polyhorn_yoga::Layout;

use crate::{
    block_layout::layout_engine::NodeLayout,
    doc_structure::{DomNode, HasNodeId},
    stylesheet::PageBreakRule,
    utils::tree_iter::NodeVisitor,
    values::Pt,
};

use super::{draw_cursor::DrawCursor, PaginatedLayoutEngine};

pub(super) struct LayoutVisitor<'a, 'b> {
    pub paginated_layout_engine: &'a mut PaginatedLayoutEngine<'b>,
    prior_sibling_layout: NodeLayout,
    draw_cursor: DrawCursor,
}

impl<'a, 'b> LayoutVisitor<'a, 'b> {
    pub fn new(paginated_layout_engine: &'a mut PaginatedLayoutEngine<'b>) -> Self {
        Self {
            paginated_layout_engine,
            prior_sibling_layout: NodeLayout::default(),
            draw_cursor: DrawCursor {
                y_offset: Pt(0.),
                page_index: 0,
                page_break_debt: Pt(0.),
            },
        }
    }
}

impl<'a, 'b> NodeVisitor<DomNode> for LayoutVisitor<'a, 'b> {
    fn node_enter(
        &mut self,
        node: &DomNode,
        _parent: Option<&DomNode>,
    ) -> Result<(), crate::error::DocumentGenerationError> {
        let node_layout = self
            .paginated_layout_engine
            .layout_engine
            .get_node_layout(node.node_id());

        let cursor_offset =
            node_layout.top - self.prior_sibling_layout.top - self.draw_cursor.page_break_debt;

        self.draw_cursor.y_offset += cursor_offset;

        self.paginated_layout_engine.draw_paginated_node(
            &mut self.draw_cursor,
            node_layout.clone(),
            node,
        )?;

        self.prior_sibling_layout = node_layout;

        Ok(())
    }

    fn node_leave(
        &mut self,
        node: &DomNode,
        _parent: Option<&DomNode>,
    ) -> Result<(), crate::error::DocumentGenerationError> {
        let style = self
            .paginated_layout_engine
            .node_lookup
            .get_style(node.node_id());

        if style.break_after == PageBreakRule::Always {
            self.draw_cursor.y_offset += self.paginated_layout_engine.page_height;
        }
        Ok(())
    }
}
