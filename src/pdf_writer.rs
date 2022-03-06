use std::{fs::File, io::BufWriter};

use printpdf::{
    lopdf, Color, IndirectFontRef, Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex,
    PdfLayerReference, PdfPageIndex, Point, Pt, Rgb, TextMatrix,
};
use skia_safe::Typeface;
use tracing::{instrument, span, Level};

use crate::{fonts::FONTS, line_metric::LineMetric};

pub struct PdfWriter {
    dimensions: (Mm, Mm),
    doc: PdfDocumentReference,
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
    fonts: Vec<IndirectFontRef>,
}

impl PdfWriter {
    #[instrument(name = "Create PDF Context")]
    pub fn new() -> Self {
        let dimensions = (Mm(210.), Mm(297.));

        let (doc, page1, layer1) =
            PdfDocument::new("Test Report", dimensions.0, dimensions.1, "Layer 1");

        let mut fonts = vec![];

        for font in FONTS {
            let font = doc.add_external_font(font).unwrap();
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

        let line = Line {
            points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        current_layer.add_shape(line);

        self
    }

    // TODO: Decouple this from Skia's line metrics
    pub fn write_lines(
        &self,
        start: Point,
        typeface: &Typeface,
        string_to_write: &str,
        line_metrics: Vec<LineMetric>,
    ) -> &Self {
        let span = span!(Level::TRACE, "Writing Lines");
        let _guard = span.enter();
        let current_layer = self.get_current_layer();

        let font = self.writer.fonts[6].clone();

        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.267, 0.29, 0.353, None)));

        let mut current_y = start.y;
        for line_metric in line_metrics {
            current_layer.set_text_matrix(TextMatrix::Translate(
                start.x,
                current_y - Pt(line_metric.height),
            ));

            let line_to_write = &string_to_write[line_metric.start_index..line_metric.end_index];

            let mut glyph_ids = vec![0; line_to_write.len()];
            typeface.str_to_glyphs(line_to_write, &mut glyph_ids);

            let bytes = glyph_ids
                .iter()
                .flat_map(|x| vec![(x >> 8) as u8, (x & 255) as u8])
                .collect::<Vec<u8>>();

            current_layer.add_operation(lopdf::content::Operation::new(
                "Tj",
                vec![lopdf::Object::String(
                    bytes,
                    lopdf::StringFormat::Hexadecimal,
                )],
            ));

            current_y -= Pt(line_metric.height);
        }
        current_layer.end_text_section();

        self
    }
}
