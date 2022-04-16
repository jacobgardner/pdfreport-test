use std::{fs::File, io::BufWriter};

use lazy_static::lazy_static;
use printpdf::{
    lopdf, IndirectFontRef, Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex,
    PdfLayerReference, PdfPageIndex, Point, Pt, Px, Rgb, Svg, SvgTransform, TextMatrix, calculate_points_for_circle,
};
use regex::Regex;
use skia_safe::Typeface;
use tracing::{instrument, span, Level};

use crate::{
    fonts::{find_font_index_by_style, FONTS},
    line_metric::LineMetric,
    rich_text::{FontWeight, RichText, RichTextStyle},
    text_layout::TextLayout,
};

const C: f64 = 0.551915024494;

pub struct PdfWriter {
    dimensions: (Mm, Mm),
    doc: PdfDocumentReference,
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
    fonts: Vec<IndirectFontRef>,
}

impl PdfWriter {
    #[instrument(name = "Create PDF Context")]
    pub fn new() -> Self {
        // A4 Page dimensions
        let dimensions = (Mm(210.), Mm(297.));

        let (doc, page1, layer1) =
            PdfDocument::new("Test Report", dimensions.0, dimensions.1, "Layer 1");

        let mut fonts = vec![];

        for font in FONTS {
            let font = doc.add_external_font(font.bytes).unwrap();
            fonts.push(font);
        }

        Self {
            dimensions,
            doc,
            pages: vec![(page1, layer1)],
            fonts,
        }
    }

    pub fn add_page(&mut self) -> PageWriter {
        let span = span!(Level::TRACE, "Adding Page To PDF");
        let _guard = span.enter();
        let (current_page_index, current_layer_index) = self.doc.add_page(
            self.dimensions.0,
            self.dimensions.1,
            format!("Test Page {}, Layer 1", self.pages.len() + 1),
        );

        self.pages.push((current_page_index, current_layer_index));

        PageWriter::new(self, current_page_index, current_layer_index)
    }

    pub fn get_page(&self, page_number: usize) -> PageWriter {
        let (current_page_index, current_layer_index) = self.pages[page_number];

        PageWriter::new(self, current_page_index, current_layer_index)
    }

    pub fn save(self, file_name: &str) {
        let span = span!(Level::TRACE, "Writing File");
        let _guard = span.enter();
        self.doc
            .save(&mut BufWriter::new(File::create(file_name).unwrap()))
            .unwrap()
    }
}

pub struct PageWriter<'a> {
    page_index: PdfPageIndex,
    layer_index: PdfLayerIndex,
    writer: &'a PdfWriter,
}

pub fn svg_to_pt(svg_unit: &str) -> Pt {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?i)(?P<quantity>[\.\d]+)(?P<units>cm|mm|Q|in|pc|pt|px)?$").unwrap();
    }

    let caps = RE.captures(svg_unit).unwrap();
    let quantity: f64 = caps.name("quantity").unwrap().as_str().parse().unwrap();
    let units = caps.name("units").map_or("px", |u| u.as_str());

    // TODO: Verify that these are correct, probably
    match units.to_lowercase().as_str() {
        "px" => Mm(quantity * (25.4 / 300.)).into(),
        "mm" => Mm(quantity).into(),
        "cm" => Mm(quantity * 10.0).into(),
        "pt" => Pt(quantity),
        "q" => Mm(quantity * 40.).into(),
        "in" => Mm(quantity / 25.4).into(),
        "pc" => Mm(quantity / 25.4 * 6.).into(),
        _ => panic!("Unknown unit types {units}"),
    }
}

impl<'a> PageWriter<'a> {
    fn new(writer: &'a PdfWriter, page_index: PdfPageIndex, layer_index: PdfLayerIndex) -> Self {
        Self {
            writer,
            page_index,
            layer_index,
        }
    }

    fn get_current_layer(&self) -> PdfLayerReference {
        self.writer
            .doc
            .get_page(self.page_index)
            .get_layer(self.layer_index)
    }

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
            let t = node.tag_name();

            // The SVG units are in px by default, and we're assuming that here.
            //  We have to convert that to Pt which printpdf has a method for, but it
            //  takes a usize, but the svg pixels can be fractions so... we replicate that
            //  here

            // TODO: Document/warn that we currently do NOT support text nodes in nested transformations
            //  OR add support for it.
            // TODO: Extract px => pt conversion
            // TODO: Support other unit types OR throw an error if provided
            // TODO: Document/warn about unsupported attributes
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

            let preferred_fonts: Vec<_> = font_stack.split(",").map(|f| f.trim()).collect();

            println!("Font Stack: {:?}", preferred_fonts);
            // We want to find a font in the stack that matches up to a loaded skia/pdf typeface.
            //   If we don't find one, default to the first typeface, probably?

            let is_centered = anchor.to_lowercase() == "middle";

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
                    italic: is_italic,
                    color: (fill.0 as f32, fill.1 as f32, fill.2 as f32),
                },
            );

            let paragraph = layout.compute_paragraph_layout(&rich, Pt(1000.0));

            assert_eq!(paragraph.line_metrics.len(), 1);

            let line_metric = paragraph.line_metrics.first().unwrap();

            println!("{:?}", line_metric);

            // current_layer.set_text_matrix(TextMatrix::Translate(start.x + Pt(x), start.y + Pt(y)));
            current_layer.set_text_matrix(TextMatrix::Translate(
                start.x + x.into() - line_metric.width / 2.,
                start.y + y.into() - ((line_metric.ascent - line_metric.descent) / 2.), // - ((line_metric.ascent - line_metric.baseline) / 2.),
            ));

            let font_idx = find_font_index_by_style(weight, is_italic);
            let current_font = &self.writer.fonts[font_idx];

            current_layer.set_font(current_font, Pt::from(font_size).0);
            current_layer
                .set_fill_color(printpdf::Color::Rgb(Rgb::new(fill.0, fill.1, fill.2, None)));

            PageWriter::write_text(&current_layer, &node_text, &layout.typeface);
        }

        self
    }

    pub fn draw_rect(&self, start: Point, end: Point, border_radius: Option<Pt>) -> &Self {
        let span = span!(Level::TRACE, "Drawing Rect");
        let _guard = span.enter();
        let current_layer = self.get_current_layer();
        
        let end = Point {x: end.x + Pt(3.), y: end.y - Pt(3.)};

        #[rustfmt::skip]
        let points = if let Some(border_radius) = border_radius {
            
            // 4 points per corner & 2 points per edge
            let mut points: Vec<(Point, bool)> = Vec::with_capacity(4 * 4 + 4 * 2);
            
            let circle_points = calculate_points_for_circle(border_radius, Pt(0.), Pt(0.));
            
            points.extend(circle_points[12..16].iter().map(|&(pt, b)| (Point { x: pt.x + border_radius + start.x, y: pt.y - border_radius + start.y}, b)));
            points.push((Point { x: start.x + border_radius, y: start.y, }, false));
            points.push((Point { x: end.x - border_radius, y: start.y, }, false));
            points.extend(circle_points[0..4].iter().map(|&(pt, b)| (Point { x: pt.x - border_radius + end.x, y: pt.y - border_radius + start.y}, b)));
            points.push((Point { x: end.x, y: start.y - border_radius, }, false));
            points.push((Point { x: end.x, y: end.y + border_radius, }, false));
            points.extend(circle_points[4..8].iter().map(|&(pt, b)| (Point { x: pt.x - border_radius + end.x, y: pt.y + border_radius + end.y}, b)));
            points.push((Point { x: end.x - border_radius, y: end.y, }, false));
            points.push((Point { x: start.x + border_radius, y: end.y , }, false));
            points.extend(circle_points[8..12].iter().map(|&(pt, b)| (Point { x: pt.x + border_radius + start.x, y: pt.y + border_radius + end.y}, b)));
            
            points

        } else {
            vec![
                (Point { x: start.x, y: start.y, }, false),
                (Point { x: end.x,   y: start.y, }, false,),
                (Point { x: end.x,   y: end.y },    false),
                (Point { x: start.x, y: end.y, },   false,),
            ]
        };

        current_layer.set_fill_color(printpdf::Color::Rgb(Rgb::new(0.8, 1., 0.8, None)));
        let line = Line {
            points,
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };

        current_layer.add_shape(line);

        self
    }

    // Borrowed from `printpdf`
    // Assumption: all styles of a typeface share the same glyph_ids
    fn encode_pdf_text(line: &str, typeface: &Typeface) -> Vec<u8> {
        let mut glyph_ids = vec![0; line.len()];
        typeface.str_to_glyphs(line, &mut glyph_ids);

        glyph_ids
            .iter()
            .flat_map(|x| vec![(x >> 8) as u8, (x & 255) as u8])
            .collect::<Vec<u8>>()
    }

    // This is more efficient than printpdf's write_line call
    //  because this uses Skia's much faster glyph lookup
    fn write_text(current_layer: &PdfLayerReference, line: &str, typeface: &Typeface) {
        let bytes = PageWriter::encode_pdf_text(line, typeface);

        current_layer.add_operation(lopdf::content::Operation::new(
            "Tj",
            vec![lopdf::Object::String(
                bytes,
                lopdf::StringFormat::Hexadecimal,
            )],
        ));
    }

    // TODO: Decouple this from Skia's structures
    pub fn write_lines(
        &self,
        start: Point,
        typeface: &Typeface,
        rich_text: &RichText,
        line_metrics: Vec<LineMetric>,
    ) -> &Self {
        let span = span!(Level::TRACE, "Writing Lines");
        let _guard = span.enter();
        let current_layer = self.get_current_layer();

        current_layer.begin_text_section();

        let mut style_iterator = rich_text.style_range_iter();
        let (mut current_range, mut current_style) = style_iterator.next().unwrap();

        let mut current_y = start.y;
        for line_metric in line_metrics {
            current_layer.set_text_matrix(TextMatrix::Translate(
                start.x + line_metric.left,
                current_y - line_metric.ascent,
            ));

            let mut current_index = line_metric.start_index;

            loop {
                let end_index = current_range.end.min(line_metric.end_index);
                let current_span = &rich_text.paragraph[current_index..end_index];

                current_index = end_index;

                let font_idx = find_font_index_by_style(current_style.weight, current_style.italic);
                let current_font = &self.writer.fonts[font_idx];

                current_layer.set_font(current_font, current_style.font_size.0);
                let clr = current_style.color;
                current_layer.set_fill_color(printpdf::Color::Rgb(Rgb::new(
                    clr.0 as f64,
                    clr.1 as f64,
                    clr.2 as f64,
                    None,
                )));

                PageWriter::write_text(&current_layer, current_span, typeface);

                if current_index == line_metric.end_index {
                    break;
                } else {
                    let (next_range, next_style) = style_iterator.next().unwrap();

                    current_range = next_range;
                    current_style = next_style;
                }
            }

            current_y -= line_metric.height;
        }
        current_layer.end_text_section();

        self
    }
}
