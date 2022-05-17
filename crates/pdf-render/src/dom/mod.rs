use serde::Deserialize;

#[derive(Deserialize)]
pub struct PdfDom {
    pub filename: String,
}
