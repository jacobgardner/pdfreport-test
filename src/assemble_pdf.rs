use std::{cell::RefCell, collections::HashMap, rc::Rc};

use futures::future::try_join_all;
use printpdf::Pt;
use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{FontFamilyInfo, PdfDom, nodes::TextNodeIterItem},
    error::BadPdfLayout,
    fonts::{FontData, FontFamily, FontManager},
    pdf_writer::{GlyphLookup, PdfWriter},
    resource_cache::ResourceCache,
    rich_text::RichText,
    text_layout::{LayoutFonts, TextLayout},
};

pub async fn load_fonts(
    resource_cache: &ResourceCache,
    font_families: &[FontFamilyInfo],
) -> Result<FontManager, BadPdfLayout> {
    let mut families = HashMap::new();

    for font_family_info in font_families {
        let font_data = try_join_all(
            font_family_info
                .fonts
                .iter()
                .map(|font_info| FontData::from_font_info(resource_cache, font_info)),
        )
        .await?;

        let font_family = FontFamily {
            family_name: font_family_info.family_name.clone(),
            fonts: font_data,
        };

        families.insert(font_family_info.family_name.clone(), font_family);
    }

    Ok(FontManager { families })
}

// TODO: Most of these impls don't belong in this file...
impl GlyphLookup for Rc<LayoutFonts> {
    fn get_glyph_ids(
        &self,
        line: &str,
        font_lookup: &crate::fonts::FontLookup,
    ) -> Result<Vec<u16>, BadPdfLayout> {
        LayoutFonts::get_glyph_ids(self, line, font_lookup)
    }
}

pub async fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    // Demonstration of the ability to have an item with a non-static lifetime
    //  doing stuff in a static lifetime
    //
    let resource_cache = ResourceCache::new();
    let layout_paragraph_metrics_map = Rc::new(RefCell::new(HashMap::new()));

    let font_manager = load_fonts(&resource_cache, &pdf_layout.fonts).await?;
    let layout_fonts = Rc::new(LayoutFonts::with_font_manager(&font_manager));

    let pdf_writer = Rc::new(RefCell::new(PdfWriter::new(
        &font_manager,
        layout_fonts.clone(),
    )));
    let text_layout = Rc::new(TextLayout::new(layout_fonts));
    let measure_errors: Rc<RefCell<Vec<BadPdfLayout>>> = Rc::new(RefCell::new(Vec::new()));
    let closure_measure_errors = measure_errors.clone();

    let _shared_pdf_writer = pdf_writer.clone();

    let styles = Rc::new(pdf_layout.styles.clone());

    // We have to use move here twice so each closure gets ownership of the Rc and can
    // manage its lifetime
    let text_compute: TextComputeFn = Box::new(move |node, text_node, current_style| {
        let text_node = text_node.clone();

        // There may be a better way to do this
        let text_layout = text_layout.clone();

        let layout_paragraph_metrics_map = layout_paragraph_metrics_map.clone();
        let styles = styles.clone();
        let measure_errors = closure_measure_errors.clone();
        MeasureFunc::Boxed(Box::new(move |sz| {
            if sz.width == Number::Undefined || !measure_errors.borrow().is_empty() {
                return Size {
                    width: 0.,
                    height: 0.,
                };
            }

            let styles = styles.clone();

            let text_layout = text_layout.clone();

            let full_text = text_node.raw_text();

            let converted_style = match (*current_style).clone().try_into() {
                Ok(style) => style,
                Err(err) => {
                    measure_errors.borrow_mut().push(err);
                    return Size {
                        width: 0.,
                        height: 0.,
                    };
                }
            };

            let rich_text = RichText::new(&full_text, converted_style);

            for TextNodeIterItem(range, style) in text_node.iter_rich_text(&current_style, &styles) {
                // TODO: Implement me please :'(
                // rich_text.push_style(style.into(), range);
                println!("- {range:?}: {style:?}");
                unimplemented!();
            }

            let width = if let Number::Defined(width) = sz.width {
                width
            } else {
                unreachable!();
            };

            let paragraph_metrics =
                text_layout.compute_paragraph_layout(&rich_text, Pt(width as f64));

            let computed_height = paragraph_metrics.height.0;

            let layout_paragraph_metrics_map = layout_paragraph_metrics_map.clone();
            layout_paragraph_metrics_map
                .borrow_mut()
                .insert(node, paragraph_metrics);

            Size {
                width,
                height: computed_height as f32,
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

    let layout = BlockLayout::build_layout(
        pdf_layout,
        text_compute,
        image_compute,
        crate::page_sizes::LETTER,
    )?;

    if !measure_errors.borrow().is_empty() {
        drop(layout);

        // We can do this as long as layout is dropped by this point
        //  because all the references to measure errors should be dropped
        //  as part of it.
        let err = Rc::try_unwrap(measure_errors)
            .unwrap()
            .into_inner()
            .into_iter()
            .next()
            .unwrap();

        return Err(err);
    }

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
