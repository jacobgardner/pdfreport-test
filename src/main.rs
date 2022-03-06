// Compute layout -> Draw Shit -> Draw Text -> Profit

use pdf_writer::PdfWriter;
use printpdf::{Mm, Point};
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
mod math;
mod pdf_writer;
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

    let mut pdf_writer = PdfWriter::new();
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

    let mut page_writer = pdf_writer.get_page(0);

    let layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();

    for i in 1..1000 {

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, text_layout.font_collection.clone());
        paragraph_builder.push_style(&ts);
        // We have to change the string every time otherwise Skia caches the
        // layout calculation and we cheat in performance
        let page_string = &output_string[i..];
        paragraph_builder.add_text(page_string);

        let mut paragraph = paragraph_builder.build();
        let span = span!(Level::TRACE, "Computing layout").entered();
        paragraph.layout(Mm(210. - 40.).into_pt().0 as f32);
        span.exit();

        let line_metrics = paragraph.get_line_metrics();


        page_writer
            .draw_rect(
                Point::new(Mm(20.), Mm(20.)),
                Point::new(Mm(20. + 210. - 40.), Mm(280.)),
            )
            .write_lines(
                Point::new(Mm(20.), Mm(280.)),
                &text_layout.typeface,
                page_string,
                line_metrics,
            );

        page_writer = pdf_writer.add_page();
    }
    layout_span.exit();

    pdf_writer.save("output.pdf");
}
