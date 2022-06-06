use std::collections::HashMap;
mod draw_cursor;
mod paginated_node;

use draw_cursor::DrawCursor;

pub use paginated_node::{DrawableContainerNode, DrawableNode, DrawableTextNode, PaginatedNode};

use crate::{
    doc_structure::{DomNode, HasNodeId, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, ParagraphStyle, RenderedTextBlock},
    rich_text::dom_node_conversion::dom_node_to_rich_text,
    stylesheet::{BreakInside, Direction, FlexWrap, Style, Stylesheet},
    utils::{debug_cursor::DebugCursor, node_lookup::NodeLookup, tree_iter::TreeNode},
    values::{Point, Pt},
};

use super::layout_engine::{LayoutEngine, NodeLayout};

pub struct PaginatedLayoutEngine<'a> {
    node_avoids_page_break: HashMap<NodeId, bool>,
    node_lookup: &'a NodeLookup<'a>,
    paginated_nodes: Vec<PaginatedNode>,
    paragraph_layout: &'a ParagraphLayout,
    layout_engine: &'a dyn LayoutEngine,
    stylesheet: &'a Stylesheet,
    page_height: Pt,
    pub debug_cursors: Vec<DebugCursor>,
}

impl<'a> PaginatedLayoutEngine<'a> {
    pub fn new(
        root_node: &DomNode,
        layout_engine: &'a dyn LayoutEngine,
        node_lookup: &'a NodeLookup,
        paragraph_layout: &'a ParagraphLayout,
        stylesheet: &'a Stylesheet,
        page_height: Pt,
    ) -> Result<Self, DocumentGenerationError> {
        let mut paginated_layout_engine = Self {
            node_avoids_page_break: HashMap::new(),
            node_lookup,
            paragraph_layout,
            paginated_nodes: vec![],
            stylesheet,
            page_height,
            layout_engine,
            debug_cursors: vec![],
        };

        paginated_layout_engine.compute_paginated_layout(root_node)?;

        Ok(paginated_layout_engine)
    }

    fn compute_paginated_layout(
        &mut self,
        root_node: &DomNode,
    ) -> Result<&mut Self, DocumentGenerationError> {
        // Probably not the most efficient way to do this
        for (node, _) in root_node.block_iter() {
            // if node is no-break and first child of parent, then parent is
            // also no break
            if self.does_node_avoid_page_break(node) {
                self.node_avoids_page_break.insert(node.node_id(), true);
                self.apply_page_break_avoid_rules(node);
            }
        }

        let mut draw_cursor = DrawCursor {
            y_offset: Pt(0.),
            page_index: 0,
            page_break_debt: Pt(0.),
        };

        let mut prior_sibling_layout = NodeLayout::default();

        for (node, _parent) in root_node.block_iter() {
            let node_layout = self.layout_engine.get_node_layout(node.node_id());

            let cursor_offset =
                node_layout.top - prior_sibling_layout.top - draw_cursor.page_break_debt;

            draw_cursor.y_offset += cursor_offset;

            self.draw_paginated_node(&mut draw_cursor, node_layout.clone(), node)?;

            prior_sibling_layout = node_layout;
        }

        Ok(self)
    }

    fn draw_paginated_node(
        &mut self,
        draw_cursor: &mut DrawCursor,
        mut node_layout: NodeLayout,
        node: &DomNode,
    ) -> Result<(), DocumentGenerationError> {
        let mut style = self.node_lookup.get_style(node).clone();

        draw_cursor.page_break_debt = Pt(0.);

        let mut adjusted_layout = NodeLayout {
            top: draw_cursor.y_offset,
            ..node_layout.clone()
        };

        if adjusted_layout.bottom() > self.page_height
            && (adjusted_layout.top > self.page_height
                || *self
                    .node_avoids_page_break
                    .get(&node.node_id())
                    .unwrap_or(&false))
        {
            adjusted_layout.top = Pt(0.);
            draw_cursor.y_offset = Pt(0.);
            draw_cursor.page_index += 1;
        }

        // By this point, the draw cursor is in the correct place to start
        // the current node.

        let drawable_node = self
            .convert_dom_node_to_drawable(node, &adjusted_layout, &style)
            .unwrap();

        let paginated_node = PaginatedNode {
            page_layout: adjusted_layout,
            page_index: draw_cursor.page_index,
            drawable_node,
        };

        if let DrawableNode::Text(text_node) = &paginated_node.drawable_node {
            self.draw_text_node(draw_cursor, &mut style, &mut node_layout, text_node)?;
        } else {
            self.paginated_nodes.push(paginated_node);
        }

        Ok(())
    }

    fn draw_text_node(
        &mut self,
        draw_cursor: &mut DrawCursor,
        style: &mut Style::Unmergeable,
        node_layout: &mut NodeLayout,
        text_node: &DrawableTextNode,
    ) -> Result<(), DocumentGenerationError> {
        let mut line_offset = 0;

        while line_offset < text_node.text_block.lines.len() {
            let cumulative_height: Vec<_> = text_node.text_block.lines[line_offset..]
                .iter()
                .scan(Pt(0.), |state, line| {
                    *state += line.line_metrics.height; // line.line_metrics.descent - line.line_metrics.ascent;

                    Some(*state)
                })
                .collect();

            let page_break_index = cumulative_height.iter().position(|&bottom| {
                bottom + style.padding.top + draw_cursor.y_offset > self.page_height
            });

            let block_height = match page_break_index {
                Some(idx) if idx > 0 => cumulative_height[idx - 1],
                Some(_) => Pt(0.),
                None => cumulative_height.last().cloned().unwrap_or(Pt(0.)),
            };

            let page_break = page_break_index
                .map(|break_offset| break_offset + line_offset)
                .unwrap_or_else(|| text_node.text_block.lines.len());

            for height in cumulative_height.iter() {
                self.debug_cursors.push(DebugCursor {
                    page_index: draw_cursor.page_index,
                    position: Point {
                        x: Pt(0.),
                        y: draw_cursor.y_offset + *height + style.padding.top,
                    },
                    label: format!("{} {block_height}", *height),
                });
            }

            let partial_text_block = RenderedTextBlock {
                lines: text_node.text_block.lines[line_offset..page_break].to_vec(),
                // ..text_node.text_block.clone()
            };

            let pn = PaginatedNode {
                page_layout: NodeLayout {
                    top: draw_cursor.y_offset,
                    ..node_layout.clone()
                },
                page_index: draw_cursor.page_index,
                drawable_node: DrawableNode::Text(DrawableTextNode {
                    text_block: partial_text_block,
                    style: style.clone(),
                }),
            };
            node_layout.height -= block_height + style.padding.top;

            self.paginated_nodes.push(pn);

            line_offset = page_break;
            if line_offset < text_node.text_block.lines.len() {
                draw_cursor.page_index += 1;
                draw_cursor.y_offset = Pt(0.);
                draw_cursor.page_break_debt += block_height + style.padding.top;
                style.margin.top = Pt(0.);
                style.padding.top = Pt(0.);
            }
        }
        Ok(())
    }

    fn convert_dom_node_to_drawable(
        &self,
        dom_node: &DomNode,
        layout: &NodeLayout,
        style: &Style::Unmergeable,
    ) -> Result<DrawableNode, DocumentGenerationError> {
        let drawable_node = match dom_node {
            DomNode::Text(text_node) => {
                // FIXME: This should also have already been computed by now
                let rich_text =
                    dom_node_to_rich_text(text_node, self.node_lookup, self.stylesheet)?;

                // FIXME: We already calculated the text block in the yoga layout
                // engine. Either re-use that or pass it into the layout engine?
                let text_block = self
                    .paragraph_layout
                    .calculate_layout(
                        ParagraphStyle::left(),
                        &rich_text,
                        layout.width - style.padding.horizontal(),
                    )
                    .unwrap();

                DrawableNode::Text(DrawableTextNode {
                    text_block,
                    style: style.clone(),
                })
            }
            _ => DrawableNode::Container(DrawableContainerNode {
                style: style.clone(),
            }),
        };

        Ok(drawable_node)
    }

    pub fn apply_page_break_avoid_rules(&mut self, node: &DomNode) {
        while let Some(parent) = self.node_lookup.get_parent(node) {
            if parent.children()[0].node_id() == node.node_id() {
                if self.node_avoids_page_break.insert(parent.node_id(), true) == Some(true) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn paginated_nodes(&self) -> &Vec<PaginatedNode> {
        &self.paginated_nodes
    }

    pub fn does_node_avoid_page_break(&self, node: &DomNode) -> bool {
        let style = self.node_lookup.get_style(node);

        matches!(node, DomNode::Image(_))
            || *self
                .node_avoids_page_break
                .get(&node.node_id())
                .unwrap_or(&false)
            || style.break_inside != BreakInside::Auto
            || style.flex.direction != Direction::Column
            || style.flex.wrap != FlexWrap::NoWrap
    }
}
