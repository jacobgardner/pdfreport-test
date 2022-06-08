use crate::{
    doc_structure::{TextChild, TextNode},
    error::DocumentGenerationError,
    fonts::FontAttributes,
    rich_text::RichTextSpan,
    stylesheet::Stylesheet,
    utils::node_lookup::NodeLookup,
    values::Pt,
};

use super::RichText;

pub fn dom_node_to_rich_text(
    text_node: &TextNode,
    node_lookup: &NodeLookup,
    stylesheet: &Stylesheet,
) -> Result<RichText, DocumentGenerationError> {
    // let ancestor_style = dom_lookup
    //     .get_ancestors(text_node.node_id)
    //     .into_iter()
    //     .rev()
    //     .fold(Default::default(), |acc, node| {
    //         dom_lookup.get_style(node).merge_inherited_styles(&acc)
    //     });

    // let parent_style = if let Some(parent_id) = dom_lookup.get_parent_id(text_node) {
    //     dom_lookup.get_style(parent_id).clone()
    // } else {
    //     Default::default()
    // };

    // let text_node_style =
    //     stylesheet.compute_style(Default::default(), &parent_style, text_node.styles())?;

    // let ancestor_style = Style::Unmergeable::default().merge_style(
    //     &dom_lookup
    //         .get_style(text_node)
    //         .merge_inherited_styles(&ancestor_style),
    // );

    let text_node_style = node_lookup.get_style(text_node);

    let mut rich_text_spans: Vec<RichTextSpan> = vec![];

    for child in &text_node.children {
        for (node, parent) in child.iter() {
            let current_style = if let Some(parent) = parent {
                if let TextChild::TextNode(text_node) = parent {
                    stylesheet.get_style(text_node_style.clone(), text_node.styles())?
                } else {
                    panic!("TextChild::Content cannot have children!");
                }
            } else {
                text_node_style.clone()
            };

            if let TextChild::Content(content) = node {
                let mut span = RichTextSpan::from(content.as_str());

                span.attributes = FontAttributes {
                    weight: current_style.font.weight,
                    style: current_style.font.style,
                };

                span.color = current_style.color;
                span.font_family = current_style.font.family;
                span.size = current_style.font.size;

                rich_text_spans.push(span);
            }
        }
    }

    Ok(RichText(rich_text_spans))
}
