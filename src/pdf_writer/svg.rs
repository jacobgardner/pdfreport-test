use lazy_static::lazy_static;
use printpdf::{Color, Mm, Point, Pt, Rgb, Svg, SvgTransform, TextMatrix};
use regex::Regex;

use crate::{
    fonts::find_font_index_by_style,
    rich_text::{FontWeight, RichText, RichTextStyle},
    text_layout::TextLayout,
};

use super::PageWriter;

const SUPPORTED_TEXT_ATTRIBUTES: [&str; 9] = [
    "x",
    "y",
    "font-weight",
    "font-style",
    "font-size",
    "fill",
    "text-anchor",
    "font-family",
    "dominant-baseline",
];

fn svg_to_pt(svg_unit: &str) -> Pt {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?i)(?P<quantity>[\.\d]+)(?P<units>\D+)?$").unwrap();
    }

    let caps = RE.captures(svg_unit).unwrap();
    let quantity: f64 = caps.name("quantity").unwrap().as_str().parse().unwrap();
    let units = caps.name("units").map_or("px", |u| u.as_str());

    match units.to_lowercase().as_str() {
        "px" => Mm(quantity * (25.4 / 300.)).into(),
        "mm" => Mm(quantity).into(),
        "cm" => Mm(quantity * 10.0).into(),
        "pt" => Pt(quantity),
        "in" => Pt(quantity * 72.),
        "pc" => Pt(quantity * 6.).into(),
        _ => panic!("Unknown unit types {units}"),
    }
}

impl<'a> PageWriter<'a> {
    pub fn draw_svg(&self, start: Point, svg_text: &str) -> &Self {
        // let string_to_path_svg = tree.to_string(&usvg::XmlOptions::default());
        // let text_node = tree.node_by_id("text");
        let current_layer = self.get_current_layer();

        let svg = Svg::parse(&svg_text).unwrap();

        let svg_ref = svg.into_xobject(&current_layer);

        svg_ref.add_to_layer(
            &current_layer,
            SvgTransform {
                translate_x: Some(start.x),
                translate_y: Some(start.y),
                scale_x: Some(1.0),
                scale_y: Some(1.0),
                ..Default::default()
            },
        );

        let doc = roxmltree::Document::parse(&svg_text).unwrap();
        for node in doc.descendants().filter(|n| n.tag_name().name() == "text") {
            // The SVG units are in px by default, and we're assuming that here.
            //  We have to convert that to Pt which printpdf has a method for, but it
            //  takes a usize, but the svg pixels can be fractions so... we replicate that
            //  here
            //

            // TODO: Document/warn that we currently do NOT support text nodes in nested transformations
            //  OR add support for it.
            for ancestor in node.ancestors() {}

            let unsupported_attribute = node
                .attributes()
                .iter()
                .find(|a| !SUPPORTED_TEXT_ATTRIBUTES.contains(&a.name().to_lowercase().as_str()));

            if let Some(unsupported_attribute) = unsupported_attribute {
                panic!("<text .../> attribute, {}, is not yet supported", unsupported_attribute.name());
            }

            let x = svg_to_pt(node.attribute("x").unwrap_or("0"));
            let y = svg_to_pt(node.attribute("y").unwrap_or("0"));
            let weight = FontWeight::from(node.attribute("font-weight").unwrap_or("regular"));
            let font_style = node.attribute("font-style").unwrap_or("normal");
            let is_italic = font_style.to_lowercase() == "italic";
            let font_size = svg_to_pt(node.attribute("font-size").unwrap_or("12"));
            let fill =
                color_processing::Color::new_string(node.attribute("fill").unwrap_or("#000000"))
                    .unwrap()
                    .get_rgba();
            let anchor = node.attribute("text-anchor").unwrap_or("start");
            let font_stack = node.attribute("font-family").unwrap_or("sans-serif");
            let dominant_baseline = node.attribute("dominant-baseline").unwrap_or("auto");

            let preferred_fonts: Vec<_> = font_stack.split(",").map(|f| f.trim()).collect();

            println!("Font Stack: {:?}", preferred_fonts);
            // We want to find a font in the stack that matches up to a loaded skia/pdf typeface.
            //   If we don't find one, default to the first typeface, probably?

            // Once we have the correct typeface found, we should be able to use Skia to get the line_metrics
            //  for the string and compute what we need to compute for center/end alignment and vertical alignment
            // TODO: This^^

            let layout = TextLayout::new();
            if !node.children().all(|n| n.is_text()) {
                panic!("For <text>, we only support all text child nodes for now");
            }

            let node_text = node.text().unwrap().trim();
            let rich = RichText::new(
                node_text,
                RichTextStyle {
                    font_size,
                    weight,
                    is_italic,
                    color: (fill.0 as f32, fill.1 as f32, fill.2 as f32),
                },
            );

            let paragraph = layout.compute_paragraph_layout(&rich, Pt(1000.0));

            assert_eq!(paragraph.line_metrics.len(), 1);

            let line_metric = paragraph.line_metrics.first().unwrap();

            let x_offset = match anchor.to_lowercase().as_str() {
                "start" => Pt(0.0),
                "middle" => line_metric.width / 2.,
                "end" => line_metric.width,
                _ => panic!(""),
            };

            let y_offset = match dominant_baseline.to_lowercase().as_str() {
                "auto" => Pt(0.),
                // TODO: This is wrong, but good enough for initial testing...
                "middle" | "central" => (line_metric.ascent - line_metric.descent) / 2.,
                baseline => panic!("{} as dominant-baseline is not yet supported", baseline),
            };

            current_layer.set_text_matrix(TextMatrix::Translate(
                start.x + x.into() - x_offset,
                start.y + y.into() - y_offset,
            ));

            let font_idx = find_font_index_by_style(weight, is_italic);
            let current_font = &self.writer.fonts[font_idx];

            current_layer.set_font(current_font, Pt::from(font_size).0);
            current_layer.set_fill_color(Color::Rgb(Rgb::new(fill.0, fill.1, fill.2, None)));

            PageWriter::write_text(&current_layer, &node_text, &layout.typeface);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_to_pt_px() {
        // pixels * 72 / DPI = pt
        assert_eq!(svg_to_pt("50"), Pt(12.0));
        assert_eq!(svg_to_pt("50px"), Pt(12.0));

        assert_eq!(svg_to_pt("100"), Pt(24.0));
        assert_eq!(svg_to_pt("100px"), Pt(24.0));
    }

    #[test]
    fn test_svg_to_pt_mm() {
        // Taken from a lookup table
        assert_eq!(svg_to_pt("50mm"), Pt(141.7322834646));
        assert_eq!(svg_to_pt("20mm"), Pt(56.6929133858));
    }

    #[test]
    fn test_svg_to_pt_cm() {
        // Taken from a lookup table
        assert_eq!(svg_to_pt("5cm"), Pt(141.7322834646));
        assert_eq!(svg_to_pt("2cm"), Pt(56.6929133858));
    }

    #[test]
    fn test_svg_to_pt_pt() {
        // 1:1
        assert_eq!(svg_to_pt("5pt"), Pt(5.));
        assert_eq!(svg_to_pt("2pt"), Pt(2.));
        assert_eq!(svg_to_pt("12pt"), Pt(12.));
        assert_eq!(svg_to_pt("12.5pt"), Pt(12.5));
    }

    #[test]
    fn test_svg_to_pt_in() {
        // 1:72
        assert_eq!(svg_to_pt("5in"), Pt(360.));
        assert_eq!(svg_to_pt("2in"), Pt(144.));
        assert_eq!(svg_to_pt("12in"), Pt(864.));
    }

    #[test]
    fn test_svg_to_pt_pc() {
        // 1:6
        assert_eq!(svg_to_pt("5pc"), Pt(30.));
        assert_eq!(svg_to_pt("2pc"), Pt(12.));
        assert_eq!(svg_to_pt("12pc"), Pt(72.));
    }

    #[test]
    #[should_panic(expected = "Unknown unit types rem")]
    fn test_unsupported_unit() {
        svg_to_pt("5rem");
    }
}
