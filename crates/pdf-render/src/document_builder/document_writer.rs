use crate::{
    block_layout::paginated_layout::PaginatedLayout,
    error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock,
    stylesheet::BorderRadiusStyle,
    values::{Color, Point, Pt, Rect},
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
        layout: &PaginatedLayout,
    ) -> Result<&mut Self, DocumentGenerationError>;
}
