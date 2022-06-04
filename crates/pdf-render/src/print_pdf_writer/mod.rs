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
    calculate_points_for_circle, IndirectFontRef, Line, PdfDocument, PdfDocumentReference,
    PdfLayerIndex, PdfLayerReference, PdfPageIndex, Point, Rgb, TextMatrix,
};

use crate::{
    block_layout::paginated_layout::{DebugCursor, DrawableNode, PaginatedLayout, PaginatedNode},
    document_builder::UnstructuredDocumentWriter,
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontAttributes, FontCollection, FontId},
    paragraph_layout::RenderedTextBlock,
    rich_text::RichTextSpan,
    stylesheet::{BorderRadiusStyle, Style},
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

static BASE_LAYER_NAME: &str = "Layer 1";

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
    page_size: Size<Pt>,

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
            BASE_LAYER_NAME,
        );

        Self {
            raw_pdf_doc: doc,
            fonts: FontLookup::new(),
            page_layer_indices: vec![(page_index, vec![layer_index])],
            font_collection,
            page_size: dimensions.into(),
            current_style_by_page: vec![CurrentStyles::default()],
        }
    }

    pub fn draw_debug_cursors(&mut self, debug_cursors: &[DebugCursor]) {
        let font = self
            .font_collection
            .lookup_font(&"Inter", &FontAttributes::bold())
            .unwrap();

        let font = self.get_font(font.font_id()).unwrap();

        for (idx, cursor) in debug_cursors.iter().enumerate() {
            let layer = self.get_base_layer(cursor.page_index);
            layer.set_outline_color(Color::black().into());
            layer.set_fill_color(printpdf::Color::Rgb(Rgb {
                r: 0.2,
                g: 0.,
                b: 0.,
                icc_profile: None,
            }));

            let x_position = Pt((idx % 6) as f64 * 90. + 10.);

            let cursor_points = vec![
                (
                    Point::new(
                        Mm::from(x_position).into(),
                        Mm::from(self.page_size.height - cursor.position.y).into(),
                    ),
                    false,
                ),
                (
                    Point::new(
                        Mm::from(x_position + Pt(20.)).into(),
                        Mm::from(self.page_size.height - cursor.position.y).into(),
                    ),
                    false,
                ),
            ];

            let line = Line {
                points: cursor_points,
                is_closed: false,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            };

            layer.set_outline_thickness(5.);
            layer.add_shape(line);

            layer.begin_text_section();
            layer.set_text_cursor(
                Mm::from(x_position + Pt(20.)).into(),
                Mm::from(self.page_size.height - cursor.position.y - Pt(15.)).into(),
            );

            layer.set_font(&font, 12.);
            layer.write_text(&format!("{} - {}", idx, cursor.label), &font);

            layer.end_text_section();
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
    fn draw_text_block(
        &mut self,
        layout: &PaginatedLayout,
        style: &Style::Unmergeable,
        text_block: &RenderedTextBlock,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let layer = self.get_base_layer(layout.page_index);

        layer.begin_text_section();

        let x = printpdf::Pt::from(style.padding.left + layout.left());
        let y = printpdf::Pt::from(self.page_size.height - (layout.top() + style.padding.top));

        let mut current_y = y;
        for line in text_block.lines.iter() {
            layer.set_text_matrix(TextMatrix::Translate(
                x + line.line_metrics.left.into(),
                current_y - line.line_metrics.ascent.into(),
            ));

            for span in line.rich_text.0.iter() {
                let font = self.set_base_layer_style(layout.page_index, &layer, &span)?;

                layer.write_text(span.text.clone(), font.as_ref());
            }

            current_y -= line.line_metrics.height.into();
        }

        layer.end_text_section();

        Ok(self)
    }

    fn draw_node(&mut self, node: &PaginatedNode) -> Result<&mut Self, DocumentGenerationError> {
        let node_style = node.drawable_node.style();

        self.draw_container(&node.layout, &node_style)?;

        match &node.drawable_node {
            DrawableNode::Text(text_node) => {
                self.draw_text_block(&node.layout, node_style, &text_node.text_block)?;
            }
            _ => {}
        }

        Ok(self)
        // todo!()
    }
}

impl<'a> PrintPdfWriter<'a> {
    fn draw_container(
        &mut self,
        layout: &PaginatedLayout,
        container_style: &Style::Unmergeable,
    ) -> Result<&mut Self, DocumentGenerationError> {
        if container_style.debug {
            self.draw_debug_outlines(layout, container_style);
        }

        Ok(self)
    }

    fn draw_debug_outlines(&mut self, layout: &PaginatedLayout, style: &Style::Unmergeable) {
        let PaginatedLayout {
            layout,
            // TODO: Figure out how to pattern match to deference
            page_index: page_number,
        } = layout;

        let mut margin_rect = Rect {
            left: layout.left - style.margin.left,
            top: layout.top - style.margin.top,
            width: layout.width + (style.margin.right + style.margin.left),
            height: layout.height + (style.margin.top + style.margin.bottom),
        };

        let mut border_rect = Rect {
            left: layout.left,
            top: layout.top,
            width: layout.width,
            height: layout.height,
        };

        let mut content_rect = Rect {
            left: border_rect.left + style.padding.left,
            top: border_rect.top + style.padding.top,
            width: border_rect.width - (style.padding.right + style.padding.left),
            height: border_rect.height - (style.padding.top + style.padding.bottom),
        };

        margin_rect.top = Pt::from(self.page_size.height) - margin_rect.top;
        border_rect.top = Pt::from(self.page_size.height) - border_rect.top;
        content_rect.top = Pt::from(self.page_size.height) - content_rect.top;

        self.draw_rect(
            *page_number,
            margin_rect,
            Pt(1.),
            Some(Color::try_from("green").unwrap()),
            None,
            None,
        );

        self.draw_rect(
            *page_number,
            border_rect,
            Pt(1.),
            Some(Color::try_from("red").unwrap()),
            None,
            None,
        );

        self.draw_rect(
            *page_number,
            content_rect,
            Pt(1.),
            Some(Color::try_from("blue").unwrap()),
            None,
            None,
        );
    }

    fn get_base_layer(&mut self, page_number: usize) -> PdfLayerReference {
        while page_number >= self.page_layer_indices.len() {
            let (page_index, layer_index) = self.raw_pdf_doc.add_page(
                Mm::from(self.page_size.width).into(),
                Mm::from(self.page_size.height).into(),
                BASE_LAYER_NAME,
            );

            self.page_layer_indices
                .push((page_index, vec![layer_index]));

            self.current_style_by_page.push(CurrentStyles::default());
        }

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

    fn draw_rect(
        &mut self,
        page_number: usize,
        rect: Rect<Pt>,
        border_width: Pt,
        border_color: Option<Color>,
        background_color: Option<Color>,
        border_radius: Option<BorderRadiusStyle::Unmergeable>,
    ) {
        let layer = self.get_base_layer(page_number);

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
