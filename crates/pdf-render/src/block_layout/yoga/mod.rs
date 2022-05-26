mod node_context;
mod style_conversions;

use std::{collections::HashMap, rc::Rc};

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, ParagraphStyle},
    rich_text::dom_node_conversion::dom_node_to_rich_text,
    stylesheet::Stylesheet,
    utils::parent_lookup::ParentLookup,
    values::Pt,
};

use self::node_context::NodeContext;

use super::layout_engine::{LayoutEngine, NodeLayout};

use polyhorn_yoga as yoga;

use crate::stylesheet::Style;

use yoga::{MeasureMode, NodeRef, Size};

#[derive(Default)]
pub struct YogaLayout {
    parent_node_ids: ParentLookup,
    yoga_nodes_by_id: HashMap<NodeId, yoga::Node>,
}

impl YogaLayout {
    pub fn new() -> Self {
        Self::default()
    }
}

// TODO: Implement for images as well
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

    Size { width, height }
}

impl LayoutEngine for YogaLayout {
    fn get_node_layout(&self, node_id: NodeId) -> NodeLayout {
        let ancestors = self.parent_node_ids.get_ancestors(node_id);

        let layout = self.yoga_nodes_by_id.get(&node_id).unwrap().get_layout();

        // Yoga doesn't give us absolute positions. All the positions are
        //  relative to the parent node so we have to build it up from ancestors
        ancestors.iter().fold(
            NodeLayout {
                left: Pt(layout.left() as f64),
                top: Pt(layout.top() as f64),
                right: Pt(layout.right() as f64),
                width: Pt(layout.width() as f64),
                height: Pt(layout.height() as f64),
            },
            |acc, node_id| {
                let parent = self.yoga_nodes_by_id.get(&node_id).unwrap().get_layout();

                NodeLayout {
                    left: acc.left + Pt(parent.left() as f64),
                    right: acc.right + Pt(parent.right() as f64),
                    top: acc.top + Pt(parent.top() as f64),
                    // We don't modify the width or height of the target node,
                    // just the offsets
                    ..acc
                }
            },
        )
    }

    fn build_node_layout(
        &mut self,
        page_width: Pt,
        root_node: &DomNode,
        stylesheet: &Stylesheet,
        paragraph_layout: Rc<ParagraphLayout>,
    ) -> Result<(), DocumentGenerationError> {
        for (node, parent) in root_node.block_iter() {
            let node_style = stylesheet.get_style(Style::default(), node.styles())?;

            let mut layout_node = yoga::Node::from(node_style);

            if let DomNode::Text(text_node) = node {
                let rich_text = dom_node_to_rich_text(text_node, &parent, stylesheet)?;

                let context = yoga::Context::new(NodeContext {
                    node_id: node.node_id(),
                    rich_text,
                    paragraph_layout: paragraph_layout.clone(),
                    paragraph_metrics: None,
                    calculate_error: None,
                });

                layout_node.set_context(Some(context));
                layout_node.set_measure_func(Some(measure_func));
            }

            if let Some(parent) = parent {
                self.parent_node_ids
                    .add_parent(node.node_id(), parent.node_id());

                let parent_yoga_node = self
                    .yoga_nodes_by_id
                    .get_mut(&parent.node_id())
                    .expect("Parent should have been added already");

                parent_yoga_node.insert_child(&mut layout_node, parent_yoga_node.child_count());
            }

            self.yoga_nodes_by_id.insert(node.node_id(), layout_node);
        }

        let root_yoga_node = self.yoga_nodes_by_id.get_mut(&root_node.node_id()).unwrap();

        root_yoga_node.calculate_layout(
            page_width.0 as f32,
            yoga::Undefined,
            yoga::Direction::LTR,
        );

        Ok(())
    }
}
