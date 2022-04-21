use printpdf::{Color, Point, Pt, Rgb, Svg, SvgTransform, TextMatrix};

use crate::{
    error::BadPdfLayout,
    fonts::FontLookup,
    line_metric::LineMetric,
    rich_text::{FontStyle, FontWeight, RichText, RichTextStyle},
    units::unit_to_pt,
};

use super::{GlyphLookup, PageWriter};

const SUPPORTED_TEXT_ATTRIBUTES: [&str; 10] = [
    "id",
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

impl<'a, T: GlyphLookup> PageWriter<'a, T> {
    pub fn draw_svg(
        &self,
        start: Point,
        svg_text: &str,
    ) -> Result<&Self, Box<dyn std::error::Error>> {
        // let string_to_path_svg = tree.to_string(&usvg::XmlOptions::default());
        // let text_node = tree.node_by_id("text");
        let current_layer = self.get_current_layer();

        let svg = Svg::parse(svg_text).unwrap();

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

        let doc = roxmltree::Document::parse(svg_text).unwrap();
        for node in doc.descendants().filter(|n| n.tag_name().name() == "text") {
            // The SVG units are in px by default, and we're assuming that here.
            //  We have to convert that to Pt which printpdf has a method for, but it
            //  takes a usize, but the svg pixels can be fractions so... we replicate that
            //  here
            //

            // TODO: Document/warn that we currently do NOT support text nodes in nested transformations
            //  OR add support for it.
            for _ancestor in node.ancestors() {
                // Here check if any ancestors do transformations and warn about
                // them OR augment the x/y below based on them
            }

            let unsupported_attribute = node
                .attributes()
                .iter()
                .find(|a| !SUPPORTED_TEXT_ATTRIBUTES.contains(&a.name().to_lowercase().as_str()));

            if let Some(unsupported_attribute) = unsupported_attribute {
                panic!(
                    "<text .../> attribute, {}, is not yet supported",
                    unsupported_attribute.name()
                );
            }

            let x = unit_to_pt(node.attribute("x").unwrap_or("0"))?;
            let y = unit_to_pt(node.attribute("y").unwrap_or("0"))?;
            let weight = FontWeight::from(node.attribute("font-weight").unwrap_or("regular"));
            let font_style = FontStyle::from(node.attribute("font-style").unwrap_or("normal"));
            let font_size = unit_to_pt(node.attribute("font-size").unwrap_or("12"))?;
            let fill =
                color_processing::Color::new_string(node.attribute("fill").unwrap_or("#000000"))
                    .unwrap()
                    .get_rgba();
            let anchor = node.attribute("text-anchor").unwrap_or("start");
            let font_stack = node.attribute("font-family").unwrap_or("sans-serif");
            let dominant_baseline = node.attribute("dominant-baseline").unwrap_or("auto");

            let found_font = self.find_best_font_from_stack(font_stack)?;

            // We want to find a font in the stack that matches up to a loaded skia/pdf typeface.
            //   If we don't find one, default to the first typeface, probably?

            // Once we have the correct typeface found, we should be able to use Skia to get the line_metrics
            //  for the string and compute what we need to compute for center/end alignment and vertical alignment
            // TODO: This^^

            // let layout = TextLayout::with_font_manager();
            if !node.children().all(|n| n.is_text()) {
                panic!("For <text>, we only support all text child nodes for now");
            }

            let node_text = node.text().unwrap().trim();
            let rich = RichText::new(
                node_text,
                RichTextStyle {
                    font_family: String::from(found_font),
                    font_size,
                    weight,
                    style: font_style,
                    color: (fill.0 as f32, fill.1 as f32, fill.2 as f32),
                },
            );

            // TODO: FIXME!!!!!
            // let paragraph = layout.compute_paragraph_layout(&rich, Pt(1000.0));

            // assert_eq!(paragraph.line_metrics.len(), 1);

            // let line_metric = paragraph.line_metrics.first().unwrap();
            let line_metric = LineMetric {
                start_index: 0,
                end_index: 10,
                ascent: Pt(0.),
                descent: Pt(1.),
                baseline: Pt(2.),
                height: Pt(3.),
                width: Pt(4.),
                left: Pt(5.),
            };

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
                start.x + x - x_offset,
                start.y + y - y_offset,
            ));

            let font_lookup = FontLookup {
                family_name: found_font,
                weight,
                style: font_style,
            };

            let current_font = self.writer.lookup_font(&font_lookup)?;

            current_layer.set_font(current_font, font_size.0);
            current_layer.set_fill_color(Color::Rgb(Rgb::new(fill.0, fill.1, fill.2, None)));

            self.write_text(&current_layer, node_text, &font_lookup)?;
        }

        Ok(self)
    }

    fn find_best_font_from_stack<'b>(
        &'a self,
        font_stack: &'b str,
    ) -> Result<&'b str, BadPdfLayout> {
        font_stack
            .split(',')
            .map(|f| f.trim())
            .find(|f| self.writer.font_families.contains_key(*f))
            .ok_or_else(|| BadPdfLayout::FontFamilyNotFound {
                font_family: String::from(font_stack),
            })
    }
}
