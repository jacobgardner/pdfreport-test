use std::collections::HashMap;

use stretch::node::MeasureFunc;
use stretch2 as stretch;
use stretch2::prelude::*;

use crate::{
    dom::{
        nodes::{ImageNode, TextNode},
        DomNode, MergeableStyle, PdfDom, Style,
    },
    error::BadPdfLayout,
};

mod flex_style;

pub type TextComputeFn<'a> = Box<dyn Fn(&'a TextNode) -> MeasureFunc>;
pub type ImageComputeFn<'a> = Box<dyn Fn(&'a ImageNode) -> MeasureFunc>;

pub struct BlockLayout<'a> {
    pdf_dom: &'a PdfDom,
    stretch: Stretch,
    text_node_compute: TextComputeFn<'a>,
    image_node_compute: ImageComputeFn<'a>,

    layout_node_map: HashMap<Node, &'a DomNode>, // _error_type: PhantomData<E>,
}

impl<'a> BlockLayout<'a> {
    pub fn build_layout(
        pdf_dom: &'a PdfDom,
        text_compute: TextComputeFn<'a>,
        image_compute: ImageComputeFn<'a>,
    ) -> Result<Self, BadPdfLayout> {
        let mut layout = Self {
            pdf_dom,
            stretch: Stretch::new(),
            text_node_compute: text_compute,
            image_node_compute: image_compute,
            layout_node_map: HashMap::new(),
        };

        let mut style_stack = vec![Style::default()];

        let current_style = style_stack.last().unwrap().clone();
        let node = layout
            .stretch
            .new_node(
                stretch::style::Style {
                    // size: Size {
                    //     // TODO: This is arbitrary. Should match the width of the page
                    //     width: Dimension::Points(100.),
                    //     height: Dimension::Undefined,
                    // },
                    ..current_style.try_into()?
                },
                &[],
            )
            .expect("This should only be able to error if children are added.");

        let root_node = &pdf_dom.root;

        layout.build_layout_nodes(&mut style_stack, node, root_node)?;

        layout.stretch.compute_layout(
            node,
            Size {
                width: Number::Defined(8.5 * 72.), // 8.5 inches
                height: Number::Undefined,
            },
        )?;

        let node_layout = layout.stretch.layout(node)?;

        println!("{:?}", node_layout);

        Ok(layout)
    }

    fn styles(&self) -> &HashMap<String, MergeableStyle> {
        &self.pdf_dom.styles
    }

    fn build_layout_nodes(
        &mut self,
        style_stack: &mut Vec<Style>,
        current_layout_node: stretch::node::Node,
        current_pdf_node: &'a DomNode,
    ) -> Result<(), BadPdfLayout> {
        match current_pdf_node {
            DomNode::Styled(styled_node) => {
                let mut updated_style = style_stack
                    .last()
                    .expect("There should always be at least one style on the stack here.")
                    .clone();

                for style_name in &styled_node.styles {
                    let mergeable_style = self.styles().get(style_name).ok_or_else(|| {
                        BadPdfLayout::UnmatchedStyle {
                            style_name: style_name.clone(),
                        }
                    })?;

                    updated_style = updated_style.merge_style(mergeable_style);
                }

                let child_node = self.stretch.new_node(updated_style.try_into()?, &[])?;
                self.stretch.add_child(current_layout_node, child_node)?;

                for child in &styled_node.children {
                    self.build_layout_nodes(style_stack, child_node, child)?
                }
            }
            DomNode::Text(text_node) => {
                let mut updated_style = style_stack
                    .last()
                    .expect("There should always be at least one style on the stack here.")
                    .clone();

                for style_name in &text_node.styles {
                    let mergeable_style = self.styles().get(style_name).ok_or_else(|| {
                        BadPdfLayout::UnmatchedStyle {
                            style_name: style_name.clone(),
                        }
                    })?;

                    // println!("Mergable: {:?}", mergeable_style);

                    updated_style = updated_style.merge_style(mergeable_style);
                }

                let stretch_style = stretch::style::Style::try_from(updated_style)?;

                // We would want to pass in a function called something like:
                //  compute_text_size which takes in the dom node, current style,
                //  etc. and returns the desired closure, if we can
                let child_node = self
                    .stretch
                    .new_leaf(stretch_style, (self.text_node_compute)(text_node))?;
                self.stretch.add_child(current_layout_node, child_node)?;
            }
            DomNode::Image(image_node) => {
                // let child_node = self.stretch.new_leaf(
                //     Style::default().try_into()?,
                //     (self.image_node_compute)(image_node),
                // )?;
                // self.stretch.add_child(current_layout_node, child_node)?;
            }
        }

        Ok(())
    }
}
