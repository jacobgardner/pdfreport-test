use serde::Deserialize;

use super::DomNode;

#[derive(Default, Deserialize, Debug)]
pub struct StyledNode {
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}

impl StyledNode {
    pub fn with_children(children: Vec<DomNode>, styles: &[&str]) -> Self {
        Self {
            children,
            styles: styles.iter().map(|&s| s.to_owned()).collect(),
            ..Default::default()
        }
    }
}
