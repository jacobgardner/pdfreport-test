// Compute layout -> Draw Shit -> Draw Text -> Profit

use std::{fs::File, io::BufWriter};

use piet_common::{
    Device, FontWeight, LineMetric, RenderContext, Text, TextAttribute, TextLayout,
    TextLayoutBuilder,
};
use printpdf::{Mm, PdfDocument, Pt, Px, TextMatrix, Color, Rgb};
use tracing::{span, Level};
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, FmtSubscriber,
};

fn main() {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    let span = span!(Level::DEBUG, "Full Time");
    let _guard = span.enter();

    let span = span!(Level::TRACE, "Build context").entered();

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(1024, 1024, 96.).unwrap();
    let mut rc = bitmap.render_context();

    span.exit();

    let text = rc.text();
    let piet_font = text
        .load_font(include_bytes!("../assets/fonts/inter/Inter-Regular.ttf"))
        .unwrap();

    let output_text = r#"
Chapter 1Y ouAr cppruaWh tu kur2
Introduction
W H Y T H I S M AT T E R S
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
CA L LO U T
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
E X P L A I N T H E G R O U P I N G S
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
CA L LO U T
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
H OW TO U S E I T
Explaining how every measure should be completed, but allows for the user to browse at their own pace and leisure.
They're welcome to scroll through, but a deep dive is necessary. At the end of each measure the users will have two
actions to take: adding items to their action planner or marking
the measure as complete.
CA L LO U T
These callout boxes will
provide supplemental information pertaining to the
adjacent section (or the
section above, depending
on your screen size).
Decision Making
W H Y T H I S M AT T E R S
    "#
    .replace("\n", " ")
    .replace("  ", " ");

    let (doc, page1, layer1) = PdfDocument::new("DVP Report", Mm(210.0), Mm(297.0), "Layer 1");

    let font = doc
        .add_external_font(File::open("assets/fonts/inter/Inter-Regular.ttf").unwrap())
        .unwrap();

    let mut current_page_index = page1;
    let mut current_layer_index = layer1;


    let layout_span = span!(Level::DEBUG, "Layout & Building PDF").entered();

    for _ in 0..54 {
        let current_layer = doc
            .get_page(current_page_index)
            .get_layer(current_layer_index);

        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.267, 0.29, 0.353, None)));

        let span = span!(Level::TRACE, "Computing layout").entered();
        let layout = text
            .new_text_layout(output_text.clone())
            .font(piet_font.clone(), 12.0)
            .max_width(Pt::from(Mm(150.0)).0)
            .default_attribute(TextAttribute::Weight(FontWeight::BLACK))
            .build()
            .unwrap();
        span.exit();

        let layout_y_start = Pt::from(Mm(280.0));
        for line_index in 0..layout.line_count() {
            let line_metric = layout.line_metric(line_index).unwrap();

            // let line_y_start = Mm(layout_y_start - 30. * line_index as f64); // - Px(line_metric.y_offset as usize).into_pt(96.0).into();
            let line = layout.line_text(line_index).unwrap();
            // println!("y_start: {:?}", line_y_start);
            // println!("line: {}", line);

            current_layer.set_text_matrix(TextMatrix::Translate(
                Pt(10.0),
                layout_y_start - Pt(line_metric.y_offset),
            ));
            // current_layer.set_text_cursor(Mm(0.), Pt(-line_metric.baseline).into());
            current_layer.write_text(line, &font);
            current_layer.add_line_break();
        }
        current_layer.end_text_section();

        let (cpi, cli) = doc.add_page(Mm(210.0), Mm(297.0), "layer 1");

        current_page_index = cpi;
        current_layer_index = cli;
    }
    layout_span.exit();

    // current_layer.use_text(output_text, 12.0, Mm(0.0), Mm(200.0), &font);

    // Text::
    // TextLayoutBuilder::font(self, font, font_size)

    rc.finish().unwrap();

    let span = span!(Level::TRACE, "Writing File");
    let _guard = span.enter();

    doc.save(&mut BufWriter::new(File::create("output.pdf").unwrap()))
        .unwrap();
}
