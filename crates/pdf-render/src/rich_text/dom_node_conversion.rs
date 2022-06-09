use crate::{
    doc_structure::{TextChild, TextNode},
    error::DocumentGenerationError,
    rich_text::RichTextSpan,
    stylesheet::Stylesheet,
    utils::node_lookup::NodeLookup,
};

use super::RichText;

pub fn dom_node_to_rich_text(
    text_node: &TextNode,
    node_lookup: &NodeLookup,
    stylesheet: &Stylesheet,
) -> Result<RichText, DocumentGenerationError> {
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
                rich_text_spans.push(RichTextSpan::new(content.as_str(), current_style));
            }
        }
    }

    Ok(RichText(rich_text_spans))
}
