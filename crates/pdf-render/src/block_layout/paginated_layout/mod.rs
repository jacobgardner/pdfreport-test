use std::{cell::RefCell, collections::HashMap, fmt::Display};

use crate::{
    doc_structure::{DomNode, HasNodeId, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, ParagraphStyle, RenderedTextBlock},
    rich_text::dom_node_conversion::dom_node_to_rich_text,
    stylesheet::{BreakInside, Direction, FlexWrap, Style, Stylesheet},
    utils::{node_lookup::NodeLookup, tree_iter::TreeNode},
    values::{Point, Pt},
};

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
}

impl Display for DrawCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Page {}, {}", self.page_index, self.y_offset)
    }
}

pub struct DebugCursor {
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
    debug_cursors: Vec<DebugCursor>,
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

        // let root_layout = layout_engine.get_node_layout(root_node.node_id());

        let draw_cursor = RefCell::new(DrawCursor {
            y_offset: Pt(0.),
            page_index: 0,
        });

        let prior_sibling_layout = RefCell::new(NodeLayout::default());
        let mut depth = 0;

        let layout_engine = self.layout_engine;

        root_node.visit_nodes(
            &mut |node, parent| {
                let mut draw_cursor = draw_cursor.borrow_mut();
                let mut prior_sibling_layout = prior_sibling_layout.borrow_mut();
                // let style = paginated_layout_engine.node_lookup.get_style(node);

                let node_layout = layout_engine.get_node_layout(node.node_id());
                if let Some(parent) = parent {
                    let parent_layout = layout_engine.get_node_layout(parent.node_id());

                    let offset = if parent.first_child() == node {
                        depth += 1;
                        println!("Parent to child {depth}");

                        // Parent to child movement
                        node_layout.top - parent_layout.top
                    } else {
                        println!(
                            "Sibling to sibling {} - {}",
                            node_layout.top,
                            prior_sibling_layout.bottom()
                        );
                        node_layout.top - prior_sibling_layout.bottom()
                    };

                    println!("Moving cursor down {offset}");

                    draw_cursor.y_offset += offset;
                }

                self.draw_paginated_node(&mut draw_cursor, &node_layout, node)?;

                *prior_sibling_layout = node_layout;

                Ok(())
            },
            &mut |node, parent| {
                // let style = paginated_layout_engine.node_lookup.get_style(node);
                let mut draw_cursor = draw_cursor.borrow_mut();
                let node_layout = layout_engine.get_node_layout(node.node_id());

                if let Some(parent) = parent {
                    // Do pagination stuff
                    //

                    let parent_layout = layout_engine.get_node_layout(parent.node_id());
                    if parent.last_child() == node {
                        draw_cursor.y_offset += parent_layout.bottom() - node_layout.bottom();
                    }
                }

                Ok(())
            },
            None,
        )?;

        Ok(self)
    }

    fn draw_paginated_node(
        &mut self,
        draw_cursor: &mut DrawCursor,
        node_layout: &NodeLayout,
        node: &DomNode,
    ) -> Result<(), DocumentGenerationError> {
        let style = self.node_lookup.get_style(node);
        let mut adjusted_layout = NodeLayout {
            top: draw_cursor.y_offset,
            ..node_layout.clone()
        };

        if adjusted_layout.bottom() <= self.page_height {
            // Not breaking
        } else {
            if *self
                .node_avoids_page_break
                .get(&node.node_id())
                .unwrap_or(&false)
            {
                adjusted_layout.top = Pt(0.);
                draw_cursor.y_offset = Pt(0.);
                draw_cursor.page_index += 1;
            }
        }

        // By this point, the draw cursor is in the correct place to start
        // the current node.

        let drawable_node = self
            .convert_dom_node_to_drawable(node, &adjusted_layout, style)
            .unwrap();

        let paginated_node = PaginatedNode {
            layout: PaginatedLayout {
                layout: adjusted_layout,
                page_index: draw_cursor.page_index,
            },
            drawable_node,
        };

        if let DrawableNode::Text(_) = paginated_node.drawable_node {
            draw_cursor.y_offset += node_layout.height; //  - Pt(style.margin.bottom);
        }

        self.paginated_nodes.push(paginated_node);

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
                        layout.width - Pt(style.padding.left + style.padding.right),
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
