// Compute layout -> Draw Shit -> Draw Text -> Profit

use std::{fs::File, io::BufWriter};

use printpdf::{Color, Line, Mm, PdfDocument, Point, Pt, Rgb, TextMatrix};
use skia_safe::{
    font_style::{Slant, Weight, Width},
    textlayout::{ParagraphBuilder, ParagraphStyle, TextStyle},
    FontStyle,
};
use text_layout::TextLayout;
use tracing::{span, Level};
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

mod fonts;
mod text_layout;

fn build_text() -> String {
    r#"
Chapter 1: Your approach to Work

Introduction

Why this matters

Your results indicate that you are moderate in Decision-Making. This indicates that you are likely to be generally effective
when it comes to making decisions. You are likely to spend
an adequate amount of time gathering and analyzing available
information, weighing risks, and plotting a course to follow.
In addition to this, you might approach many decisions with a
somewhat open mind to new directions. Finally, you might be
generally effective at considering potential outcomes of your
decisions prior to making them.
While you are likely to be effective in most day-to-day decision-making situations, you might have difficulty when presented with especially complex decisions. These difficulties
might take the form of being overwhelmed by options, spending too much time analyzing information, or making decisions
based on incomplete information.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
EXPLAIN THE GROUPINGS
Here's a quick snapshot of what these three groupings mean.
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc
viverra ligula nec posuere dictum. Curabitur varius erat eget
nisi sodales, nec auctor leo placerat. Vivamus sit amet porttitor urna, sed rutrum augue. Donec bibendum lacus vel felis
viverra, a auctor orci vestibulum. Nulla et erat ac sem gravida
lacinia. Maecenas molestie orci et augue convallis, ut vehicula ante fermentum. Curabitur nec blandit arcu. Suspendisse
potenti. Suspendisse leo arcu, aliquam in porttitor scelerisque,
condimentum id quam. Nulla facilisi. Vivamus id urna ipsum.
Nulla non sapien leo. Vestibulum non ligula sapien. Nullam
finibus nibh a massa auctor sodales. Nulla mauris neque,
bibendum vel velit id, aliquam pharetra neque. Nam ultricies
posuere dolor eget congue.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
HOW TO USE IT
Explaining how every measure should be completed, but allows for the user to browse at their own pace and leisure.
They're welcome to scroll through, but a deep dive is necessary. At the end of each measure the users will have two
actions to take: adding items to their action planner or marking
the measure as complete.
CALLOUT
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
Decision Making
WHY THIS MATTERS
    "#
    .replace("\n", " ")
    .replace("  ", " ")
}

fn main() {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    let output_string = build_text();

    let text_layout = TextLayout::new();

    let span = span!(Level::DEBUG, "Full Time");
    let _guard = span.enter();

    let span = span!(Level::TRACE, "Build context").entered();

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

    span.exit();

    let (doc, page1, layer1) = PdfDocument::new("DVP Report", Mm(210.0), Mm(297.0), "Layer 1");

    let font = doc
        .add_external_font(File::open("assets/fonts/inter/Inter-Regular.ttf").unwrap())
        .unwrap();

    let mut current_page_index = page1;
    let mut current_layer_index = layer1;

    let layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();

    for _ in 1..1000 {
        let span = span!(Level::TRACE, "Computing layout").entered();

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, text_layout.font_collection.clone());
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(output_string.as_str());

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(Mm(210. - 40.).into_pt().0 as f32);

        let metrics = paragraph.get_line_metrics();

        span.exit();

        let layout_y_start = Pt::from(Mm(280.0));
        let mut current_y = layout_y_start;

        let current_page = doc.get_page(current_page_index);
        let current_layer = current_page.get_layer(current_layer_index);

        let points = vec![
            (Point::new(Mm(20.), Mm(20.)), false),
            (Point::new(Mm(20. + 210. - 40.), Mm(20.)), false),
            (Point::new(Mm(20. + 210. - 40.), Mm(280.)), false),
            (Point::new(Mm(20.), Mm(280.)), false),
        ];
        let line = Line {
            points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        current_layer.add_shape(line);

        let span = span!(Level::TRACE, "Write Lines").entered();
        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.267, 0.29, 0.353, None)));

        for line_metric in metrics {
            current_layer.set_text_matrix(TextMatrix::Translate(
                Mm(20.0).into_pt(),
                current_y - Pt(line_metric.height),
            ));
            current_layer.write_text(
                &output_string[line_metric.start_index..line_metric.end_index],
                &font,
            );

            current_y -= Pt(line_metric.height);
        }
        current_layer.end_text_section();
        span.exit();

        let (cpi, cli) = doc.add_page(Mm(210.0), Mm(297.0), "layer 1");

        current_page_index = cpi;
        current_layer_index = cli;
    }
    layout_span.exit();

    let span = span!(Level::TRACE, "Writing File");
    let _guard = span.enter();

    doc.save(&mut BufWriter::new(File::create("output.pdf").unwrap()))
        .unwrap();
}
