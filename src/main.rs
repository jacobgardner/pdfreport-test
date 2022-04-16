// Compute layout -> Draw Shit -> Draw Text -> Profit

use pdf_writer::PdfWriter;
use printpdf::{Mm, Point, Pt};
use rich_text::{RichText, RichTextStyle, RichTextStyleChanges};
use text_layout::TextLayout;
use tracing::{span, Level};

mod fonts;
mod line_metric;
mod math;
mod pdf_writer;
mod rich_text;
mod text_layout;
mod paginated_layout;
mod styles;
mod dom;
mod block_layout;

const SVG: &str = include_str!("../assets/svg-test.svg");

fn build_text() -> String {
    r#"Chapter 1: Your approach to Work

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
    #[cfg(feature = "trace_chrome")]
    let _guard = {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::prelude::*;
        let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        _guard
    };

    let output_text = build_text();

    let mut pdf_writer = PdfWriter::new();
    let text_layout = TextLayout::new();

    let span = span!(Level::DEBUG, "Full Time");
    let _guard = span.enter();

    let mut page_writer = pdf_writer.get_page(0);

    let layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();


    let page_count = 1;

    for i in 0..page_count {
        let default_style = RichTextStyle {
            font_size: Pt(14.),
            weight: rich_text::FontWeight::Regular,
            italic: false,
            color: (0.267, 0.29, 0.353),
        };

        let mut rich_text = RichText::new(&output_text[i..], default_style);

        rich_text
            .push_style(
                RichTextStyleChanges {
                    font_size: Some(Pt(32.)),
                    weight: Some(rich_text::FontWeight::Bold),
                    ..Default::default()
                },
                0..32,
            )
            .push_style(
                RichTextStyleChanges {
                    color: Some((i as f32 / page_count as f32, 0., 0.)),
                    italic: Some(true),
                    ..Default::default()
                },
                16..32,
            );

        // // We have to change the string every time otherwise Skia caches the
        // // layout calculation and we cheat in performance
        // let page_string = &output_string[i..];

        // 20 Mm of padding on left & right
        let text_width = Mm(210. - 40.).into_pt();

        let paragraph_metrics = text_layout.compute_paragraph_layout(&rich_text, text_width);

        page_writer
            .draw_rect(
                Point::new(Mm(20.), Mm(280.)),
                Point::new(
                    Mm(20.) + text_width.into(),
                    Mm(280.) - paragraph_metrics.height.into(),
                ),
                Some(Pt(5.))
            )
            .write_lines(
                Point::new(Mm(20.), Mm(280.)),
                &text_layout.typeface,
                &rich_text,
                paragraph_metrics.line_metrics,
            );
        
        page_writer.draw_svg(Point::new(Mm(20.), Mm(280.)), &SVG);

        page_writer = pdf_writer.add_page();
    }
    layout_span.exit();

    pdf_writer.save("output.pdf");
}
