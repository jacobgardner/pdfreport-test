use crate::values::{Point, Pt};


pub struct DebugCursor {
    pub page_index: usize,
    pub position: Point<Pt>,
    pub label: String,
}