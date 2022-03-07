use std::ops::RangeBounds;

use itertools::Itertools;
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
    rich_text::RichText,
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
            // Safe because all the font dat
            // a is 'static
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
    pub fn compute_paragraph_layout(&self, rich_text: &RichText, width: Pt) -> ParagraphMetrics {
        let mut paragraph_style = ParagraphStyle::new();

        let mut default_style = TextStyle::new();

        default_style.set_font_style(FontStyle::new(
            rich_text.default_style.weight.unwrap_or_default().into(),
            Width::NORMAL,
            if rich_text.default_style.italic.unwrap_or_default() {
                Slant::Italic
            } else {
                Slant::Upright
            },
        ));
        default_style.set_font_size(rich_text.default_style.font_size.unwrap_or(Pt(12.)).0 as f32);
        // TODO: Make configurable in the future
        default_style.set_font_families(&["Inter"]);

        paragraph_style.set_text_style(&default_style);
        paragraph_style.set_text_align(TextAlign::Center);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

        let style_stack = vec![default_style];

        let mut current_position = 0;

        let range_stack = vec![0..rich_text.paragraph.len()];

        let next_range = 0;
        

        while !range_stack.is_empty() {
            let current_range = range_stack.last().unwrap();

            if let Some((a, b)) = rich_text.style_ranges.get(next_range) {

            }


            // next_range += 1;


            // if let Some((a, b)) = range_iter.next() {

            // } else {

            // }



        }

        // for (range, style) in rich_text.style_ranges.iter() {

        //     let current_range = range_stack.last().unwrap();

        //     if range.start <= current_range.end {
        //         // Nested

        //         // current_range.start -> range.start
        //         // push?
        //     } else {
        //         // current_range.end -> range.start
        //         // pop?

        //     }

        // }





        // for (first, second) in rich_text.style_ranges.iter().tuple_windows() {

        //     // if current_position < first.0.start {

        //     // }
        // }

        // paragraph_builder.push_style(&ts);

        paragraph_builder.add_text(rich_text.paragraph);
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

use crate::rich_text::FontWeight;

impl From<FontWeight> for Weight {
    fn from(weight: FontWeight) -> Self {
        // TODO: FIXME
        // There's an easier way to do this, but I didn't feel like looking it
        // up right now.
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
