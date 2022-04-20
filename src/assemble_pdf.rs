use std::{cell::RefCell, rc::Rc};

use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{nodes::TextNode, PdfDom},
    error::BadPdfLayout,
    pdf_writer::PdfWriter,
};

pub fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    // Demonstration of the ability to have an item with a non-static lifetime
    //  doing stuff in a static lifetime
    //

    let pdf_writer = Rc::new(RefCell::new(PdfWriter::new()));

    let shared_pdf_writer = pdf_writer.clone();
    // We have to use move here twice so each closure gets ownership of the Rc and can
    // manage its lifetime
    let text_compute: TextComputeFn = Box::new(move |text_node: &TextNode| {
        let text_node = text_node.clone();

        // There may be a better way to do this
        let pdf_writer = { Rc::clone(&shared_pdf_writer) };
        MeasureFunc::Boxed(Box::new(move |_sz| {
            // TODO: Replace with real text size calculation
            //
            pdf_writer.borrow_mut().add_page();

            Size {
                width: 32.,
                height: 32.,
            }
        }))
    });

    pdf_writer.borrow_mut().add_page();

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
