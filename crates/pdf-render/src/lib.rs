#![doc = include_str!("../README.md")]

use block_layout::{layout_engine::LayoutEngine, yoga::YogaLayout};
use bytes::Bytes;
use doc_structure::{DomNode, FontFamilyInfo};
use document_builder::DocumentBuilder;
use fonts::{FontCollection, FontFamilyCollection};
use paragraph_layout::{ParagraphLayout, ParagraphStyle};
use print_pdf_writer::PrintPdfWriter;
use rich_text::dom_node_conversion::dom_node_to_rich_text;
use std::{io::Write, rc::Rc};
use utils::dom_lookup::NodeLookup;
use values::{Point, Pt};

pub mod block_layout;
pub mod doc_structure;
pub mod document_builder;
pub mod error;
pub mod fonts;
pub mod page_sizes;
pub mod paragraph_layout;
pub mod print_pdf_writer;
pub mod rich_text;
pub mod stylesheet;
pub mod utils;
pub mod values;

use error::DocumentGenerationError;

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
    let pdf_writer = PrintPdfWriter::new(
        &doc_structure.document_title,
        page_sizes::LETTER,
        &font_collection,
    );

    let stylesheet = &doc_structure.stylesheet;
    let dom_lookup = NodeLookup::from_root_node(&doc_structure.root, &stylesheet)?;

    let mut paragraph_layout = ParagraphLayout::new();
    paragraph_layout.load_fonts(&font_collection)?;

    let paragraph_layout = Rc::new(paragraph_layout);

    let mut layout_engine = YogaLayout::new(&dom_lookup);
    layout_engine.build_node_layout(
        page_sizes::LETTER.width.into(),
        &doc_structure.root,
        stylesheet,
        paragraph_layout.clone(),
    )?;

    let mut pdf_builder = DocumentBuilder::new(pdf_writer);

    for (node, parent) in doc_structure.root.block_iter() {
        if let DomNode::Text(text_node) = node {
            let layout = layout_engine.get_node_layout(text_node.node_id);

            let style = dom_lookup.get_style(text_node);
            let rich_text = dom_node_to_rich_text(text_node, &dom_lookup, stylesheet)?;

            // FIXME: We already calculated the text block in the yoga layout
            // engine. Either re-use that or pass it into the layout engine?
            let text_block = paragraph_layout
                .calculate_layout(
                    ParagraphStyle::default(),
                    &rich_text,
                    layout.width - Pt(style.padding.left + style.padding.right),
                )
                .unwrap();

            pdf_builder.draw_dom_node(node, &dom_lookup, &layout)?;

            // TODO: Can we change this to take a ref instead?
            pdf_builder.write_text_block(
                text_block,
                Point {
                    x: layout.left + Pt(style.padding.left),
                    y: Pt::from(page_sizes::LETTER.height) - (layout.top + Pt(style.padding.top)),
                },
            )?;
        }
    }

    pdf_builder.into_inner().save(pdf_doc_writer)
}
