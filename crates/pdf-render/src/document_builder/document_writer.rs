use crate::{
    error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock,
    values::{Point, Pt},
};

pub trait DocumentWriter {
    fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError>;
}
