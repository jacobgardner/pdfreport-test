use crate::{error::DocumentGenerationError, rich_text::RichTextSpan};

use skia_safe::{textlayout as skia_layout, FontStyle};

use super::ParagraphLayout;

impl ParagraphLayout {
    fn layout_style_from_span(
        &self,
        span: &RichTextSpan,
    ) -> Result<skia_layout::TextStyle, DocumentGenerationError> {
        let mut span_style = skia_layout::TextStyle::new();
        
        // let skia_font_style = FontStyle::new(
        //     rich_text.default_style.weight.into(),
        //     Width::NORMAL,
        //     match rich_text.default_style.style {
        //         crate::rich_text::FontStyle::Normal => Slant::Upright,
        //         crate::rich_text::FontStyle::Italic => Slant::Italic,
        //     },
        // );

        span_style.set_font_size(span.size.0 as f32);
        span_style.set_font_families(&[self.get_font_family(span.font_id)?]);
        // span_style.set_font_style(skia_font_style);

        Ok(span_style)
    }
}
