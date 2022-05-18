use std::{collections::HashMap, hash::Hash};

use skia_safe::{
    textlayout::{ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle, TypefaceFontProvider},
    typeface, Data, FontMgr, Typeface,
};

use crate::{
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontCollection, FontId},
    geometry::Pt,
    rich_text::RichText,
};

#[derive(Debug)]
pub struct LineMetrics {
    pub ascent: Pt,
    pub descent: Pt,
    pub baseline: Pt,
    pub height: Pt,
    pub width: Pt,
    pub left: Pt,
}

struct RenderedTextLine {
    rich_text: RichText,
    line_metrics: LineMetrics,
}

struct RenderedTextBlock {
    // block_metrics: BlockMetrics
    lines: Vec<RenderedTextLine>,
}

struct ParagraphLayout {
    skia_font_collection: skia_safe::textlayout::FontCollection,
    fonts: HashMap<FontId, String>,
}

// TODO: rename
struct LayoutStyle {
    // align:
}

impl ParagraphLayout {
    pub fn new() -> Self {
        Self {
            skia_font_collection: skia_safe::textlayout::FontCollection::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn load_fonts(
        &mut self,
        font_collection: &FontCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let mut typeface_font_provider = TypefaceFontProvider::new();

        for (family_name, font_family) in font_collection.as_ref().iter() {
            for (attributes, data) in font_family.as_ref().iter() {
                self.fonts.insert(data.font_id(), family_name.clone());

                let data = Data::new_copy(data.as_bytes());
                let typeface = Typeface::from_data(data, None).ok_or_else(|| {
                    InternalServerError::SkiaTypefaceFailure {
                        family_name: family_name.clone(),
                        attributes: attributes.clone(),
                    }
                })?;

                typeface_font_provider.register_typeface(typeface, Some(&family_name));
            }
        }

        let font_manager = FontMgr::from(typeface_font_provider);
        self.skia_font_collection
            .set_asset_font_manager(font_manager);
        self.skia_font_collection.disable_font_fallback();

        Ok(self)
    }

    pub fn calculate_layout(
        &self,
        layout_style: LayoutStyle,
        rich_text: &RichText,
        width: Pt,
    ) -> Result<RenderedTextBlock, DocumentGenerationError> {
        let mut paragraph_style = ParagraphStyle::new();
        let mut default_style = TextStyle::new();

        paragraph_style.set_text_align(TextAlign::Left);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.skia_font_collection.clone());

        for span in rich_text.0.iter() {
            let mut span_style = default_style.clone();

            span_style.set_font_size(span.size.0 as f32);

            paragraph_builder.push_style(&span_style);
            paragraph_builder.add_text(&span.text);
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(width.0 as f32);

        let mut rendered_text_block = RenderedTextBlock { lines: vec![] };

        for line_metrics in paragraph.get_line_metrics().iter() {
            rendered_text_block.lines.push(RenderedTextLine {
                rich_text: rich_text.substr(line_metrics.start_index, line_metrics.end_index)?,
                line_metrics: line_metrics.into(),
            });
        }

        Ok(rendered_text_block)
    }
}

impl<'a> From<&skia_safe::textlayout::LineMetrics<'a>> for LineMetrics {
    fn from(metrics: &skia_safe::textlayout::LineMetrics) -> Self {
        Self {
            ascent: metrics.ascent.into(),
            descent: metrics.descent.into(),
            baseline: metrics.baseline.into(),
            height: metrics.height.into(),
            width: metrics.width.into(),
            left: metrics.left.into(),
        }
    }
}
