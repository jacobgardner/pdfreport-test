use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ImageNode {
    #[serde(default)]
    pub styles: Vec<String>,
    pub content: String,
}
