use crate::{
    block_layout::layout_engine::NodeLayout, image::Svg, paragraph_layout::RenderedTextBlock,
    stylesheet::Style,
};

#[derive(Clone, Debug)]
pub struct DrawableTextNode {
    pub text_block: RenderedTextBlock,
    pub style: Style,
}

#[derive(Clone, Debug)]
pub struct DrawableContainerNode {
    pub style: Style,
}

#[derive(Clone, Debug)]
pub enum Image {
    Svg(Svg),
}

#[derive(Clone, Debug)]
pub struct DrawableImageNode {
    pub style: Style,
    pub image: Image,
}

#[derive(Clone, Debug)]
pub struct PaginatedNode {
    pub page_layout: NodeLayout,
    pub page_index: usize,
    pub drawable_node: DrawableNode,
}

#[derive(Clone, Debug)]
pub enum DrawableNode {
    Text(DrawableTextNode),
    Container(DrawableContainerNode),
    Image(DrawableImageNode),
}

impl DrawableNode {
    pub fn style(&self) -> &Style {
        match self {
            Self::Text(node) => &node.style,
            Self::Container(node) => &node.style,
            Self::Image(node) => &node.style,
        }
    }

    pub fn is_leaf_node(&self) -> bool {
        !matches!(self, Self::Container(_))
    }
}
