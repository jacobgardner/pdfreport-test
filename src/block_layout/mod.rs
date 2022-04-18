use stretch::Stretch;

use crate::dom::{PdfDom, Style};

mod flex_style;

pub fn compute_pdf_layout(pdf_layout: &PdfDom) -> Result<(), Box<dyn std::error::Error>> {
    let mut stretch = Stretch::new();

    let style_stack = vec![Style::default()];

    let current_style = style_stack.last().unwrap().clone();
    let node = stretch.new_node(current_style.try_into()?, vec![])?;

    // pdf_layout.root

    let _layout = stretch.layout(node)?;

    Ok(())
}
