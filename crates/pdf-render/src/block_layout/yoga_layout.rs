use std::collections::HashMap;

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    stylesheet::{Direction, Stylesheet},
};

use super::layout_engine::LayoutEngine;

use polyhorn_yoga as yoga;

use crate::stylesheet::Style;

use yoga::{Edge, FlexDirection, MeasureMode, NodeRef, Size, StyleUnit};

pub struct YogaLayout {}

impl YogaLayout {
    pub fn new() -> Self {
        Self {}
    }
}

struct NodeContext<'a> {
    dom_node: &'a DomNode
}

extern "C" fn measure_func(
    node_ref: NodeRef,
    width: f32,
    width_measure_mode: MeasureMode,
    height: f32,
    height_measure_mode: MeasureMode,
) -> Size {
    let context = yoga::Node::get_context(&node_ref)
        .unwrap()
        .downcast_ref::<NodeContext>()
        .unwrap();

    println!("NODE CONTEXT: {:?}", context.dom_node);

    unimplemented!();
}

impl LayoutEngine for YogaLayout {
    fn build_node_layout(
        &mut self,
        root_node: &DomNode,
        stylesheet: &Stylesheet,
    ) -> Result<(), DocumentGenerationError> {
        let mut yoga_nodes_by_id: HashMap<NodeId, yoga::Node> = HashMap::new();
        
        for (node, parent) in root_node.block_iter() {
            let style = stylesheet.get_style(
                &node
                    .styles()
                    .iter()
                    .map(|classname| classname.as_ref())
                    .collect::<Vec<_>>(),
            )?;

            let mut layout_node = yoga::Node::from(style);

            if let DomNode::Text(text_node) = node {
                let context = yoga::Context::new(NodeContext { dom_node: &node });

                layout_node.set_context(Some(context));
                layout_node.set_measure_func(Some(measure_func));
            }

            if let Some(parent) = parent {
                let parent_yoga_node = yoga_nodes_by_id
                    .get_mut(&parent.node_id())
                    .expect("Parent should have been added already");

                parent_yoga_node.insert_child(&mut layout_node, parent_yoga_node.child_count());
            }

            yoga_nodes_by_id.insert(node.node_id(), layout_node);
        }

        let root_yoga_node = yoga_nodes_by_id.get_mut(&root_node.node_id()).unwrap();

        root_yoga_node.calculate_layout(100., 100., yoga::Direction::LTR);

        for (_, yoga_node) in yoga_nodes_by_id.iter() {
            println!("Layout is {:?}", yoga_node.get_layout());
        }

        todo!()
    }
}

impl From<Style> for yoga::Node {
    fn from(style: Style) -> Self {
        let mut layout_node = yoga::Node::new();

        // TODO: Look into flex_styles! macro to clean this up
        layout_node.set_border(Edge::All, style.border.width);

        layout_node.set_margin(Edge::Top, StyleUnit::Point(style.margin.top.into()));
        layout_node.set_margin(Edge::Right, StyleUnit::Point(style.margin.right.into()));
        layout_node.set_margin(Edge::Bottom, StyleUnit::Point(style.margin.bottom.into()));
        layout_node.set_margin(Edge::Left, StyleUnit::Point(style.margin.left.into()));

        layout_node.set_padding(Edge::Top, StyleUnit::Point(style.padding.top.into()));
        layout_node.set_padding(Edge::Right, StyleUnit::Point(style.padding.right.into()));
        layout_node.set_padding(Edge::Bottom, StyleUnit::Point(style.padding.bottom.into()));
        layout_node.set_padding(Edge::Left, StyleUnit::Point(style.padding.left.into()));

        layout_node.set_flex_direction(style.flex.direction.into());
        layout_node.set_flex_wrap(style.flex.wrap.into());
        // layout_node.set_align_items(style.flex.align_items.into());
        // layout_node.set_align_self(style.flex.align_self.into());
        layout_node.set_flex_grow(style.flex.grow);
        layout_node.set_flex_shrink(style.flex.shrink);
        // layout_node.set_flex_basis(style.flex.basis);

        layout_node
    }
}
