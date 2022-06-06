use crate::{
    block_layout::layout_engine::NodeLayout, paragraph_layout::RenderedTextBlock,
    stylesheet::Style,
};

#[derive(Clone, Debug)]
pub struct DrawableTextNode {
    pub text_block: RenderedTextBlock,
    pub style: Style::Unmergeable,
}

#[derive(Clone, Debug)]
pub struct DrawableContainerNode {
    pub style: Style::Unmergeable,
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
    // Image(DrawableImageNode)
}

impl DrawableNode {
    pub fn style(&self) -> &Style::Unmergeable {
        match self {
            Self::Text(node) => &node.style,
            Self::Container(node) => &node.style,
        }
    }

    pub fn is_leaf_node(&self) -> bool {
        !matches!(self, Self::Container(_))
    }
}

// #[derive(Clone, Debug)]
// pub struct PaginatedLayout {
//     // TODO: Rename to something better
//     pub layout: NodeLayout,
//     pub page_index: usize,
// }

// impl PaginatedLayout {
//     pub fn left(&self) -> Pt {
//         self.layout.left
//     }

//     pub fn top(&self) -> Pt {
//         self.layout.top
//     }
// }

// impl Display for PaginatedLayout {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Page {} -> {}", self.page_index, self.layout)
//     }
// }
