use thiserror::Error;

#[derive(Error, Debug)]
pub enum BadPdfLayout {
    #[error("Could not find style, {style_name}, in stylesheet. Style names are case-sensitive.")]
    UnmatchedStyle { style_name: String },
}
