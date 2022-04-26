// Compute layout -> Draw Shit -> Draw Text -> Profit
#![allow(dead_code)]

use std::rc::Rc;

use dom::PdfDom;
use itertools::Itertools;
use pdf_writer::PdfWriter;
use regex::Regex;
use text_layout::TextLayout;
use tracing::{span, Level};

use crate::{fonts::FontManager, text_layout::LayoutFonts};

mod assemble_pdf;
mod error;
mod fonts;
mod line_metric;
mod page_sizes;
mod pdf_writer;
mod resource_cache;
mod rich_text;
mod text_layout;
// mod paginated_layout;
mod block_layout;
mod dom;
mod styles;
mod units;

const SVG: &str = include_str!("../assets/svg-test.svg");
const BASE_FONT_URL: &str =
    "https://github.com/jacobgardner/pdfreport-test/blob/main/assets/fonts/inter-static/";

fn example_layout() -> PdfDom {
    let re = Regex::new(r"(?i)Inter-(UI-)?(?P<weight>.*?)(?P<style>Italic)?(-BETA)?\.ttf").unwrap();

    let fonts = std::fs::read_dir("./assets/fonts/inter-static")
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap())
        .map(|filename| {
            let captures = re.captures(&filename).unwrap();
            let weight = captures.name("weight").unwrap().as_str();
            let style = captures
                .name("style")
                .map(|m| m.as_str())
                .unwrap_or("Normal");

            let weight = if weight.is_empty() { "Regular" } else { weight };

            format!(
                r#"
                {{
                    "source": "{BASE_FONT_URL}{filename}?raw=true",
                    "weight": "{weight}",
                    "style": "{style}"
                }}
            "#
            )
        })
        .join(",");

    // Not using format! here because I don't want to escape every {{ }}
    let json_str = r##"{
        "fonts": [{
            "family_name": "Inter",
            "fonts": [
    "##
    .to_owned()
        + &fonts
        + r##"
            ]
        }],
        "styles": {
            "root": {
                "font": {
                    "family": "Inter" 
                },
                "padding": {
                    "left": 20,
                    "right": 20,
                    "top": 20,
                    "bottom": 20
                }
            },
            "container": {
             "flex": {
                    "direction": "Row"
                }               
            },
            "c1": {
                "flex": {
                    "shrink": 1,
                    "grow": 1
                }
            },
            "c2": {
                "flex": {
                    "shrink": 1,
                    "grow": 2
                }
            },
            "h1": {
                "color": "#A98765",
                "font": {
                    "size": 20
                },
                "flex": {
                    "direction": "Column"
                },
                "padding": {
                    "left": 0,
                    "right": 0
                },
                "border": {
                    "width": 1,  
                    "color": "#ABCDEF",
                    "radius": {
                        "topRight": 5,
                        "bottomRight": 5
                    }
                }
            },
            "italic": {
                "color": "pink",
                "font": {
                    "size": 12,
                    "style": "Italic"
                } 
            }
        },
        "root": {
            "type": "Styled",
            "styles": ["root"],
            "children": [{
                "type": "Text",
                "styles": ["h1"],
                "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}] 
            },
            {
                "type": "Styled",
                "styles": ["container"],
                "children": [ 
                    {
                        "type": "Text",
                        "styles": ["h1", "c1"],
                        "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}] 
                    },       
                    {
                        "type": "Text",
                        "styles": ["h1", "c2"],
                        "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}] 
                    }
                ]
            }]
        }
    }"##;

    serde_json::from_str(&json_str).unwrap()
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "trace_chrome")]
    let _guard = {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::prelude::*;
        let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        _guard
    };

    let fm = FontManager::new();
    let layout_fonts = Rc::new(LayoutFonts::with_font_manager(&fm));
    // let pdf_writer = PdfWriter::new(&fm, layout_fonts.clone(), LETTER);

    let _text_layout = TextLayout::new(layout_fonts);

    let span = span!(Level::DEBUG, "Full Time");
    let _guard = span.enter();

    // let mut _page_writer = pdf_writer.get_page(0);

    let _layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();

    let pdf_layout = example_layout();

    assemble_pdf::assemble_pdf(&pdf_layout).await.unwrap();

    println!("Done assembling...");

    // {
    //     let text_compute: TextComputeFn = Box::new(|text_node: &TextNode| {
    //         let text_node = text_node.clone();
    //         MeasureFunc::Boxed(Box::new(move |_sz| {
    //             println!("{:?}", text_node.styles);
    //             Size {
    //                 width: 32.,
    //                 height: 32.,
    //             }
    //         }))
    //     });

    //     let image_compute: ImageComputeFn = Box::new(|_image_node| {
    //         MeasureFunc::Raw(move |_sz| Size {
    //             width: 32.,
    //             height: 32.,
    //         })
    //     });

    //     let _layout = BlockLayout::build_layout(&pdf_layout, text_compute, image_compute).unwrap();
    // }

    // let page_count = 1;

    // for i in 0..page_count {
    //     let default_style = RichTextStyle {
    //         font_size: Pt(14.),
    //         weight: rich_text::FontWeight::Regular,
    //         is_italic: false,
    //         color: (0.267, 0.29, 0.353),
    //     };

    //     let mut rich_text = RichText::new(&output_text[i..], default_style);

    //     rich_text
    //         .push_style(
    //             RichTextStyleChanges {
    //                 font_size: Some(Pt(32.)),
    //                 weight: Some(rich_text::FontWeight::Bold),
    //                 ..Default::default()
    //             },
    //             0..32,
    //         )
    //         .push_style(
    //             RichTextStyleChanges {
    //                 color: Some((i as f32 / page_count as f32, 0., 0.)),
    //                 italic: Some(true),
    //                 ..Default::default()
    //             },
    //             16..32,
    //         );

    //     // // We have to change the string every time otherwise Skia caches the
    //     // // layout calculation and we cheat in performance
    //     // let page_string = &output_string[i..];

    //     // 20 Mm of padding on left & right
    //     let text_width = Mm(210. - 40.).into_pt();

    //     let paragraph_metrics = text_layout.compute_paragraph_layout(&rich_text, text_width);

    //     page_writer
    //         .draw_rect(
    //             Point::new(Mm(20.), Mm(280.)),
    //             Point::new(
    //                 Mm(20.) + text_width.into(),
    //                 Mm(280.) - paragraph_metrics.height.into(),
    //             ),
    //             Some(Pt(5.)),
    //         )
    //         .write_lines(
    //             Point::new(Mm(20.), Mm(280.)),
    //             &text_layout.typeface,
    //             &rich_text,
    //             paragraph_metrics.line_metrics,
    //         );

    //     page_writer
    //         .draw_svg(Point::new(Mm(21.), Mm(270.)), SVG)
    //         .unwrap();

    //     page_writer = pdf_writer.add_page();
    // }
    // layout_span.exit();

    // pdf_writer.save("output.pdf");
}
