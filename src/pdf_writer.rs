use std::{fs::File, io::BufWriter};

use printpdf::{
    lopdf, Color, IndirectFontRef, Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex,
    PdfLayerReference, PdfPageIndex, Point, Rgb, TextMatrix,
};
use skia_safe::Typeface;
use tracing::{instrument, span, Level};

use crate::{fonts::{FONTS, find_font_index_by_style}, line_metric::LineMetric, rich_text::RichText};

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

    pub fn draw_rect(&self, start: Point, end: Point) -> &Self {
        let span = span!(Level::TRACE, "Drawing Rect");
        let _guard = span.enter();
        let current_layer = self.get_current_layer();

        // RustFmt kinda fucks up this if we let it do its thing
        #[rustfmt::skip]
        let points = vec![
            (Point { x: start.x, y: start.y, }, false),
            (Point { x: end.x,   y: start.y, }, false,),
            (Point { x: end.x,   y: end.y },    false),
            (Point { x: start.x, y: end.y, },   false,),
        ];

        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.8, 1., 0.8, None)));
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
                current_layer.set_fill_color(Color::Rgb(Rgb::new(clr.0 as f64, clr.1 as f64, clr.2 as f64, None)));

                PageWriter::write_text(&current_layer, current_span, typeface);

                if current_index == line_metric.end_index {
                    break;
                } else {
                    let (next_range, next_style) = style_iterator.next().unwrap();

                    current_range = next_range;
                    current_style = next_style;

                    // (current_range, current_style) = ;
                }
            }

            // let line_to_write = &rich_text.paragraph[line_metric.start_index..line_metric.end_index];

            current_y -= line_metric.height;
        }
        current_layer.end_text_section();

        self
    }
}
