use std::rc::Rc;

use crate::{
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, RenderedTextBlock},
    rich_text::RichText,
};

pub(super) struct TextNodeContext {
    pub rich_text: RichText,
    pub paragraph_layout: Rc<ParagraphLayout>,
    pub text_block: Option<RenderedTextBlock>,
    pub calculate_error: Option<DocumentGenerationError>,
}
