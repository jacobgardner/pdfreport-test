use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum TextChild {
    Content(String),
    TextNode(TextNode),
}

#[derive(Deserialize, Debug)]
pub struct TextNode {
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

#[derive(Deserialize, Debug)]
pub struct ImageNode {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct StyledNode {
    styles: Vec<String>,
    children: Vec<Node>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Node {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}
