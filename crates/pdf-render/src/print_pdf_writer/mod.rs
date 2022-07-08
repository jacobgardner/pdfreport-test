//! This is ultimately what takes nodes that have been styled
//!  and laid out and writes them to a PDF.
use std::{
    io::{BufWriter, Write},
    rc::Rc,
};

use printpdf::{
    IndirectFontRef, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfLayerReference,
    PdfPageIndex, Svg, SvgTransform, TextMatrix,
};

mod corners;
mod debug;
mod font_lookup;
mod rect;

use crate::{
    block_layout::{
        layout_engine::NodeLayout,
        paginated_layout::{DrawableImageNode, DrawableNode, Image, PaginatedNode},
    },
    document_builder::UnstructuredDocumentWriter,
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontAttributes, FontCollection, FontId, FontSlant, FontWeight},
    paragraph_layout::{LineMetrics, RenderedTextBlock, RenderedTextLine},
    rich_text::{RichText, RichTextSpan},
    stylesheet::{EdgeStyle, Style},
    values::{Color, Mm, Pt, Size},
};

use self::{corners::Circles, font_lookup::FontLookup};

#[derive(Clone, Default)]
struct CurrentStyles {
    font_id: Option<FontId>,
    font_size: Option<Pt>,
    letter_spacing: Option<Pt>,

    color: Option<Color>,
}

static BASE_LAYER_NAME: &str = "Layer 1";

pub struct PrintPdfWriter<'a> {
    raw_pdf_doc: PdfDocumentReference,
    fonts: FontLookup,
    page_layer_indices: Vec<(PdfPageIndex, Vec<PdfLayerIndex>)>,
    font_collection: &'a FontCollection,
    page_size: Size<Pt>,
    page_margins: EdgeStyle::Unmergeable,
    current_style_by_page: Vec<CurrentStyles>,
    circle_cache: Circles,
}

impl<'a> PrintPdfWriter<'a> {
    pub fn new(
        doc_title: &str,
        page_size: impl Into<Size<Mm>>,
        page_margins: impl Into<EdgeStyle::Unmergeable>,
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
            page_margins: page_margins.into(),
            page_size: dimensions.into(),
            current_style_by_page: vec![CurrentStyles::default()],
            circle_cache: Default::default(),
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
        node: &PaginatedNode,
        style: &Style::Unmergeable,
        text_block: &RenderedTextBlock,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let layer = self.get_base_layer(node.page_index);

        layer.begin_text_section();

        let x =
            printpdf::Pt::from(style.padding.left + node.page_layout.left + self.page_margins.left);
        let y = printpdf::Pt::from(
            self.page_size.height
                - (node.page_layout.top + style.padding.top + self.page_margins.top),
        );

        let mut current_y = y;
        for line in text_block.lines.iter() {
            layer.set_text_matrix(TextMatrix::Translate(
                x + line.line_metrics.left.into(),
                current_y - line.line_metrics.ascent.into(),
            ));

            for span in line.rich_text.0.iter() {
                let font = self.set_base_layer_style(node.page_index, &layer, span)?;

                layer.write_text(span.text.clone(), font.as_ref());
            }

            current_y -= line.line_metrics.height.into();
        }

        layer.end_text_section();

        Ok(self)
    }

    fn draw_node(&mut self, node: &PaginatedNode) -> Result<&mut Self, DocumentGenerationError> {
        let node_style = node.drawable_node.style();

        self.draw_container(node, node_style)?;

        match &node.drawable_node {
            DrawableNode::Text(text_node) => {
                self.draw_text_block(node, node_style, &text_node.text_block)?;
            }
            DrawableNode::Image(image_node) => {
                self.draw_image(node, image_node)?;
            }
            _ => {}
        }

        Ok(self)
    }
}

const SUPPORTED_SVG_TEXT_ATTRIBUTES: [&str; 10] = [
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

impl<'a> PrintPdfWriter<'a> {
    fn get_placement_coords(&self, layout: &NodeLayout) -> (Pt, Pt) {
        let x_position = layout.left + self.page_margins.left;
        let y_position =
            self.page_size.height - (layout.top + self.page_margins.top) - layout.height;

        (x_position, y_position)
    }

    // TODO: Remove this if we ever add another variant to the Image enum
    #[allow(irrefutable_let_patterns)]
    fn draw_image(
        &mut self,
        svg: &crate::image::Svg,
        paginated_node: &PaginatedNode,
        // image_node: &DrawableImageNode,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let layer = self.get_base_layer(paginated_node.page_index);

        if let Image::SVG(ref svg_content) = image_node.image {
            let svg = Svg::parse(&svg_content).unwrap();

            let svg_xobject = svg.into_xobject(&layer);

            let (x_position, y_position) = self.get_placement_coords(&paginated_node.page_layout);

            let doc = roxmltree::Document::parse(&svg_content).unwrap();

            let svg_node = doc
                .descendants()
                .find(|node| node.has_tag_name("svg"))
                .unwrap();

            let svg_width = Pt::from_px(svg_node.attribute("width").unwrap().parse().unwrap());
            let svg_height = Pt::from_px(svg_node.attribute("height").unwrap().parse().unwrap());

            let x_scale = paginated_node.page_layout.width / svg_width;
            let y_scale = paginated_node.page_layout.height / svg_height;

            svg_xobject.add_to_layer(
                &layer,
                SvgTransform {
                    translate_x: Some(x_position.into()),
                    translate_y: Some(y_position.into()),
                    scale_x: Some(x_scale),
                    scale_y: Some(y_scale),
                    ..Default::default()
                },
            );

            for node in doc.descendants().filter(|n| n.has_tag_name("text")) {
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

                let unsupported_attribute = node.attributes().iter().find(|a| {
                    !SUPPORTED_SVG_TEXT_ATTRIBUTES.contains(&a.name().to_lowercase().as_str())
                });

                if let Some(unsupported_attribute) = unsupported_attribute {
                    panic!(
                        "<text .../> attribute, {}, is not yet supported",
                        unsupported_attribute.name()
                    );
                }

                let x = Pt::try_from(node.attribute("x").unwrap_or("0"))? * x_scale;
                let y = Pt::try_from(node.attribute("y").unwrap_or("0"))? * y_scale;
                let weight = FontWeight::from(node.attribute("font-weight").unwrap_or("regular"));
                let font_style = FontSlant::from(node.attribute("font-style").unwrap_or("normal"));
                let font_size = Pt::try_from(node.attribute("font-size").unwrap_or("12"))? * y_scale;
                // TODO: Don't unwrap
                let fill = Color::try_from(node.attribute("fill").unwrap_or("#000000")).unwrap();
                let anchor = node.attribute("text-anchor").unwrap_or("start");
                let font_stack = node.attribute("font-family").unwrap_or("sans-serif");
                let dominant_baseline = node.attribute("dominant-baseline").unwrap_or("auto");

                // TODO: Don't hardcode
                let found_font = "Inter"; // self.find_best_font_from_stack(font_stack)?;

                if !node.children().all(|n| n.is_text()) {
                    panic!("For <text>, we only support all text child nodes for now");
                }

                let node_text = node.text().unwrap().trim();

                let text_block = RenderedTextBlock {
                    lines: vec![RenderedTextLine {
                        rich_text: RichText(vec![RichTextSpan {
                            text: node_text.to_string(),
                            attributes: FontAttributes {
                                weight,
                                style: font_style,
                            },
                            font_family: found_font.to_string(),
                            size: font_size,
                            color: fill,
                            letter_spacing: Pt(0.),
                            line_height: 1.,
                        }]),
                        line_metrics: LineMetrics { ascent: Pt(5.), descent: Pt(5.), baseline: Pt(5.), height: Pt(5.), width: Pt(5.), left: Pt(5.) },
                    }],
                };

                self.draw_text_block(
                    &PaginatedNode {
                        page_layout: NodeLayout {
                            left: paginated_node.page_layout.left + x,
                            right: paginated_node.page_layout.right + x,
                            top: paginated_node.page_layout.top + y,
                            ..paginated_node.page_layout.clone()
                        },
                        ..paginated_node.clone()
                        // page_layout: (),
                        // page_index: (),
                        // drawable_node: (),
                    },
                    &Style::Unmergeable::default(),
                    &text_block,
                ).unwrap();

                // let rich = RichText::new(
                //     node_text,
                //     RichTextStyle {
                //         font_family: String::from(found_font),
                //         font_size,
                //         weight,
                //         style: font_style,
                //         color: (fill.0 as f32, fill.1 as f32, fill.2 as f32),
                //     },
                // );

                // let paragraph = layout.compute_paragraph_layout(&rich, Pt(1000.0));
                // assert_eq!(paragraph.line_metrics.len(), 1);

                // let line_metric = paragraph.line_metrics.first().unwrap();

                // let x_offset = match anchor.to_lowercase().as_str() {
                //     "start" => Pt(0.0),
                //     "middle" => line_metric.width / 2.,
                //     "end" => line_metric.width,
                //     _ => panic!(""),
                // };

                // let y_offset = match dominant_baseline.to_lowercase().as_str() {
                //     "auto" => Pt(0.),
                //     // TODO: This is wrong, but good enough for initial testing...
                //     "middle" | "central" => (line_metric.ascent - line_metric.descent) / 2.,
                //     baseline => panic!("{} as dominant-baseline is not yet supported", baseline),
                // };

                // layer.set_text_matrix(TextMatrix::Translate(
                //     start.x + x - x_offset,
                //     start.y + y - y_offset,
                // ));

                // let font_lookup = FontLookup {
                //     family_name: found_font,
                //     weight,
                //     style: font_style,
                // };

                // let current_font = self.writer.lookup_font(&font_lookup)?;

                // layer.set_font(current_font, font_size.0);
                // layer.set_fill_color(printpdf::Color::Rgb(Rgb::new(fill.0, fill.1, fill.2, None)));

                // self.write_text(&layer, node_text, &font_lookup)?;
            }
        } else {
            unimplemented!("Only SVGs are currently supported");
        }

        Ok(self)
    }

    fn draw_container(
        &mut self,
        node: &PaginatedNode,
        container_style: &Style::Unmergeable,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let coords = self.get_placement_coords(&node.page_layout);

        let rect = crate::values::Rect {
            left: coords.0,
            top: coords.1,
            width: node.page_layout.width,
            height: node.page_layout.height,
        };

        self.draw_rect(
            node.page_index,
            rect,
            container_style.border.width.clone(),
            Some(container_style.border.color.clone()),
            container_style.background_color.clone(),
            Some(container_style.border.radius.clone()),
        );

        if container_style.debug {
            self.draw_debug_outlines(node, container_style);
        }

        Ok(self)
    }

    fn get_base_layer(&mut self, page_index: usize) -> PdfLayerReference {
        while page_index >= self.page_layer_indices.len() {
            let (page_index, layer_index) = self.raw_pdf_doc.add_page(
                Mm::from(self.page_size.width).into(),
                Mm::from(self.page_size.height).into(),
                BASE_LAYER_NAME,
            );

            self.page_layer_indices
                .push((page_index, vec![layer_index]));

            self.current_style_by_page.push(CurrentStyles::default());
        }

        let (page_index, layers) = &self.page_layer_indices[page_index];
        let first_layer = layers[0];

        let page = self.raw_pdf_doc.get_page(*page_index);

        page.get_layer(first_layer)
    }

    fn set_base_layer_style(
        &mut self,
        page_index: usize,
        layer: &PdfLayerReference,
        span: &RichTextSpan,
    ) -> Result<Rc<IndirectFontRef>, DocumentGenerationError> {
        let font = self
            .font_collection
            .lookup_font(&span.font_family, &span.attributes)?;

        let current_style = &self.current_style_by_page[page_index];

        let font_ref = self.get_font(font.font_id())?;

        let mut new_style = current_style.clone();
        if current_style.font_id != Some(font.font_id())
            || current_style.font_size != Some(span.size)
        {
            layer.set_font(font_ref.as_ref(), span.size.0);
            layer.set_line_height(span.size.0);

            new_style.font_id = Some(font.font_id());
            new_style.font_size = Some(span.size);
        }

        if current_style.letter_spacing != Some(span.letter_spacing) {
            layer.set_character_spacing(span.letter_spacing.0);

            new_style.letter_spacing = Some(span.letter_spacing);
        }

        if current_style.color.as_ref() != Some(&span.color) {
            layer.set_fill_color(span.color.clone().into());

            new_style.color = Some(span.color.clone());
        }

        self.current_style_by_page[page_index] = new_style;

        Ok(font_ref)
    }
}
