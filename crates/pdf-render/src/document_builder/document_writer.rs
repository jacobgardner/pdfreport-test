use crate::{
    error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock,
    stylesheet::BorderRadiusStyle,
    values::{Point, Pt, Rect, Color},
};

pub trait UnstructuredDocumentWriter {
    fn draw_rect(
        &mut self,
        rect: Rect<Pt>,
        border_width: Pt,
        border_color: Option<Color>,
        background_color: Option<Color>,
        border_radius: Option<BorderRadiusStyle::Unmergeable>,
    );

    fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError>;
}
