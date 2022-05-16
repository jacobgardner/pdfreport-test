use std::{collections::HashMap, rc::Rc};

use printpdf::Pt;
use skia_safe::{
    font_style::{Slant, Weight, Width},
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle,
        TypefaceFontProvider,
    },
    Data, FontMgr, FontStyle, Typeface,
};
use tracing::instrument;

use crate::{
    error::BadPdfLayout,
    fonts::{FontLookup, FontManager},
    line_metric::{LineMetric, ParagraphMetrics},
    pdf_writer::{FontKey, GlyphLookup},
    rich_text::RichText,
};

#[derive(Debug)]
pub struct LayoutFonts {
    font_collection: FontCollection,
    font_manager: FontMgr,
    typeface_lookup: HashMap<String, HashMap<FontKey, Typeface>>,
}

impl HasFontCollection for Rc<LayoutFonts> {
    fn font_collection(&self) -> &skia_safe::textlayout::FontCollection {
        &self.font_collection
    }
}

impl LayoutFonts {
    pub fn with_font_manager(font_manager: &FontManager) -> Self {
        let mut font_collection = FontCollection::new();

        let mut tfp = TypefaceFontProvider::new();
        let mut typeface_lookup = HashMap::new();

        for (family_name, family) in font_manager.families.iter() {
            let mut family_lookup = HashMap::new();
            for font in family.fonts.iter() {
                unsafe {
                    let d = Data::new_bytes(font.bytes.as_ref());
                    let t = Typeface::from_data(d, None).unwrap();

                    family_lookup.insert(
                        FontKey {
                            weight: font.weight,
                            style: font.style,
                        },
                        t.clone(),
                    );

                    tfp.register_typeface(t, Some(&family_name));
                }
            }

            typeface_lookup.insert(family_name.clone(), family_lookup);
        }

        let font_manager = FontMgr::from(tfp);

        font_collection.set_asset_font_manager(font_manager.clone());
        font_collection.disable_font_fallback();

        Self {
            font_collection,
            font_manager,
            typeface_lookup,
        }
    }

    fn typeface_by_font_style(&self, lookup: &FontLookup) -> Result<Typeface, BadPdfLayout> {
        self.typeface_lookup
            .get(lookup.family_name)
            .ok_or_else(|| BadPdfLayout::FontFamilyNotFound {
                font_family: lookup.family_name.to_owned(),
            })?
            .get(&FontKey {
                weight: lookup.weight,
                style: lookup.style,
            })
            .cloned()
            .ok_or_else(|| BadPdfLayout::FontStyleNotFoundForFamily {
                font_family: lookup.family_name.to_owned(),
                font_weight: lookup.weight,
                font_style: lookup.style,
            })
    }
}

impl GlyphLookup for LayoutFonts {
    fn get_glyph_ids(
        &self,
        line: &str,
        font_lookup: &FontLookup,
    ) -> Result<Vec<u16>, BadPdfLayout> {
        let typeface = self.typeface_by_font_style(font_lookup)?;

        let mut glyph_ids = vec![0; line.len()];
        typeface.str_to_glyphs(line, &mut glyph_ids);

        Ok(glyph_ids)
    }
}

#[derive(Debug)]
pub struct TextLayout<T: HasFontCollection> {
    layout_fonts: T,
}

pub trait HasFontCollection: std::fmt::Debug {
    fn font_collection(&self) -> &FontCollection;
}

impl<T: HasFontCollection> TextLayout<T> {
    #[instrument(name = "Initialize text layout engine")]
    pub fn new(layout_fonts: T) -> Self {
        Self { layout_fonts }
    }

    #[instrument(name = "Computing paragraph layout")]
    pub fn compute_paragraph_layout(&self, rich_text: &RichText, width: Pt) -> ParagraphMetrics {
        let mut paragraph_style = ParagraphStyle::new();

        let mut default_style = TextStyle::new();

        default_style.set_font_style(FontStyle::new(
            rich_text.default_style.weight.into(),
            Width::NORMAL,
            match rich_text.default_style.style {
                crate::rich_text::FontStyle::Normal => Slant::Upright,
                crate::rich_text::FontStyle::Italic => Slant::Italic,
            },
        ));
        default_style.set_font_size(rich_text.default_style.font_size.0 as f32);
        // TODO: Throw an error if font does not exist
        default_style.set_font_families(&[&rich_text.default_style.font_family]);

        paragraph_style.set_text_style(&default_style);
        paragraph_style.set_text_align(TextAlign::Left);

        let mut paragraph_builder = ParagraphBuilder::new(
            &paragraph_style,
            self.layout_fonts.font_collection().clone(),
        );

        for (range, style) in rich_text.style_range_iter() {
            let mut ts = default_style.clone();
            ts.set_font_style(FontStyle::new(
                style.weight.into(),
                Width::NORMAL,
                match style.style {
                    crate::rich_text::FontStyle::Italic => Slant::Italic,
                    crate::rich_text::FontStyle::Normal => Slant::Upright,
                },
            ));
            ts.set_font_size(style.font_size.0 as f32);

            paragraph_builder.push_style(&ts);

            let current_span = &rich_text.paragraph[range.start..range.end];
            paragraph_builder.add_text(current_span);
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(width.0 as f32);

        let mut height = 0.;

        let metrics = paragraph
            .get_line_metrics()
            .iter()
            .map(|metrics| {
                let metric = LineMetric {
                    start_index: metrics.start_index,
                    end_index: metrics.end_index,
                    height: Pt(metrics.height),
                    left: Pt(metrics.left),
                    width: Pt(metrics.width),
                    ascent: Pt(metrics.ascent),
                    descent: Pt(metrics.descent),
                    baseline: Pt(metrics.baseline),
                };

                height += metrics.height;

                metric
            })
            .collect();

        ParagraphMetrics {
            line_metrics: metrics,
            height: Pt(height),
        }
    }
}

use crate::rich_text::FontWeight;

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

impl From<crate::rich_text::FontStyle> for Slant {
    fn from(style: crate::rich_text::FontStyle) -> Self {
        match style {
            crate::rich_text::FontStyle::Italic => Slant::Italic,
            crate::rich_text::FontStyle::Normal => Slant::Upright,
        }
    }
}
