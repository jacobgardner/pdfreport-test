use std::{collections::HashMap, fmt::Display, hash::Hash};

use crate::{
    block_layout::paginated_layout,
    doc_structure::{DomNode, HasNodeId, NodeId},
    stylesheet::{BreakInside, Direction, FlexWrap},
    utils::{dom_lookup::NodeLookup, tree_iter::TreeNode},
    values::Pt,
};

use super::layout_engine::{LayoutEngine, NodeLayout};

pub struct PaginatedLayout {
    layout_relative_to_page: NodeLayout,
    page_number: usize,
}

impl PaginatedLayout {
    pub fn left(&self) -> Pt {
        self.layout_relative_to_page.left
    }

    pub fn top(&self) -> Pt {
        self.layout_relative_to_page.top
    }
}

impl Display for PaginatedLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Page {} -> {}",
            self.page_number, self.layout_relative_to_page
        )
    }
}

pub struct PaginatedLayoutEngine<'a> {
    layouts: HashMap<NodeId, PaginatedLayout>,
    node_avoids_page_break: HashMap<NodeId, bool>,
    node_lookup: &'a NodeLookup<'a>,
}

impl<'a> PaginatedLayoutEngine<'a> {
    pub fn new(
        root_node: &DomNode,
        layout_engine: &dyn LayoutEngine,
        // absolute_layout: &HashMap<NodeId, NodeLayout>,
        node_lookup: &'a NodeLookup,
        page_height: Pt,
    ) -> Self {
        let mut paginated_layout_engine = Self {
            layouts: HashMap::new(),
            node_avoids_page_break: HashMap::new(),
            node_lookup,
        };

        // Probably not the most efficient way to do this
        for (node, _) in root_node.block_iter() {
            if paginated_layout_engine.does_node_avoid_page_break(&node) {
                paginated_layout_engine
                    .node_avoids_page_break
                    .insert(node.node_id(), true);
                paginated_layout_engine.apply_page_break_avoid_rules(&node);
            }

            // if node is no-break and first child of parent, then parent is
            // also no break
        }

        // We want the relative offset between the previous node and the current
        // to calculate the adjusted position. ()

        let mut prev_top = Pt(0.);
        let mut prev_page_number = 0;
        let mut prev_page_top = Pt(0.);

        //  By this point we should know if all the nodes avoid break or not
        for (node, _) in root_node.block_iter() {
            let layout = layout_engine.get_node_layout(node.node_id());

            let relative_offset = layout.top - prev_top;

            let offset_layout = NodeLayout {
                top: prev_page_top + relative_offset,
                ..layout
            };

            let paginated_layout = PaginatedLayout {
                layout_relative_to_page: offset_layout,
                page_number: 0,
            };

            println!("New Layout: {paginated_layout}");

            prev_page_number = paginated_layout.page_number;
            prev_page_top = paginated_layout.layout_relative_to_page.top;
            prev_top = layout.top;

            paginated_layout_engine
                .layouts
                .insert(node.node_id(), paginated_layout);
        }

        paginated_layout_engine
    }

    pub fn get_node_layout(&self, node_id: NodeId) -> &PaginatedLayout {
        self.layouts.get(&node_id).unwrap()
    }

    pub fn apply_page_break_avoid_rules(&mut self, mut node: &DomNode) {
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
