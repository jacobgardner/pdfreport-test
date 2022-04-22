use itertools::Itertools;
use num::iter::Range;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextChild {
    Content(String),
    TextNode(TextNode),
}

impl TextChild {
    pub fn raw_text(&self) -> String {
        match self {
            TextChild::Content(content) => content.clone(),
            TextChild::TextNode(node) => node.raw_text(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

pub struct TextNodeIterator<'a> {
    root_node: &'a TextNode,
}

impl TextNode {
    pub fn raw_text(&self) -> String {
        self.children.iter().map(|t| t.raw_text()).join("")
    }
}

impl<'a> Iterator for TextNodeIterator<'a> {
    type Item = (Range<usize>, &'a [&'a str]);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct ImageNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub content: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct StyledNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}

#[derive(Clone, Deserialize, Debug)]
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
