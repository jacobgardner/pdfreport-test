use serde::Deserialize;

#[derive(Deserialize)]
pub struct DocStructure {
    pub filename: String,
    pub document_title: String,
}
