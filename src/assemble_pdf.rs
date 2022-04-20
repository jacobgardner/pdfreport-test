use std::collections::HashMap;

use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{nodes::TextNode, DomNode, PdfDom, Style},
    error::BadPdfLayout,
};

pub fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    let text_compute: TextComputeFn = Box::new(|text_node: &TextNode| {
        let text_node = text_node.clone();
        MeasureFunc::Boxed(Box::new(move |_sz| {
            println!("{:?}", text_node.styles);
            Size {
                width: 32.,
                height: 32.,
            }
        }))
    });

    let image_compute: ImageComputeFn = Box::new(|_image_node| {
        MeasureFunc::Raw(move |_sz| Size {
            width: 32.,
            height: 32.,
        })
    });

    let layout = BlockLayout::build_layout(pdf_layout, text_compute, image_compute)?;

    let layout_to_style_nodes: HashMap<Node, Style> = HashMap::new();
    let layout_to_dom_nodes: HashMap<Node, &DomNode> = HashMap::new();

    Ok(())
}
