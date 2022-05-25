use std::{collections::HashMap, hash::Hash, rc::Rc};

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    page_sizes,
    paragraph_layout::{ParagraphLayout, ParagraphStyle, RenderedTextBlock},
    rich_text::{dom_node_conversion::dom_node_to_rich_text, RichText},
    stylesheet::{Direction, Stylesheet},
    values::Pt,
};

use super::layout_engine::LayoutEngine;

use polyhorn_yoga as yoga;

use crate::stylesheet::Style;

use yoga::{Edge, FlexDirection, MeasureMode, NodeRef, Size, StyleUnit, style};
use yoga::prelude::*;

pub struct YogaLayout {}

impl YogaLayout {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct NodeContext {
    pub rich_text: RichText,
    pub paragraph_layout: Rc<ParagraphLayout>,
    pub paragraph_metrics: Option<RenderedTextBlock>,
    pub node_id: NodeId,
}

extern "C" fn measure_func(
    node_ref: NodeRef,
    width: f32,
    width_measure_mode: MeasureMode,
    height: f32,
    height_measure_mode: MeasureMode,
) -> Size {
    let context = yoga::Node::get_context_mut(&node_ref)
        .unwrap()
        .downcast_mut::<NodeContext>()
        .unwrap();

    println!("Measure: {width}:{width_measure_mode:?}x{height}:{height_measure_mode:?}");
    println!("{}", context.rich_text.0[0].text);

    // FIXME: See if we can bubble up the error somehow?????
    let paragraph_metrics = context
        .paragraph_layout
        .calculate_layout(
            ParagraphStyle::default(),
            &context.rich_text,
            Pt(width as f64),
        )
        .unwrap();

    let height = paragraph_metrics.height().0 as f32;

    context.paragraph_metrics = Some(paragraph_metrics);

    let size = Size { width, height };

    println!("Output: {size:?}");

    size
}

impl LayoutEngine for YogaLayout {
    fn build_node_layout(
        &mut self,
        root_node: &DomNode,
        stylesheet: &Stylesheet,
        paragraph_layout: Rc<ParagraphLayout>,
    ) -> Result<HashMap<NodeId, yoga::Node>, DocumentGenerationError> {
        let mut yoga_nodes_by_id: HashMap<NodeId, yoga::Node> = HashMap::new();

        for (node, parent) in root_node.block_iter() {
            let node_style = stylesheet.get_style(Style::default(), node.styles())?;

            let mut layout_node = yoga::Node::from(node_style);

            if let DomNode::Text(text_node) = node {
                let rich_text = dom_node_to_rich_text(text_node, &parent, &stylesheet)?;

                // TODO: Can we do this without cloning???
                let context = yoga::Context::new(NodeContext {
                    node_id: node.node_id(),
                    rich_text,
                    paragraph_layout: paragraph_layout.clone(),
                    paragraph_metrics: None,
                });

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

        // println!("Root node child count: {}", root_yoga_node.child_count());
        // root_yoga_node.set_height(StyleUnit::Point((Pt::from(page_sizes::LETTER.height).0 as f32).into()));
        // root_yoga_node.set_width(StyleUnit::Point((Pt::from(page_sizes::LETTER.width).0 as f32).into()));
        // root_yoga_node.set_flex_wrap(yoga::Wrap::Wrap);

        root_yoga_node.calculate_layout(
            Pt::from(page_sizes::LETTER.width).0 as f32,
            yoga::Undefined,
            yoga::Direction::LTR,
        );

        // for (_, yoga_node) in yoga_nodes_by_id.iter() {
        //     println!("Layout is {:?}", yoga_node.get_layout());
        // }

        Ok(yoga_nodes_by_id)
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
