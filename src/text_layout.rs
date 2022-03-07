use std::ops::RangeBounds;

use itertools::Itertools;
use printpdf::{Pt, Rgb};
use skia_safe::{
    font_style::{Slant, Weight, Width},
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle,
        TypefaceFontProvider,
    },
    Color, Data, FontMgr, FontStyle, Typeface, RGB,
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
        // paragraph_style.set_text_align(TextAlign::Center);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

        let mut style_stack = vec![default_style];

        let mut range_stack = vec![0..rich_text.paragraph.len()];

        let mut next_range_index = 0;
        let mut current_position = 0;

        // let mut next_position = rich_text.paragraph.len();

        while !range_stack.is_empty() {
            let current_range = range_stack.last().unwrap().clone();
            let current_style = style_stack.last().unwrap().clone();

            let end_position =
                if let Some((range, _)) = rich_text.style_ranges.get(next_range_index) {
                    if range.start < current_range.end {
                        range.start
                    } else {
                        style_stack.pop();
                        range_stack.pop();
                        // Finish up the current range
                        current_range.end
                    }
                } else {
                    style_stack.pop();
                    range_stack.pop();
                    current_range.end
                };

            if current_position < end_position {
                // Do the stuff

                // let mut style = ParagraphStyle::new();
                // paragraph_style.set_text_style(&current_style);
                // style.set_text_style(&current_style);
                paragraph_builder.push_style(&current_style);

                let current_span = &rich_text.paragraph[current_position..end_position];
                paragraph_builder.add_text(current_span);
                println!("{}..{}", current_position, end_position);
                println!("{}", current_span);
            }

            current_position = end_position;

            if let Some((range, style)) = rich_text.style_ranges.get(next_range_index) {
                range_stack.push(range.clone());

                let mut next_style = current_style.clone();
                if let Some(Pt(font_size)) = style.font_size {
                    next_style.set_font_size(font_size as f32);
                }

                let prev_font_style = next_style.font_style();

                let next_weight = if let Some(weight) = style.weight {
                    weight.into()
                } else {
                    prev_font_style.weight()
                };

                let next_slant = if let Some(italic) = style.italic {
                    if italic {
                        Slant::Italic
                    } else {
                        Slant::Upright
                    }
                } else {
                    prev_font_style.slant()
                };

                let next_font_style =
                    FontStyle::new(next_weight, prev_font_style.width(), next_slant);

                if next_font_style != prev_font_style {
                    next_style.set_font_style(next_font_style);
                }

                // Don't need Color doesn't matter for layout...
                // if let Some(color) = style.color {
                //     next_style.set_color(
                //         Color::from_rgb(color.0, color.1 as f64, color.2 as f64).into(),
                //     );
                // }

                // Create new style based on prev_style
                style_stack.push(next_style);
                next_range_index += 1;
            };
        }

        // paragraph_builder.add_text(rich_text.paragraph);
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

        println!("{:?}", metrics);

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
