use std::char::MAX;


pub trait DomNode {
    fn add_child(&mut self, child: Box<dyn DomNode>);
    fn children(&self) -> &[Box<dyn DomNode>];
}

const MAX_PAGE_Y: f32 = 300.;

pub fn calculate_layout() {

  let remaining_y = MAX_PAGE_Y;

}