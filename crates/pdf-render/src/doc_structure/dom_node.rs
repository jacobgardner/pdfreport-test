use serde::Deserialize;

use super::{StyledNode, TextNode, ImageNode};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DomNode {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}