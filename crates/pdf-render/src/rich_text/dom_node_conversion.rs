use crate::{doc_structure::{DomNode, TextNode, TextChild}, stylesheet::{Stylesheet, Style}, error::DocumentGenerationError, rich_text::RichTextSpan, fonts::FontAttributes, values::Pt};

use super::RichText;

pub fn dom_node_to_rich_text(
    text_node: &TextNode,
    parent_node: &Option<&DomNode>,
    stylesheet: &Stylesheet,
) -> Result<RichText, DocumentGenerationError> {
  
  let parent_style = if let Some(parent_node) = parent_node {
    // FIXME: This does not take into account the parent relying on ancestors'
    // default styles
    stylesheet.get_style(Default::default(), parent_node.styles())?
  } else {
    Default::default()
  };
  
  let text_node_style = stylesheet.get_style(parent_style, text_node.styles() )?;
  

  let mut rich_text_spans:  Vec<RichTextSpan> = vec![];


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
          
          // TODO: Just have span use font styles
          span.attributes = FontAttributes {
            weight: current_style.font.weight,
            style: current_style.font.style
          };
          
          span.color = current_style.color;
          span.font_family = current_style.font.family;
          span.size = Pt(current_style.font.size as f64);
          
          
          rich_text_spans.push(span);
        }
      }
  }

  Ok(RichText(rich_text_spans))
}
