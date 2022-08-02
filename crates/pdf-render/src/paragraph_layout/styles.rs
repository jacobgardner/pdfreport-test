use crate::{
    error::{DocumentGenerationError, InternalServerError},
    fonts::{FontSlant, FontWeight},
    rich_text::RichTextSpan,
};

use skia_safe::{
    font_style::{Slant, Weight, Width},
    textlayout as skia_layout, FontStyle,
};

use super::ParagraphLayout;

impl ParagraphLayout {
    pub(super) fn layout_style_from_span(
        &self,
        span: &RichTextSpan,
    ) -> Result<skia_layout::TextStyle, DocumentGenerationError> {
        let mut span_style = skia_layout::TextStyle::new();

        let skia_font_style = FontStyle::new(
            span.attributes.weight.into(),
            Width::NORMAL,
            span.attributes.style.into(),
        );

        span_style.set_height_override(true);
        span_style.set_height(span.line_height as f32);
        span_style.set_font_size(span.size.0 as f32);
        // let ratio = span.letter_spacing / span.size;
        // span_style.set_letter_spacing(1.50);
        span_style.set_letter_spacing(span.letter_spacing.0 as f32);

        if !self.font_families.contains(&span.font_family) {
            return Err(
                InternalServerError::FontFamilyNotRegisteredForLayoutEngine {
                    family_name: span.font_family.clone(),
                }
                .into(),
            );
        }

        span_style.set_font_families(&[&span.font_family]);
        span_style.set_font_style(skia_font_style);

        Ok(span_style)
    }
}

impl From<FontWeight> for Weight {
    fn from(weight: FontWeight) -> Self {
        match weight {
            FontWeight::Thin => Weight::THIN,
            FontWeight::ExtraLight => Weight::EXTRA_LIGHT,
            FontWeight::Light => Weight::LIGHT,
            FontWeight::Regular => Weight::NORMAL,
            FontWeight::Medium => Weight::MEDIUM,
            FontWeight::SemiBold => Weight::SEMI_BOLD,
            FontWeight::Bold => Weight::BOLD,
            FontWeight::ExtraBold => Weight::EXTRA_BOLD,
            FontWeight::Black => Weight::BLACK,
        }
    }
}

impl From<FontSlant> for Slant {
    fn from(style: FontSlant) -> Self {
        match style {
            FontSlant::Italic => Slant::Italic,
            FontSlant::Normal => Slant::Upright,
        }
    }
}
