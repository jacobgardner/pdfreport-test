// TODO: Remove these once we have more stuff implemented
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use bytes::Bytes;
use doc_structure::FontFamilyInfo;
use document_builder::DocumentBuilder;
use fonts::{FontAttributes, FontCollection, FontFamilyCollection, FontStyle, FontWeight};
use geometry::Pt;
use paragraph_layout::{ParagraphLayout, LayoutStyle};
use print_pdf_writer::PrintPdfWriter;
use rich_text::{RichText, RichTextSpan};
use std::io::Write;

pub mod doc_structure;
pub mod document_builder;
pub mod error;
mod fonts;
pub mod geometry;
pub mod page_sizes;
pub mod paragraph_layout;
pub mod print_pdf_writer;
pub mod rich_text;

use error::DocumentGenerationError;

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
    let mut pdf_writer = PrintPdfWriter::new(&doc_structure.document_title, page_sizes::LETTER);

    let font_collection = load_fonts_from_doc_structure(&doc_structure.fonts)?;

    let mut paragraph_layout = ParagraphLayout::new();
    paragraph_layout.load_fonts(&font_collection)?;
    

    pdf_writer.load_fonts(&font_collection)?;

    let regular = font_collection
        .lookup_font("Inter", &FontAttributes::default())?
        .font_id();
    let bold = font_collection
        .lookup_font(
            "Inter",
            &FontAttributes {
                weight: FontWeight::ExtraBold,
                ..Default::default()
            },
        )?
        .font_id();
    let italic = font_collection
        .lookup_font(
            "Inter",
            &FontAttributes {
                style: FontStyle::Italic,
                ..Default::default()
            },
        )?
        .font_id();

    let mut pdf_builder = DocumentBuilder::new(pdf_writer);

    let line = RichText(vec![
        RichTextSpan {
            text: "The quick brown".to_owned(),
            font_id: bold,
            size: Pt(32.),
        },
        RichTextSpan {
            text: " fox jumps over the".to_owned(),
            font_id: regular,
            size: Pt(15.),
        },
        RichTextSpan {
            text: " lazy dog".to_owned(),
            font_id: italic,
            size: Pt(8.),
        },
    ]);
    
    let text_block = paragraph_layout.calculate_layout(LayoutStyle {}, &line, Pt(200.))?;
    
    for line in text_block.lines {
        pdf_builder.write_line(line.rich_text)?;
    }

    pdf_builder.into_inner().save(pdf_doc_writer)
}
