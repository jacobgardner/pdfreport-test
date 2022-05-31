//! This is ultimately what takes nodes that have been styled
//!  and laid out and writes them to a PDF.
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{BufWriter, Write},
    ops::Range,
    rc::Rc,
};

use printpdf::{
    calculate_points_for_circle, IndirectFontRef, PdfDocument, PdfDocumentReference, PdfLayerIndex,
    PdfLayerReference, PdfPageIndex, Point, TextMatrix, Line,
};

use crate::{
    document_builder::UnstructuredDocumentWriter,
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontCollection, FontId},
    paragraph_layout::RenderedTextBlock,
    rich_text::RichTextSpan,
    stylesheet::BorderRadiusStyle,
    values::{Color, Mm, Pt, Rect, Size},
};

#[derive(Clone, Default)]
struct CurrentStyles {
    font_id: Option<FontId>,
    font_size: Option<Pt>,

    color: Option<Color>,
}

const TOP_LEFT_CORNER: Range<usize> = 12..16;
const TOP_RIGHT_CORNER: Range<usize> = 0..4;
const BOTTOM_RIGHT_CORNER: Range<usize> = 4..8;
const BOTTOM_LEFT_CORNER: Range<usize> = 8..12;

struct FontLookup(RefCell<HashMap<FontId, Rc<IndirectFontRef>>>);

impl FontLookup {
    fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    fn get(&self, font_id: FontId) -> Option<Rc<IndirectFontRef>> {
        self.0.borrow().get(&font_id).cloned()
    }

    fn insert(&self, font_id: FontId, font_ref: IndirectFontRef) {
        self.0.borrow_mut().insert(font_id, Rc::new(font_ref));
    }

    fn insert_and_get(&self, font_id: FontId, font_ref: IndirectFontRef) -> Rc<IndirectFontRef> {
        self.insert(font_id, font_ref);

        self.get(font_id)
            .expect("We just inserted it. It has to exist")
    }
}

pub struct PrintPdfWriter<'a> {
    raw_pdf_doc: PdfDocumentReference,
    fonts: FontLookup,
    page_layer_indices: Vec<(PdfPageIndex, Vec<PdfLayerIndex>)>,
    font_collection: &'a FontCollection,

    current_style_by_page: Vec<CurrentStyles>,
}

impl<'a> PrintPdfWriter<'a> {
    pub fn new(
        doc_title: &str,
        page_size: impl Into<Size<Mm>>,
        font_collection: &'a FontCollection,
    ) -> Self {
        let dimensions = page_size.into();

        let (doc, page_index, layer_index) = PdfDocument::new(
            doc_title,
            dimensions.width.into(),
            dimensions.height.into(),
            "Layer 1",
        );

        Self {
            raw_pdf_doc: doc,
            fonts: FontLookup::new(),
            page_layer_indices: vec![(page_index, vec![layer_index])],
            font_collection,
            current_style_by_page: vec![CurrentStyles::default()],
        }
    }

    pub fn get_font(
        &self,
        font_id: FontId,
    ) -> Result<Rc<IndirectFontRef>, DocumentGenerationError> {
        if let Some(font) = self.fonts.get(font_id) {
            Ok(font)
        } else {
            let font_data = self
                .font_collection
                .get_font(font_id)
                .ok_or(InternalServerError::FontIdNotLoaded)?;

            let font_ref = self
                .raw_pdf_doc
                .add_external_font(font_data.as_bytes())
                .map_err(|e| InternalServerError::LoadFontError {
                    source: Box::new(e),
                    family_name: font_data.family_name().to_owned(),
                    attributes: *font_data.attributes(),
                })?;

            Ok(self.fonts.insert_and_get(font_data.font_id(), font_ref))
        }
    }

    pub fn save<W: Write>(
        self,
        pdf_doc_writer: W,
    ) -> Result<W, crate::error::DocumentGenerationError> {
        let mut buf_writer = BufWriter::new(pdf_doc_writer);

        self.raw_pdf_doc.save(&mut buf_writer).unwrap();

        let write_result = buf_writer
            .into_inner()
            .map_err(|e| InternalServerError::WritePdfError(e.into()));

        Ok(write_result?)
    }
}

impl<'a> UnstructuredDocumentWriter for PrintPdfWriter<'a> {
    fn write_text_block(
        &mut self,
        text_block: RenderedTextBlock,
        position: crate::values::Point<Pt>,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let page_number = 0;

        let layer = self.get_base_layer(page_number);

        layer.begin_text_section();

        let x = printpdf::Pt::from(position.x);
        let y = printpdf::Pt::from(position.y);

        let mut current_y = y;
        for line in text_block.lines.iter() {
            layer.set_text_matrix(TextMatrix::Translate(
                x + line.line_metrics.left.into(),
                current_y - line.line_metrics.ascent.into(),
            ));

            for span in line.rich_text.0.iter() {
                let font = self.set_base_layer_style(page_number, &layer, &span)?;

                layer.write_text(span.text.clone(), font.as_ref());
            }

            current_y -= line.line_metrics.height.into();
        }

        layer.end_text_section();

        Ok(self)
    }

    fn draw_rect(
        &mut self,
        rect: Rect<Pt>,
        border_width: Pt,
        border_color: Option<Color>,
        background_color: Option<Color>,
        border_radius: Option<BorderRadiusStyle::Unmergeable>,
    ) {
        let layer = self.get_base_layer(0);



        let start = Point {
            x: rect.left.into(),
            y: rect.top.into(),
        };

        let end = Point {
            x: (rect.left + rect.width).into(),
            y: (rect.top - rect.height).into(),
        };

        #[rustfmt::skip]
        let points = match border_radius {
            Some(border_radius) if border_radius != BorderRadiusStyle::Unmergeable::default() => {
                // 4 points per corner & 2 points per edge
                let mut points: Vec<(printpdf::Point, bool)> = Vec::with_capacity(4 * 4 + 4 * 2);

                // TODO: Skip any corners where the radius is 0
                // TODO: Optimization: Don't recalculate corners that have matching radius
                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.top_left), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[TOP_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + printpdf::Pt(border_radius.top_left) + start.x, y: pt.y - printpdf::Pt(border_radius.top_left) + start.y}, b)));
                points.push((Point { x: start.x + printpdf::Pt(border_radius.top_left), y: start.y, }, false));
                points.push((Point { x: end.x - printpdf::Pt(border_radius.top_left), y: start.y, }, false));

                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.top_right), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[TOP_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - printpdf::Pt(border_radius.top_right) + end.x, y: pt.y - printpdf::Pt(border_radius.top_right) + start.y}, b)));
                points.push((Point { x: end.x, y: start.y - printpdf::Pt(border_radius.top_right), }, false));
                points.push((Point { x: end.x, y: end.y + printpdf::Pt(border_radius.top_right), }, false));

                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.bottom_right), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[BOTTOM_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - printpdf::Pt(border_radius.bottom_right) + end.x, y: pt.y + printpdf::Pt(border_radius.bottom_right) + end.y}, b)));
                points.push((Point { x: end.x - printpdf::Pt(border_radius.bottom_right), y: end.y, }, false));
                points.push((Point { x: start.x + printpdf::Pt(border_radius.bottom_right), y: end.y , }, false));
                
                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.bottom_left), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[BOTTOM_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + printpdf::Pt(border_radius.bottom_left) + start.x, y: pt.y + printpdf::Pt(border_radius.bottom_left) + end.y}, b)));

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
        
        layer.save_graphics_state();
        let line = Line {
            points,
            is_closed: true,
            has_fill: background_color.is_some(),
            has_stroke: border_color.is_some(),
            is_clipping_path: false,
        };

        if let Some(color) = border_color {
            layer.set_outline_color(color.into());
        }

        if let Some(color) = background_color {
            layer.set_fill_color(color.into());
        }

        layer.set_outline_thickness(border_width.0);

        layer.add_shape(line);
        
        layer.restore_graphics_state();



    }
}

impl<'a> PrintPdfWriter<'a> {
    fn get_base_layer(&self, page_number: usize) -> PdfLayerReference {
        let (page_index, layers) = &self.page_layer_indices[page_number];
        let first_layer = layers[0];

        let page = self.raw_pdf_doc.get_page(*page_index);
        let layer = page.get_layer(first_layer);

        layer
    }

    fn set_base_layer_style(
        &mut self,
        page_number: usize,
        layer: &PdfLayerReference,
        span: &RichTextSpan,
    ) -> Result<Rc<IndirectFontRef>, DocumentGenerationError> {
        let font = self
            .font_collection
            .lookup_font(&span.font_family, &span.attributes)?;

        let style = &self.current_style_by_page[page_number];

        let font_ref = self.get_font(font.font_id())?;

        let mut new_style = style.clone();
        if style.font_id != Some(font.font_id()) || style.font_size != Some(span.size) {
            layer.set_font(font_ref.as_ref(), span.size.0);

            new_style.font_id = Some(font.font_id());
            new_style.font_size = Some(span.size);
        }

        if style.color.as_ref() != Some(&span.color) {
            layer.set_fill_color(span.color.clone().into());

            new_style.color = Some(span.color.clone());
        }

        self.current_style_by_page[page_number] = new_style;

        Ok(font_ref)
    }
}
