// TODO: Remove these once we have more stuff implemented
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use block_layout::{layout_engine::LayoutEngine, yoga_layout::YogaLayout};
use bytes::Bytes;
use doc_structure::{FontFamilyInfo, NodeId};
use document_builder::DocumentBuilder;
use fonts::{FontAttributes, FontCollection, FontFamilyCollection, FontSlant, FontWeight};
use paragraph_layout::{ParagraphLayout, ParagraphStyle};
use print_pdf_writer::PrintPdfWriter;
use rich_text::{RichText, RichTextSpan};
use std::{collections::HashMap, io::Write, rc::Rc, thread::current};
use values::{Color, Point, Pt};

mod block_layout;
pub mod doc_structure;
mod document_builder;
pub mod error;
mod fonts;
mod page_sizes;
mod paragraph_layout;
mod print_pdf_writer;
mod rich_text;
mod stylesheet;
mod utils;
mod values;

use error::DocumentGenerationError;

use crate::block_layout::yoga_layout::NodeContext;

static TEMP_FONT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/inter-static/Inter-Regular.ttf");

pub fn load_fonts_from_doc_structure(
    fonts: &[FontFamilyInfo],
) -> Result<FontCollection, DocumentGenerationError> {
    let mut font_collection = FontCollection::new();

    for font_family_info in fonts.iter() {
        let mut font_family = FontFamilyCollection::new(&font_family_info.family_name);

        for font_info in font_family_info.fonts.iter() {
            let f = std::fs::read(&font_info.source).unwrap();

            font_family.add_font(font_info.attributes, Bytes::from(f))?;
        }

        font_collection.add_family(font_family)?;
    }

    Ok(font_collection)
}

pub fn build_pdf_from_dom<W: Write>(
    doc_structure: &doc_structure::DocStructure,
    pdf_doc_writer: W,
) -> Result<W, DocumentGenerationError> {
    let font_collection = load_fonts_from_doc_structure(&doc_structure.fonts)?;
    let mut pdf_writer = PrintPdfWriter::new(
        &doc_structure.document_title,
        page_sizes::LETTER,
        &font_collection,
    );

    let stylesheet = &doc_structure.stylesheet;

    let mut paragraph_layout = ParagraphLayout::new();
    paragraph_layout.load_fonts(&font_collection)?;

    let paragraph_layout = Rc::new(paragraph_layout);

    let mut layout_engine = YogaLayout::new();
    let layout_nodes = layout_engine.build_node_layout(
        &doc_structure.root,
        &stylesheet,
        paragraph_layout.clone(),
    )?;

    pdf_writer.load_fonts(&font_collection)?;

    let mut pdf_builder = DocumentBuilder::new(pdf_writer);

    let mut node_parents: HashMap<NodeId, NodeId> = HashMap::new();

    for (node, parent) in doc_structure.root.block_iter() {
        if let Some(parent) = parent {
            node_parents.insert(node.node_id(), parent.node_id());
        }
    }

    let calc_abs_layout = |mut node_id: NodeId| {
        let mut current_layout = layout_nodes.get(&node_id).unwrap().get_layout();

        let mut left = current_layout.left();
        let mut top = current_layout.top();

        loop {
            let parent = node_parents.get(&node_id);

            if let Some(parent) = parent {
                node_id = *parent;
                let yoga_node = layout_nodes.get(parent).unwrap();
                let parent_layout = yoga_node.get_layout();

                left += parent_layout.left();
                top += parent_layout.top();
            } else {
                break;
            }
        }

        (left, top)
    };

    for (_, node) in layout_nodes.iter() {
        let context = node.get_own_context();

        if let Some(context) = context {
            let context = context.downcast_ref::<NodeContext>().unwrap();

            let text_block = context.paragraph_metrics.as_ref().unwrap().clone();

            let (x, y) = calc_abs_layout(context.node_id);

            println!("Final layout: {x}, {y}");

            // TODO: Can we change this to take a ref instead?
            pdf_builder.write_text_block(
                text_block,
                Point {
                    x: Pt(x as f64),
                    y: Pt::from(page_sizes::LETTER.height) - Pt(y as f64),
                },
            )?;
        }
    }

    //     pdf_builder.write_text_block(
    //     text_block,
    //     Point {
    //         x: Pt(10.),
    //         y: Pt(600.),
    //     },
    // )?;

    pdf_builder.into_inner().save(pdf_doc_writer)
}
