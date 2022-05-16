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
mod block_layout;
mod dom;
mod error;
mod fonts;
mod line_metric;
mod page_sizes;
mod pdf_writer;
mod resource_cache;
mod rich_text;
mod styles;
mod text_layout;
mod units;

const SVG: &str = include_str!("../assets/svg-test.svg");
const BASE_FONT_URL: &str =
    "https://github.com/jacobgardner/pdfreport-test/blob/main/assets/fonts/inter-static/";

// fn example_layout() -> PdfDom {
//     let re = Regex::new(r"(?i)Inter-(UI-)?(?P<weight>.*?)(?P<style>Italic)?(-BETA)?\.ttf").unwrap();

//     let fonts = std::fs::read_dir("./assets/fonts/inter-static")
//         .unwrap()
//         .map(|path| path.unwrap().file_name().into_string().unwrap())
//         .map(|filename| {
//             let captures = re.captures(&filename).unwrap();
//             let weight = captures.name("weight").unwrap().as_str();
//             let style = captures
//                 .name("style")
//                 .map(|m| m.as_str())
//                 .unwrap_or("Normal");

//             let weight = if weight.is_empty() { "Regular" } else { weight };

//             format!(
//                 r#"
//                 {{
//                     "source": "{BASE_FONT_URL}{filename}?raw=true",
//                     "weight": "{weight}",
//                     "style": "{style}"
//                 }}
//             "#
//             )
//         })
//         .join(",");

//     // Not using format! here because I don't want to escape every {{ }}
//     let json_str = r##"{
//         "fonts": [{
//             "family_name": "Inter",
//             "fonts": [
//     "##
//     .to_owned()
//         + &fonts
//         + r##"
//             ]
//         }],
//         "styles": {
//             "root": {
//                 "font": {
//                     "family": "Inter"
//                 },
//                 "padding": {
//                     "left": 20,
//                     "right": 20,
//                     "top": 20,
//                     "bottom": 20
//                 }
//             },
//             "container": {
//                 "flex": {
//                     "direction": "Row"
//                 }
//             },
//             "c1": {
//                 "flex": {
//                     "shrink": 1,
//                     "grow": 1
//                 }
//             },
//             "c2": {
//                 "flex": {
//                     "shrink": 1,
//                     "grow": 2
//                 }
//             },
//             "h1": {
//                 "color": "#A98765",
//                 "font": {
//                     "size": 20
//                 },
//                 "flex": {
//                     "direction": "Column"
//                 },
//                 "padding": {

//                 },
//                 "border": {
//                     "width": 1,
//                     "color": "#ABCDEF",
//                     "radius": {
//                         "topRight": 5,
//                         "bottomRight": 5
//                     }
//                 }
//             },
//             "italic": {
//                 "color": "pink",
//                 "font": {
//                     "size": 12,
//                     "style": "Italic"
//                 }
//             }
//         },
//         "root": {
//             "type": "Styled",
//             "styles": ["root"],
//             "children": [{
//                 "type": "Text",
//                 "styles": ["h1"],
//                 "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}]
//             },
//             {
//                 "type": "Styled",
//                 "styles": ["container"],
//                 "children": [
//                     {
//                         "type": "Text",
//                         "styles": ["h1", "c1"],
//                         "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}]
//                     },
//                     {
//                         "type": "Text",
//                         "styles": ["h1", "c2"],
//                         "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text that continues on to the next line if there is enough text to wrap??"]}]
//                     }
//                 ]
//             }]
//         }
//     }"##;

//     println!("{json_str}");

//     serde_json::from_str(&json_str).unwrap()
// }

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

    let json_file = std::fs::read_to_string("assets/example.json").unwrap();

    let pdf_layout = serde_json::from_str(&json_file).unwrap();

    assemble_pdf::assemble_pdf(&pdf_layout).await.unwrap();

    println!("Done assembling...");
}
