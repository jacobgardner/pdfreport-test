use std::collections::HashMap;

use stretch::{geometry::Size, style::Dimension, Stretch};

use crate::{
    dom::{MergeableStyle, Node, PdfDom, Style},
    error::BadPdfLayout,
};

mod flex_style;

fn build_layout_nodes(
    stretch: &mut Stretch,
    style_stack: &mut Vec<Style>,
    style_lookup: &HashMap<String, MergeableStyle>,
    current_layout_node: stretch::node::Node,
    current_pdf_node: &Node,
) -> Result<(), Box<dyn std::error::Error>> {
    match current_pdf_node {
        Node::Styled(styled_node) => {
            let mut updated_style = style_stack
                .last()
                .expect("There should always be at least one style on the stack here.")
                .clone();

            for style_name in &styled_node.styles {
                let mergeable_style =
                    style_lookup
                        .get(style_name)
                        .ok_or_else(|| BadPdfLayout::UnmatchedStyle {
                            style_name: style_name.clone(),
                        })?;

                updated_style = updated_style.merge_style(mergeable_style);
            }

            let child_node = stretch.new_node(updated_style.try_into()?, vec![])?;
            stretch.add_child(current_layout_node, child_node)?;

            for child in &styled_node.children {
                build_layout_nodes(stretch, style_stack, style_lookup, child_node, child)?
            }
        }
        Node::Text(text_node) => {
            let mut updated_style = style_stack
                .last()
                .expect("There should always be at least one style on the stack here.")
                .clone();

            for style_name in &text_node.styles {
                let mergeable_style =
                    style_lookup
                        .get(style_name)
                        .ok_or_else(|| BadPdfLayout::UnmatchedStyle {
                            style_name: style_name.clone(),
                        })?;
            
                println!("Mergable: {:?}", mergeable_style);

                updated_style = updated_style.merge_style(mergeable_style);
            }

            let stretch_style = stretch::style::Style::try_from(updated_style)?;
            println!("{:?}", stretch_style);

            let child_node = stretch.new_node(stretch_style, vec![])?;
            stretch.add_child(current_layout_node, child_node)?;
        }
        Node::Image(_image_node) => {
            let child_node = stretch.new_node(Style::default().try_into()?, vec![])?;
            stretch.add_child(current_layout_node, child_node)?;
        }
    }

    Ok(())
}

pub fn compute_pdf_layout(pdf_layout: &PdfDom) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Right now the root node of stretch is a generic node that doesn't
    // have a DOM equivalent. We may want to change that so that the stretch
    // root node is 1:1 with the dom root node if we can

    let mut stretch = Stretch::new();

    let mut style_stack = vec![Style::default()];

    let current_style = style_stack.last().unwrap().clone();
    let node = stretch.new_node(
        stretch::style::Style {
            // size: Size {
            //     // TODO: This is arbitrary. Should match the width of the page
            //     width: Dimension::Points(100.),
            //     height: Dimension::Undefined,
            // },
            ..current_style.try_into()?
        },
        vec![],
    )?;

    let root_node = &pdf_layout.root;

    build_layout_nodes(
        &mut stretch,
        &mut style_stack,
        &pdf_layout.styles,
        node,
        root_node,
    )?;

    // pdf_layout.root
    stretch.compute_layout(node, Size::undefined())?;

    let layout = stretch.layout(node)?;

    println!("{:?}", layout);

    Ok(())
}
