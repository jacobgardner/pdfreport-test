mod node_context;
mod style_conversions;

use crate::doc_structure::HasNodeId;
use std::{collections::HashMap, rc::Rc};

use crate::{
    doc_structure::{DomNode, NodeId},
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, ParagraphStyle},
    rich_text::dom_node_conversion::dom_node_to_rich_text,
    stylesheet::Stylesheet,
    utils::dom_lookup::NodeLookup,
    values::Pt,
};

use self::node_context::NodeContext;

use super::layout_engine::{LayoutEngine, NodeLayout};

use polyhorn_yoga as yoga;

use yoga::{MeasureMode, NodeRef, Size};

pub struct YogaLayout<'a> {
    dom_lookup: &'a NodeLookup<'a>,
    yoga_nodes_by_id: HashMap<NodeId, yoga::Node>,
}

impl<'a> YogaLayout<'a> {
    pub fn new(dom_lookup: &'a NodeLookup) -> Self {
        Self {
            dom_lookup,
            yoga_nodes_by_id: HashMap::new(),
        }
    }
}

// TODO: Implement for images as well
// TODO: We should *PROBABLY* respect the measure mode
extern "C" fn measure_func(
    node_ref: NodeRef,
    width: f32,
    _width_measure_mode: MeasureMode,
    _height: f32,
    _height_measure_mode: MeasureMode,
) -> Size {
    let context = yoga::Node::get_context_mut(&node_ref)
        .unwrap()
        .downcast_mut::<NodeContext>()
        .unwrap();

    let text_block = context.paragraph_layout.calculate_layout(
        ParagraphStyle::default(),
        &context.rich_text,
        Pt(width as f64 - context.style.padding.left - context.style.padding.right),
    );

    match text_block {
        Ok(text_block) => {
            let height = text_block.height().0 as f32;
            let width = text_block.width().0 as f32;

            context.text_block = Some(text_block);

            Size { width, height }
        }
        Err(err) => {
            context.calculate_error = Some(err);

            Size {
                width: 0.,
                height: 0.,
            }
        }
    }
}

impl<'a> LayoutEngine for YogaLayout<'a> {
    fn get_node_layout(&self, node_id: NodeId) -> NodeLayout {
        let ancestors = self.dom_lookup.get_ancestor_ids(node_id);

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
                let parent = self.yoga_nodes_by_id.get(node_id).unwrap().get_layout();

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
            let node_style = stylesheet.get_style(Default::default(), node.styles())?;

            let mut layout_node = yoga::Node::from(node_style.clone());

            if let DomNode::Text(text_node) = node {
                let rich_text = dom_node_to_rich_text(text_node, &self.dom_lookup, stylesheet)?;

                let context = yoga::Context::new(NodeContext {
                    node_id: node.node_id(),
                    rich_text,
                    paragraph_layout: paragraph_layout.clone(),
                    style: node_style,
                    text_block: None,
                    calculate_error: None,
                });

                layout_node.set_context(Some(context));
                layout_node.set_measure_func(Some(measure_func));
            }

            if let Some(parent) = parent {
                let parent_yoga_node = self
                    .yoga_nodes_by_id
                    .get_mut(&parent.node_id())
                    .expect("Parent should have been added already");

                parent_yoga_node.insert_child(&mut layout_node, parent_yoga_node.child_count());
            }

            self.yoga_nodes_by_id.insert(node.node_id(), layout_node);
        }

        let root_yoga_node = self.yoga_nodes_by_id.get_mut(&root_node.node_id()).unwrap();

        root_yoga_node.calculate_layout(page_width.0 as f32, yoga::Undefined, yoga::Direction::LTR);

        // We stored any errors during calculation in the context so now we have
        // to check them now that we're back in our own code.
        for (_, node) in self.yoga_nodes_by_id.iter() {
            check_node_for_error(node)?;
        }

        Ok(())
    }
}

fn check_node_for_error(node: &yoga::Node) -> Result<(), DocumentGenerationError> {
    if let Some(context) = node.get_own_context_mut() {
        let context = context.downcast_mut::<NodeContext>().unwrap();

        let err = context.calculate_error.take();

        if let Some(err) = err {
            return Err(err);
        }
    }

    Ok(())
}
