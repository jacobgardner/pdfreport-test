use std::{cell::RefCell, collections::HashMap, rc::Rc};

use futures::future::try_join_all;
use printpdf::{Mm, Point, Pt};
use std::ops::Deref;
use std::ops::DerefMut;
use stretch2 as stretch;

use stretch::{node::MeasureFunc, prelude::*};

use crate::dom::DomNode;
use crate::page_sizes::A4;
use crate::page_sizes::LETTER;
use crate::{
    block_layout::{BlockLayout, ImageComputeFn, TextComputeFn},
    dom::{nodes::TextNodeIterItem, FontFamilyInfo, PdfDom},
    error::BadPdfLayout,
    fonts::{FontData, FontFamily, FontManager},
    line_metric::ParagraphMetrics,
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

use crate::dom::MergeableStyle;

// TODO: Better name lol
#[derive(Debug)]
struct RelevantThings {
    layout_paragraph_metrics_map: HashMap<Node, ParagraphMetrics>,
    layout_rich_text_map: HashMap<Node, RichText>,
    text_layout: TextLayout<Rc<LayoutFonts>>,
    measure_errors: Vec<BadPdfLayout>,
    styles: HashMap<String, MergeableStyle>,
}

pub async fn assemble_pdf(pdf_layout: &PdfDom) -> Result<(), BadPdfLayout> {
    // Demonstration of the ability to have an item with a non-static lifetime
    //  doing stuff in a static lifetime
    //
    let resource_cache = ResourceCache::new();

    let font_manager = load_fonts(&resource_cache, &pdf_layout.fonts).await?;
    let layout_fonts = Rc::new(LayoutFonts::with_font_manager(&font_manager));

    let pdf_writer = Rc::new(RefCell::new(PdfWriter::new(
        &font_manager,
        layout_fonts.clone(),
    )));

    let relevant_things = Rc::new(RefCell::new(RelevantThings {
        layout_paragraph_metrics_map: HashMap::new(),
        layout_rich_text_map: HashMap::new(),
        text_layout: TextLayout::new(layout_fonts),
        measure_errors: Vec::new(),
        styles: pdf_layout.styles.clone(),
    }));

    let relevant_things_for_closure = relevant_things.clone();

    // We have to use move here twice so each closure gets ownership of the Rc and can
    // manage its lifetime
    let text_compute: TextComputeFn = Box::new(move |node, text_node, current_style| {
        let text_node = text_node.clone();

        // There may be a better way to do this
        let relevant_things_for_closure = relevant_things_for_closure.clone();

        MeasureFunc::Boxed(Box::new(move |sz| {
            let relevant_things_for_closure = relevant_things_for_closure.clone();
            let mut relevant_things_for_closure = relevant_things_for_closure.borrow_mut();
            let RelevantThings {
                layout_rich_text_map,
                layout_paragraph_metrics_map,
                text_layout,
                styles,
                measure_errors,
                ..
            } = relevant_things_for_closure.deref_mut();

            if sz.width == Number::Undefined || !measure_errors.is_empty() {
                return Size {
                    width: 0.,
                    height: 0.,
                };
            }

            // let styles = styles.clone();

            // let text_layout = text_layout.clone();

            let full_text = text_node.raw_text();

            let converted_style = match (*current_style).clone().try_into() {
                Ok(style) => style,
                Err(err) => {
                    measure_errors.push(err);
                    return Size {
                        width: 0.,
                        height: 0.,
                    };
                }
            };

            let mut rich_text = RichText::new(&full_text, converted_style);

            for TextNodeIterItem(range, style) in text_node.iter_rich_text(&styles) {
                rich_text.push_style(style, range);
            }

            let width = if let Number::Defined(width) = sz.width {
                width
            } else {
                unreachable!();
            };

            let paragraph_metrics =
                text_layout.compute_paragraph_layout(&rich_text, Pt(width as f64));

            let computed_height = paragraph_metrics.height.0;

            layout_paragraph_metrics_map.insert(node, paragraph_metrics);

            layout_rich_text_map.insert(node, rich_text);

            Size {
                width,
                height: computed_height as f32,
            }
        }))
    });

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

    if !relevant_things.borrow().measure_errors.is_empty() {
        drop(layout);

        // We can do this as long as layout is dropped by this point
        //  because all the references to measure errors should be dropped
        //  as part of it.
        let err = Rc::try_unwrap(relevant_things)
            .unwrap()
            .into_inner()
            .measure_errors
            .into_iter()
            .next()
            .unwrap();

        return Err(err);
    }

    {
        let mut pdf_writer = pdf_writer.borrow_mut();
        let page_writer = pdf_writer.get_page(0);

        let relevant_things = relevant_things.borrow();
        let RelevantThings {
            layout_rich_text_map,
            layout_paragraph_metrics_map,
            ..
        } = relevant_things.deref();

        for node in layout.draw_order() {
            let style = layout.get_style(node);
            let dom_node = layout.get_dom_node(node);

            match dom_node {
                DomNode::Text(text_node) => {
                    let rich_text = layout_rich_text_map.get(&node).unwrap();
                    let paragraph_metrics = layout_paragraph_metrics_map.get(&node).unwrap();

                    let layout_info = layout.get_layout(node)?;

                    page_writer
                        .write_lines(
                            Point::new(
                                Pt(layout_info.location.x as f64).into(),
                                // TODO: Lookup the page size instead of using
                                // this const.
                                A4.height //- Pt(layout_info.size.height as f64).into()
                                    - Pt(layout_info.location.y as f64).into(),
                            ),
                            rich_text,
                            &paragraph_metrics.line_metrics,
                        )
                        .unwrap();
                }
                _ => {}
            }
        }
    }

    drop(layout);

    Rc::try_unwrap(pdf_writer)
        .map_err(|_| BadPdfLayout::UnmatchedStyle {
            style_name: "".to_owned(),
        })?
        .into_inner()
        .save("output.pdf");

    Ok(())
}
