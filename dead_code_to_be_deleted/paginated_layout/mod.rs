use crate::styles::{BlockSpacing, Color};

use self::dom_node::DomNode;

mod dom_node;
mod block_container;
mod text_node;

// A book is a collection of pages
//  the book is the arbitrator of when new pages are created
struct Book {}


struct PaginatedDocument {
    children: Vec<Box<dyn DomNode>>,
}

impl PaginatedDocument {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn with_child(&mut self, f: impl FnOnce(&mut Self) -> ()) -> &mut Self {
        f(self);

        self
    }

    pub fn set_style(&mut self) -> &mut Self {
        self
    }
}

impl DomNode for PaginatedDocument {
    fn add_child(&mut self, child: Box<dyn DomNode>) {
        self.children.push(child);
    }

    fn children(&self) -> &[Box<dyn DomNode>] {
        &self.children
    }
}

pub fn pseudo_layout() {
    let mut root = PaginatedDocument::new();

    root.with_child(|ctx| {
        ctx.set_style();

        ctx.with_child(|ctx| {});
    })
    .with_child(|ctx| {
        // ctx.
    })
    .with_child(|ctx| {});
}

