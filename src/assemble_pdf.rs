use std::{cell::RefCell, collections::HashMap, rc::Rc};

use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{nodes::TextNode, FontFamilyInfo, PdfDom},
    error::BadPdfLayout,
    fonts::{FontData, FontFamily, FontManager},
    pdf_writer::{GlyphLookup, PdfWriter},
    resource_cache::ResourceCache,
    text_layout::LayoutFonts,
};

pub async fn load_fonts(
    resource_cache: &mut ResourceCache,
    font_families: &[FontFamilyInfo],
) -> Result<FontManager, BadPdfLayout> {
    // let font_manager = FontManager::new();

    let mut families = HashMap::new();

    for font_family_info in font_families {
        let mut font_data: Vec<FontData> = Vec::new();

        for font_info in font_family_info.fonts.iter() {
            font_data.push(FontData::from_font_info(resource_cache, font_info).await?);
        }

        let font_family = FontFamily {
            family_name: font_family_info.family_name.clone(),
            fonts: font_data,
        };

        families.insert(font_family_info.family_name.clone(), font_family);
    }

    Ok(FontManager { families })
}

impl GlyphLookup for Rc<LayoutFonts> {
    fn get_glyph_ids(&self, line: &str, font_lookup: &crate::fonts::FontLookup) -> Vec<u16> {
        LayoutFonts::get_glyph_ids(self, line, font_lookup)
    }
}

pub async fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    // Demonstration of the ability to have an item with a non-static lifetime
    //  doing stuff in a static lifetime
    //
    let mut resource_cache = ResourceCache::new();

    let font_manager = load_fonts(&mut resource_cache, &pdf_layout.fonts).await?;
    let layout_fonts = Rc::new(LayoutFonts::with_font_manager(&font_manager));

    let pdf_writer = Rc::new(RefCell::new(PdfWriter::new(&font_manager, layout_fonts)));

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
