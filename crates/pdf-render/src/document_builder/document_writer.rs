use crate::{
    block_layout::paginated_layout::{PaginatedLayout, PaginatedNode},
    error::DocumentGenerationError,
    paragraph_layout::RenderedTextBlock,
    stylesheet::Style,
};

pub trait UnstructuredDocumentWriter {
    fn draw_node(&mut self, node: &PaginatedNode) -> Result<&mut Self, DocumentGenerationError>;

    // fn draw_rect(
    //     &mut self,
    //     rect: Rect<Pt>,
    //     border_width: Pt,
    //     border_color: Option<Color>,
    //     background_color: Option<Color>,
    //     border_radius: Option<BorderRadiusStyle::Unmergeable>,
    // );

    fn draw_text_block(
        &mut self,
        layout: &PaginatedLayout,
        style: &Style::Unmergeable,
        text_block: &RenderedTextBlock,
    ) -> Result<&mut Self, DocumentGenerationError>;
}
