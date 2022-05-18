use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TextNode {
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextChild {
    Content(String),
    TextNode(TextNode),
}