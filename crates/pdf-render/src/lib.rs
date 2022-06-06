#![doc = include_str!("../README.md")]

use block_layout::{
    layout_engine::LayoutEngine, paginated_layout::PaginatedLayoutEngine, yoga::YogaLayout,
};
use bytes::Bytes;
use doc_structure::FontFamilyInfo;
use document_builder::DocumentBuilder;
use fonts::{FontCollection, FontFamilyCollection};
use paragraph_layout::ParagraphLayout;
use print_pdf_writer::PrintPdfWriter;

use std::{io::Write, rc::Rc};

use utils::node_lookup::NodeLookup;
use values::Pt;

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
    let node_lookup = NodeLookup::from_root_node(&doc_structure.root, stylesheet)?;

    let mut paragraph_layout = ParagraphLayout::new();
    paragraph_layout.load_fonts(&font_collection)?;

    let paragraph_layout = Rc::new(paragraph_layout);

    let mut layout_engine = YogaLayout::new(&node_lookup);
    layout_engine.build_node_layout(
        page_sizes::LETTER.width.into(),
        &doc_structure.root,
        stylesheet,
        paragraph_layout.clone(),
    )?;

    let paginated_layout = PaginatedLayoutEngine::new(
        &doc_structure.root,
        &layout_engine,
        &node_lookup,
        &paragraph_layout,
        stylesheet,
        Pt::from(page_sizes::LETTER.height),
    )?;

    let mut pdf_builder = DocumentBuilder::new(pdf_writer, page_sizes::LETTER);

    // pdf_builder.draw_node(&PaginatedNode {
    //     layout: PaginatedLayout {
    //         layout: NodeLayout {
    //             left: Pt(0.),
    //             top: Pt(10.),
    //             width: Pt(100.),
    //             height: Pt(100.),
    //             right: Pt(0.),
    //         },
    //         page_number: 10,
    //     },
    //     drawable_node: DrawableNode::Text(DrawableTextNode {
    //         text_block: RenderedTextBlock::default(),
    //         style: Style::Unmergeable {
    //             debug: true,
    //             ..Default::default()
    //         }
    //     })
    // })?;

    for drawable_node in paginated_layout.paginated_nodes().iter() {
        pdf_builder.draw_node(drawable_node)?;
    }

    let pdf_writer = pdf_builder.into_inner();
    // pdf_writer.draw_debug_cursors(&paginated_layout.debug_cursors);

    // pdf_builder.draw_debug_cursors(&paginated_layout.debug_cursors);

    // for (node, parent) in doc_structure.root.block_iter() {
    //     let layout = layout_engine.get_node_layout(node.node_id());
    //     pdf_builder.draw_dom_node(node, &node_lookup, &layout)?;
    //     if let DomNode::Text(text_node) = node {
    //         let style = node_lookup.get_style(text_node);
    //         let rich_text = dom_node_to_rich_text(text_node, &node_lookup, stylesheet)?;

    //         // FIXME: We already calculated the text block in the yoga layout
    //         // engine. Either re-use that or pass it into the layout engine?
    //         let text_block = paragraph_layout
    //             .calculate_layout(
    //                 ParagraphStyle::left(),
    //                 &rich_text,
    //                 layout.width - Pt(style.padding.left + style.padding.right),
    //             )
    //             .unwrap();

    //         let node_layout = paginated_layout.get_node_layout(node.node_id());

    //         // TODO: Can we change this to take a ref instead?
    //         pdf_builder.write_text_block(text_block, &node_layout, &style)?;
    //     }
    // }

    pdf_writer.save(pdf_doc_writer)
}
