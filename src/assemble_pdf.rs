use std::{cell::RefCell, rc::Rc};

use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{nodes::TextNode, PdfDom},
    error::BadPdfLayout,
};

struct DropMe {}

impl DropMe {
    pub fn yo(&self) {
        println!("Yo!");
    }
}

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Drop it like it's hot!");
    }
}

pub fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    // Demonstration of the ability to have an item with a non-static lifetime
    //  doing stuff in a static lifetime
    let droppy = Rc::new(RefCell::new(DropMe {}));
    let mut old_droppy = Rc::clone(&droppy);

    let text_compute: TextComputeFn = Box::new(move |text_node: &TextNode| {
        let text_node = text_node.clone();
        let droppy = { Rc::clone(&droppy) };
        MeasureFunc::Boxed(Box::new(move |_sz| {
            // TODO: Replace with real text size calculation
            droppy.borrow().yo();
            Size {
                width: 32.,
                height: 32.,
            }
        }))
    });

    old_droppy.borrow().yo();

    let image_compute: ImageComputeFn = Box::new(|_image_node| {
        // TODO: Replace with real image size calculation
        MeasureFunc::Raw(move |_sz| Size {
            width: 32.,
            height: 32.,
        })
    });

    let layout = BlockLayout::build_layout(pdf_layout, text_compute, image_compute)?;

    for node in layout.draw_order() {
        let style = layout.get_style(node);
        let dom_node = layout.get_dom_node(node);

        println!("Node: {node:?}");
        println!("Style: {style:?}");
        println!("Dom: {dom_node:?}");
        // println!("{:?}", layout.layout_style_map());
    }
    // let layout_to_style_nodes: HashMap<Node, Style> = HashMap::new();
    // let layout_to_dom_nodes: HashMap<Node, &DomNode> = HashMap::new();

    Ok(())
}
