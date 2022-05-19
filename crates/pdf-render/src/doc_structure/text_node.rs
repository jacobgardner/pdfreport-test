use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
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

impl TextNode {
    pub fn with_children(children: Vec<TextChild>, styles: &[&str]) -> Self {
        Self {
            children,
            styles: styles.iter().map(|&s| s.to_owned()).collect(),
            ..Default::default()
        }
    }
}
