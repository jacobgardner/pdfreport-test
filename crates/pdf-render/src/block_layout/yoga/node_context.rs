use std::rc::Rc;

use crate::{
    error::DocumentGenerationError,
    paragraph_layout::{ParagraphLayout, RenderedTextBlock},
    rich_text::RichText,
    values::Pt,
};

#[derive(Default)]
pub(super) struct TextBlockWithWidth(pub Vec<(Pt, RenderedTextBlock)>);

impl TextBlockWithWidth {
    pub fn take_closest_by_width(mut self, width: Pt) -> RenderedTextBlock {
        self.0.drain(..).min_by_key(|(content_width, _)| {
            ((width.0 - content_width.0).abs() * 10000.) as usize
        }).unwrap().1
    }
}

pub(super) struct TextNodeContext {
    pub rich_text: RichText,
    pub paragraph_layout: Rc<ParagraphLayout>,
    pub text_block_by_width: TextBlockWithWidth,
    pub calculate_error: Option<DocumentGenerationError>,
}
