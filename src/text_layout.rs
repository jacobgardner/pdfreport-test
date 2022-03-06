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
    fonts::FONTS,
    line_metric::{LineMetric, ParagraphMetrics},
};

#[derive(Debug)]
pub struct TextLayout {
    // TODO: Remove pub once everything is encapsulated
    pub font_collection: FontCollection,
    pub typeface: Typeface,
}

impl TextLayout {
    #[instrument(name = "Initialize text layout engine")]
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();

        let mut tfp = TypefaceFontProvider::new();

        let mut typeface: Option<Typeface> = None;

        for font in FONTS {
            // Safe because all the font data is 'static
            // They probably could have enforced this with a type to be safe...
            unsafe {
                let d = Data::new_bytes(font);
                let t = Typeface::from_data(d, None);
                typeface = Some(t.clone().unwrap());
                tfp.register_typeface(t.unwrap(), Some("Inter"));
            }
        }

        let manager = FontMgr::from(tfp);

        font_collection.set_asset_font_manager(manager);
        font_collection.disable_font_fallback();

        Self {
            font_collection,
            typeface: typeface.unwrap(),
        }
    }

    #[instrument(name = "Computing paragraph layout")]
    pub fn compute_paragraph_layout(&self, text: &str, width: Pt) -> ParagraphMetrics {
        let mut paragraph_style = ParagraphStyle::new();

        let mut ts = TextStyle::new();
        ts.set_font_style(FontStyle::new(
            Weight::NORMAL,
            Width::NORMAL,
            Slant::Upright,
        ));
        ts.set_font_size(12.);
        ts.set_font_families(&["Inter"]);

        paragraph_style.set_text_style(&ts);
        paragraph_style.set_text_align(TextAlign::Center);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

        paragraph_builder.push_style(&ts);

        paragraph_builder.add_text(text);
        // paragraph_builder.set_paragraph_style(ParagraphStyle::new().set_text_align(TextAlign::Center));

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
                    ascent: Pt(metrics.ascent),
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
