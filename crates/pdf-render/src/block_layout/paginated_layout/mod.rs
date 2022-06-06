use std::{collections::HashMap, fmt::Display};
mod node_visitor;

use crate::{
    doc_structure::{DomNode, HasNodeId, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, ParagraphStyle, RenderedTextBlock},
    rich_text::dom_node_conversion::dom_node_to_rich_text,
    stylesheet::{BreakInside, Direction, FlexWrap, Style, Stylesheet},
    utils::{node_lookup::NodeLookup, tree_iter::TreeNode},
    values::{Point, Pt},
};

use self::node_visitor::PaginationVisitor;

use super::layout_engine::{LayoutEngine, NodeLayout};

#[derive(Clone, Debug)]
pub struct PaginatedLayout {
    // TODO: Rename to something better
    pub layout: NodeLayout,
    pub page_index: usize,
}

impl PaginatedLayout {
    pub fn left(&self) -> Pt {
        self.layout.left
    }

    pub fn top(&self) -> Pt {
        self.layout.top
    }
}

impl Display for PaginatedLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Page {} -> {}", self.page_index, self.layout)
    }
}

pub struct DrawCursor {
    y_offset: Pt,
    page_index: usize,
    debt: Pt,
}

impl Display for DrawCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {}, {} -{}",
            self.page_index, self.y_offset, self.debt
        )
    }
}

pub struct DebugCursor {
    pub page_index: usize,
    pub position: Point<Pt>,
    pub label: String,
}

pub struct PaginatedLayoutEngine<'a> {
    layouts: HashMap<NodeId, PaginatedLayout>,
    node_avoids_page_break: HashMap<NodeId, bool>,
    node_lookup: &'a NodeLookup<'a>,
    paginated_nodes: Vec<PaginatedNode>,
    paragraph_layout: &'a ParagraphLayout,
    layout_engine: &'a dyn LayoutEngine,
    stylesheet: &'a Stylesheet,
    page_height: Pt,
    pub debug_cursors: Vec<DebugCursor>,
}

// Json Processed -> Flexbox layout (yoga) -> Text layout -> Pagination Layout
// -> PDF writer

impl<'a> PaginatedLayoutEngine<'a> {
    pub fn new(
        root_node: &DomNode,
        layout_engine: &'a dyn LayoutEngine,
        // absolute_layout: &HashMap<NodeId, NodeLayout>,
        node_lookup: &'a NodeLookup,
        paragraph_layout: &'a ParagraphLayout,
        stylesheet: &'a Stylesheet,
        page_height: Pt,
    ) -> Result<Self, DocumentGenerationError> {
        let mut paginated_layout_engine = Self {
            layouts: HashMap::new(),
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
            if self.does_node_avoid_page_break(&node) {
                self.node_avoids_page_break.insert(node.node_id(), true);
                self.apply_page_break_avoid_rules(&node);
            }

            // if node is no-break and first child of parent, then parent is
            // also no break
        }

        // We want the relative offset between the previous node and the current
        // to calculate the adjusted position. ()

        let mut visitor = PaginationVisitor::new(self);

        root_node.visit_nodes(&mut visitor, None)?;

        Ok(self)
    }

    fn draw_paginated_node(
        &mut self,
        draw_cursor: &mut DrawCursor,
        mut node_layout: NodeLayout,
        node: &DomNode,
    ) -> Result<(), DocumentGenerationError> {
        let mut style = self.node_lookup.get_style(node).clone();

        draw_cursor.debt = Pt(0.);

        let mut adjusted_layout = NodeLayout {
            top: draw_cursor.y_offset,
            ..node_layout.clone()
        };

        if adjusted_layout.bottom() > self.page_height {
            if adjusted_layout.top > self.page_height
                || *self
                    .node_avoids_page_break
                    .get(&node.node_id())
                    .unwrap_or(&false)
            {
                adjusted_layout.top = Pt(0.);
                draw_cursor.y_offset = Pt(0.);
                draw_cursor.page_index += 1;
            }
        }

        self.debug_cursors.push(DebugCursor {
            page_index: draw_cursor.page_index,
            position: Point {
                x: Pt(200.),
                y: draw_cursor.y_offset,
            },
            label: format!("Draw Start - {}", node_layout.height),
        });

        // By this point, the draw cursor is in the correct place to start
        // the current node.

        let drawable_node = self
            .convert_dom_node_to_drawable(node, &adjusted_layout, &style)
            .unwrap();

        let paginated_node = PaginatedNode {
            layout: PaginatedLayout {
                layout: adjusted_layout,
                page_index: draw_cursor.page_index,
            },
            drawable_node,
        };

        // println!("Pn: {:?}", paginated_node.layout);

        if let DrawableNode::Text(text_node) = &paginated_node.drawable_node {
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

                // if let Some(idx) = page_break_index if idx > 1
                let block_height = match page_break_index {
                    Some(idx) if idx > 0 => cumulative_height[idx - 1],
                    Some(_) => Pt(0.),
                    None => cumulative_height.last().cloned().unwrap_or(Pt(0.)),
                };

                // let block_height = page_break_index
                //     .map(|idx| {
                //         if idx > 0 {
                //             cumulative_height[idx - 1]
                //         } else {
                //         }
                //     })
                //     .unwrap_or(Pt(0.));

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
                    lines: text_node.text_block.lines[line_offset..page_break]
                        .iter()
                        .cloned()
                        .collect(),
                    ..text_node.text_block.clone()
                };

                println!("Height remaining: {}", node_layout.height);

                let pn = PaginatedNode {
                    layout: PaginatedLayout {
                        layout: NodeLayout {
                            top: draw_cursor.y_offset,
                            ..node_layout.clone()
                        },
                        page_index: draw_cursor.page_index,
                    },
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
                    draw_cursor.debt += block_height + style.padding.top;
                    style.margin.top = Pt(0.);
                    style.padding.top = Pt(0.);
                }
            }
        } else {
            self.paginated_nodes.push(paginated_node);
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
                    dom_node_to_rich_text(text_node, &self.node_lookup, &self.stylesheet)?;

                // FIXME: We already calculated the text block in the yoga layout
                // engine. Either re-use that or pass it into the layout engine?
                let text_block = self
                    .paragraph_layout
                    .calculate_layout(
                        ParagraphStyle::left(),
                        &rich_text,
                        layout.width - (style.padding.left + style.padding.right),
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

    pub fn get_node_layout(&self, node_id: NodeId) -> &PaginatedLayout {
        self.layouts.get(&node_id).unwrap()
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

pub struct DrawableNodeIter {}

#[derive(Clone, Debug)]
pub enum DrawableNode {
    Text(DrawableTextNode),
    Container(DrawableContainerNode),
    // Image(DrawableImageNode)
}

impl DrawableNode {
    pub fn style(&self) -> &Style::Unmergeable {
        match self {
            Self::Text(node) => &node.style,
            Self::Container(node) => &node.style,
        }
    }

    pub fn is_leaf_node(&self) -> bool {
        match self {
            Self::Container(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawableTextNode {
    pub text_block: RenderedTextBlock,
    pub style: Style::Unmergeable,
}

#[derive(Clone, Debug)]
pub struct DrawableContainerNode {
    pub style: Style::Unmergeable,
}

#[derive(Clone, Debug)]
pub struct PaginatedNode {
    pub layout: PaginatedLayout,
    pub drawable_node: DrawableNode,
}
