use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextChild {
    Content(String),
    TextNode(TextNode),
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextNode {
    #[serde(default="styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

#[derive(Deserialize, Debug)]
pub struct ImageNode {
    #[serde(default="styles_list")]
    pub styles: Vec<String>,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct StyledNode {
    #[serde(default="styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DomNode {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}

pub fn styles_list() -> Vec<String> {
    Vec::new()
}

impl DomNode {
    pub fn styles(&self) -> &Vec<String> {
        match self {
            DomNode::Styled(node) => &node.styles,
            DomNode::Text(node) => &node.styles,
            DomNode::Image(node) => &node.styles,
        }
    }
}
