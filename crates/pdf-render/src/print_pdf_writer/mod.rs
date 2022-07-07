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
    fonts::{FontCollection, FontId},
    paragraph_layout::RenderedTextBlock,
    rich_text::RichTextSpan,
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

impl<'a> PrintPdfWriter<'a> {
    fn get_placement_coords(&self, layout: &NodeLayout) -> (Pt, Pt) {
        let x_position = layout.left + self.page_margins.left;
        let y_position = self.page_size.height
            - (layout.top + self.page_margins.top)
            - layout.height;

        (x_position, y_position)
    }

    fn draw_image(
        &mut self,
        node: &PaginatedNode,
        image_node: &DrawableImageNode,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let layer = self.get_base_layer(node.page_index);

        if let Image::SVG(ref svg_content) = image_node.image {
            let svg = Svg::parse(&svg_content).unwrap();

            let svg_xobject = svg.into_xobject(&layer);

            let (x_position, y_position) = self.get_placement_coords(&node.page_layout);

            let doc = roxmltree::Document::parse(&svg_content).unwrap();

            let svg_node = doc
                .descendants()
                .find(|node| node.has_tag_name("svg"))
                .unwrap();

            let svg_width = Pt::from_px(svg_node.attribute("width").unwrap().parse().unwrap());
            let svg_height = Pt::from_px(svg_node.attribute("height").unwrap().parse().unwrap());

            svg_xobject.add_to_layer(
                &layer,
                SvgTransform {
                    translate_x: Some(x_position.into()),
                    translate_y: Some(y_position.into()),
                    scale_x: Some((node.page_layout.width / svg_width).0),
                    scale_y: Some((node.page_layout.height / svg_height).0),
                    ..Default::default()
                },
            );
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
