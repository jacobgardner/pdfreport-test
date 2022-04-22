// Compute layout -> Draw Shit -> Draw Text -> Profit
#![allow(dead_code)]

use std::rc::Rc;

use dom::PdfDom;
use pdf_writer::PdfWriter;
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

fn build_text() -> String {
    r#"Chapter 1: Your approach to Work

Introduction

Why this matters

Your results indicate that you are moderate in Decision-Making. This indicates that you are likely to be generally effective
when it comes to making decisions. You are likely to spend
an adequate amount of time gathering and analyzing available
information, weighing risks, and plotting a course to follow.
In addition to this, you might approach many decisions with a
somewhat open mind to new directions. Finally, you might be
generally effective at considering potential outcomes of your
decisions prior to making them.
While you are likely to be effective in most day-to-day decision-making situations, you might have difficulty when presented with especially complex decisions. These difficulties
might take the form of being overwhelmed by options, spending too much time analyzing information, or making decisions
based on incomplete information.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
EXPLAIN THE GROUPINGS
Here's a quick snapshot of what these three groupings mean.
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc
viverra ligula nec posuere dictum. Curabitur varius erat eget
nisi sodales, nec auctor leo placerat. Vivamus sit amet porttitor urna, sed rutrum augue. Donec bibendum lacus vel felis
viverra, a auctor orci vestibulum. Nulla et erat ac sem gravida
lacinia. Maecenas molestie orci et augue convallis, ut vehicula ante fermentum. Curabitur nec blandit arcu. Suspendisse
potenti. Suspendisse leo arcu, aliquam in porttitor scelerisque,
condimentum id quam. Nulla facilisi. Vivamus id urna ipsum.
Nulla non sapien leo. Vestibulum non ligula sapien. Nullam
finibus nibh a massa auctor sodales. Nulla mauris neque,
bibendum vel velit id, aliquam pharetra neque. Nam ultricies
posuere dolor eget congue.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
HOW TO USE IT
Explaining how every measure should be completed, but allows for the user to browse at their own pace and leisure.
They're welcome to scroll through, but a deep dive is necessary. At the end of each measure the users will have two
actions to take: adding items to their action planner or marking
the measure as complete.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
Decision Making
WHY THIS MATTERS
    "#
    .replace('\n', " ")
    .replace("  ", " ")
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

    let _output_text = build_text();

    let fm = FontManager::new();
    let layout_fonts = Rc::new(LayoutFonts::with_font_manager(&fm));
    let pdf_writer = PdfWriter::new(&fm, layout_fonts.clone());

    let _text_layout = TextLayout::new(layout_fonts);

    let span = span!(Level::DEBUG, "Full Time");
    let _guard = span.enter();

    let mut _page_writer = pdf_writer.get_page(0);

    let _layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();

    let pdf_layout: PdfDom = serde_json::from_str(
        r##"{
            "fonts": [{
                "family_name": "Inter",
                "fonts": [
                    {
                        "source": "https://github.com/jacobgardner/pdfreport-test/blob/main/assets/fonts/inter-static/Inter-Black.ttf?raw=true",
                        "weight": "Black"
                    },
                    {
                        "source": "https://github.com/jacobgardner/pdfreport-test/blob/main/assets/fonts/inter-static/Inter-Bold.ttf?raw=true",
                        "weight": "Bold"
                    }
                ]
            }],
            "styles": {
                "root": {
                    "font": {
                        "family": "Inter" 
                    }
                },
                "h1": {
                    "color": "#ABCDEF",
                    "flex": {
                        "direction": "Column"
                    },
                    "margin": {
                        "bottom": 4
                    },
                    "padding": {
                        "left": 40,
                        "right": 40
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
                     
                }
            },
            "root": {
                "type": "Styled",
                "styles": ["root"],
                "children": [{
                    "type": "Text",
                    "styles": ["h1"],
                    "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text"]}] 
                }, {
                    "type": "Image",
                    "content": "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\" role=\"img\" aria-label=\"22\" width=\"73\" height=\"73\" viewBox=\"0 0 73 73\" xmlns=\"http://www.w3.org/2000/svg\"><circle class=\"donutMetric__innerCircle\" cx=\"36.5\" cy=\"36.5\" r=\"25\" fill=\"#D3D1E6\" /></svg>"
                }]
            }
        }"##,
    )
    .unwrap();

    println!("Layout: {pdf_layout:?}");

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
