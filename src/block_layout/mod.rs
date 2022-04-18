use stretch::Stretch;

use crate::dom::{PdfLayout, Style};

mod flex_style;

pub fn layout_pdf(_pdf: &PdfLayout) -> Result<(), Box<dyn std::error::Error>> {
    let mut stretch = Stretch::new();

    let style_stack = vec![Style::default()];

    let current_style = style_stack.last().unwrap().clone();
    let node = stretch.new_node(current_style.try_into()?, vec![])?;

    let _layout = stretch.layout(node)?;

    Ok(())
}
