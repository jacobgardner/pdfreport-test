//! Handles the text layout within a paragraph. Determines when
//!  to wrap text and how lines should be positioned relative to each other. This
//!  is utilized by the block_layout engine because the block height is determined
//!  by the paragraph height constrained by the block width.
use std::collections::HashSet;

mod layout_style;
mod line_metrics;
mod styles;
mod text_block;

pub use layout_style::{ParagraphStyle, TextAlign};
pub use line_metrics::LineMetrics;
pub use text_block::{RenderedTextBlock, RenderedTextLine};

use skia_layout::{ParagraphBuilder, TypefaceFontProvider};
use skia_safe::textlayout::{self as skia_layout};
use skia_safe::{Data, FontMgr, Typeface};

use crate::{
    error::{DocumentGenerationError, InternalServerError},
    fonts::FontCollection,
    rich_text::RichText,
    values::Pt,
};

pub struct ParagraphLayout {
    skia_font_collection: skia_layout::FontCollection,
    font_families: HashSet<String>,
}

impl Default for ParagraphLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl ParagraphLayout {
    pub fn new() -> Self {
        Self {
            skia_font_collection: skia_layout::FontCollection::new(),
            font_families: HashSet::new(),
        }
    }

    pub fn load_fonts(
        &mut self,
        font_collection: &FontCollection,
    ) -> Result<&mut Self, DocumentGenerationError> {
        let mut typeface_font_provider = TypefaceFontProvider::new();

        for (family_name, font_family) in font_collection.as_ref().iter() {
            for (attributes, data) in font_family.as_ref().iter() {
                self.font_families.insert(family_name.clone());

                let data = Data::new_copy(data.as_bytes());
                let typeface = Typeface::from_data(data, None).ok_or_else(|| {
                    InternalServerError::SkiaTypefaceFailure {
                        family_name: family_name.clone(),
                        attributes: *attributes,
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
        layout_style: ParagraphStyle,
        rich_text: &RichText,
        width: Pt,
    ) -> Result<RenderedTextBlock, DocumentGenerationError> {
        let mut paragraph_style = skia_layout::ParagraphStyle::new();

        paragraph_style.set_text_align(layout_style.align.into());
        // paragraph_style.set_text_height_behavior(skia_layout::TextHeightBehavior::DisableAll);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.skia_font_collection.clone());

        for span in rich_text.0.iter() {
            let span_style = self.layout_style_from_span(span)?;

            paragraph_builder.push_style(&span_style);
            paragraph_builder.add_text(&span.text);
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(width.0 as f32);

        let mut rendered_text_block = RenderedTextBlock { lines: vec![] };

        let mut height = Pt(0.);
        for line_metrics in paragraph.get_line_metrics().iter() {
            height += Pt(line_metrics.height as f64);
            rendered_text_block.lines.push(RenderedTextLine {
                rich_text: rich_text.substr(line_metrics.start_index, line_metrics.end_index)?,
                line_metrics: line_metrics.into(),
            });
        }

        Ok(rendered_text_block)
    }
}
