use std::{collections::HashMap, fs::File, io::BufWriter, ops::Range};

use printpdf::{
    calculate_points_for_circle, lopdf, Color, IndirectFontRef, Line, Mm, PdfDocument,
    PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex, PdfPageReference, Point,
    Pt, Rgb, TextMatrix,
};
use skia_safe::Typeface;
use tracing::{instrument, span, Level};

mod svg;

use crate::{
    error::BadPdfLayout,
    fonts::{FontLookup, FontManager},
    line_metric::LineMetric,
    rich_text::{FontStyle, FontWeight, RichText},
};

#[derive(Hash, Eq, PartialEq)]
struct FontKey {
    weight: FontWeight,
    style: FontStyle,
}

pub struct PdfWriter {
    dimensions: (Mm, Mm),
    doc: PdfDocumentReference,
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
    font_families: HashMap<String, HashMap<FontKey, IndirectFontRef>>, // fonts: Vec<IndirectFontRef>,
}

const TOP_LEFT_CORNER: Range<usize> = 12..16;
const TOP_RIGHT_CORNER: Range<usize> = 0..4;
const BOTTOM_RIGHT_CORNER: Range<usize> = 4..8;
const BOTTOM_LEFT_CORNER: Range<usize> = 8..12;

impl PdfWriter {
    #[instrument(name = "Create PDF Context")]
    pub fn new(font_manager: &FontManager) -> Self {
        // A4 Page dimensions
        let dimensions = (Mm(210.), Mm(297.));

        let font_families = HashMap::new();

        let (doc, page1, layer1) =
            PdfDocument::new("Test Report", dimensions.0, dimensions.1, "Layer 1");

        for (family_name, font_family) in font_manager.families.iter() {
            let font_family_fonts = HashMap::new();

            for font in font_family.fonts.iter() {
                let indirect_font_ref = doc.add_external_font((*font.bytes).as_ref()).unwrap();

                font_family_fonts.insert(
                    FontKey {
                        weight: font.weight,
                        style: font.style,
                    },
                    indirect_font_ref,
                );
            }
        }

        Self {
            dimensions,
            doc,
            pages: vec![(page1, layer1)],
            font_families,
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

    fn lookup_font(&self, font_lookup: &FontLookup) -> Result<&IndirectFontRef, BadPdfLayout> {
        let family = self
            .font_families
            .get(font_lookup.family_name)
            .ok_or_else(|| BadPdfLayout::FontFamilyNotFound {
                font_family: String::from(font_lookup.family_name),
            })?;

        family
            .get(&FontKey {
                weight: font_lookup.weight,
                style: font_lookup.style,
            })
            .ok_or_else(|| BadPdfLayout::FontStyleNotFoundForFamily {
                font_family: String::from(font_lookup.family_name),
                font_weight: font_lookup.weight,
                font_style: font_lookup.style,
            })
    }
}

pub struct PageWriter<'a> {
    page_index: PdfPageIndex,
    layer_index: PdfLayerIndex,
    writer: &'a PdfWriter,
}

impl<'a> PageWriter<'a> {
    fn new(writer: &'a PdfWriter, page_index: PdfPageIndex, layer_index: PdfLayerIndex) -> Self {
        Self {
            writer,
            page_index,
            layer_index,
        }
    }

    fn get_current_page(&self) -> PdfPageReference {
        self.writer.doc.get_page(self.page_index)
    }

    fn get_current_layer(&self) -> PdfLayerReference {
        self.get_current_page().get_layer(self.layer_index)
    }

    pub fn draw_rect(&self, start: Point, end: Point, border_radius: Option<Pt>) -> &Self {
        let span = span!(Level::TRACE, "Drawing Rect");
        let _guard = span.enter();
        let current_layer = self.get_current_layer();

        let start = Point {
            x: start.x - Pt(3.0),
            y: start.y + Pt(3.0),
        };

        let end = Point {
            x: end.x + Pt(3.0),
            y: end.y - Pt(3.0),
        };

        #[rustfmt::skip]
        let points = match border_radius {
            Some(border_radius) if border_radius != Pt(0.) => {
                // 4 points per corner & 2 points per edge
                let mut points: Vec<(Point, bool)> = Vec::with_capacity(4 * 4 + 4 * 2);

                let circle_points = calculate_points_for_circle(border_radius, Pt(0.), Pt(0.));

                points.extend(circle_points[TOP_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + border_radius + start.x, y: pt.y - border_radius + start.y}, b)));
                points.push((Point { x: start.x + border_radius, y: start.y, }, false));
                points.push((Point { x: end.x - border_radius, y: start.y, }, false));
                points.extend(circle_points[TOP_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - border_radius + end.x, y: pt.y - border_radius + start.y}, b)));
                points.push((Point { x: end.x, y: start.y - border_radius, }, false));
                points.push((Point { x: end.x, y: end.y + border_radius, }, false));
                points.extend(circle_points[BOTTOM_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - border_radius + end.x, y: pt.y + border_radius + end.y}, b)));
                points.push((Point { x: end.x - border_radius, y: end.y, }, false));
                points.push((Point { x: start.x + border_radius, y: end.y , }, false));
                points.extend(circle_points[BOTTOM_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + border_radius + start.x, y: pt.y + border_radius + end.y}, b)));

                points
            },
            _ => {
                vec![
                    (Point { x: start.x, y: start.y, }, false),
                    (Point { x: end.x,   y: start.y, }, false),
                    (Point { x: end.x,   y: end.y    }, false),
                    (Point { x: start.x, y: end.y,   }, false),
                ]
            }
        };

        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.8, 1., 0.8, None)));
        let line = Line {
            points,
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };

        current_layer.save_graphics_state();
        current_layer.set_outline_thickness(2.);
        current_layer.set_outline_color(Color::Rgb(Rgb {
            r: 1.0,
            g: 0.,
            b: 1.,
            icc_profile: None,
        }));

        current_layer.add_shape(line);
        current_layer.restore_graphics_state();

        self
    }

    // Borrowed from `printpdf`
    // Assumption: all styles of a typeface share the same glyph_ids
    fn encode_pdf_text(&self, line: &str, font_lookup: &FontLookup) -> Vec<u8> {
        let mut glyph_ids = vec![0; line.len()];
        typeface.str_to_glyphs(line, &mut glyph_ids);

        glyph_ids
            .iter()
            .flat_map(|x| vec![(x >> 8) as u8, (x & 255) as u8])
            .collect::<Vec<u8>>()
    }

    // This is more efficient than printpdf's write_line call
    //  because this uses Skia's much faster glyph lookup
    fn write_text(&self, current_layer: &PdfLayerReference, line: &str, font_lookup: &FontLookup) {
        let bytes = self.encode_pdf_text(line, font_lookup);

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
    ) -> Result<&Self, BadPdfLayout> {
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

                // TODO: Look up appropriate font
                // let font_idx =
                //     find_font_index_by_style(current_style.weight, current_style.is_italic);
                // let font_idx = 0;
                // let current_font = &self.writer.fonts[font_idx];

                let font_lookup = FontLookup {
                    family_name: "Inter",
                    weight: current_style.weight,
                    style: current_style.style,
                };

                let current_font = self.writer.lookup_font(&font_lookup)?;

                current_layer.set_font(current_font, current_style.font_size.0);
                let clr = current_style.color;
                current_layer.set_fill_color(Color::Rgb(Rgb::new(
                    clr.0 as f64,
                    clr.1 as f64,
                    clr.2 as f64,
                    None,
                )));

                self.write_text(&current_layer, current_span, &font_lookup);

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

        Ok(self)
    }
}
