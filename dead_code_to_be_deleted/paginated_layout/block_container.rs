use crate::styles::{BlockSpacing, Color};

use super::dom_node::DomNode;

#[derive(Debug, Default, Clone)]
struct BlockStyle {
    margin: BlockSpacing,
    padding: BlockSpacing,
    border_width: f32,
    border_radius: f32,
    border_color: Color,
}

impl BlockStyle {}

// A block is the basic building block of the layout
struct Block {
    breakable: bool,
    style: BlockStyle,
    content_width: f32,
    content_height: f32,
    children: Vec<Box<dyn DomNode>>,
}

// If possible would like to use stretch or similar layout library
//
// Given we know the width, we can compute the height
// Given we know the height, we can determine if we exceed the page
// Given we know we exceed the page, profit??

impl Block {
    pub fn width(&self) -> f32 {
        self.style.margin.width()
            + self.style.border_width
            + self.style.padding.width()
            + self.content_width
    }

    pub fn height(&self) -> f32 {
        self.style.margin.height()
            + self.style.border_width
            + self.style.padding.height()
            + self.content_height
    }

    // pub fn content_rect(&self) -> Rect {}
}

impl Default for Block {
    fn default() -> Self {
        Self {
            breakable: true,
            ..Default::default()
        }
    }
}

impl DomNode for Block {
    fn add_child(&mut self, child: Box<dyn DomNode>) {
        self.children.push(child);
    }

    fn children(&self) -> &[Box<dyn DomNode>] {
        &self.children
    }
}
