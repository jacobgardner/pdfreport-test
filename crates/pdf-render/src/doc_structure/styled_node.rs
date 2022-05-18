use serde::Deserialize;

use super::DomNode;


#[derive(Deserialize, Debug)]
pub struct StyledNode {
    #[serde(default)]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}
