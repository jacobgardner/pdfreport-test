use crate::{
    error::{DocumentGenerationError, UserInputError},
    fonts::{FontSlant, FontWeight, FontAttributes},
    values::{Color, Pt}, paragraph_layout::{RenderedTextBlock, RenderedTextLine, LineMetrics}, rich_text::{RichText, RichTextSpan},
};

pub struct Svg {
    parsed_content: String,
    width: Pt,
    height: Pt,
}

const SUPPORTED_SVG_TEXT_ATTRIBUTES: [&str; 10] = [
    "id",
    "x",
    "y",
    "font-weight",
    "font-style",
    "font-size",
    "fill",
    "text-anchor",
    "font-family",
    "dominant-baseline",
];

impl Svg {
    pub fn new(content: String) -> Result<Self, DocumentGenerationError> {
        let doc = roxmltree::Document::parse(&content).unwrap();

        let svg_node = doc
            .descendants()
            .find(|node| node.has_tag_name("svg"))
            .expect("Every SVG should have an <svg /> node.");

        let svg_width = svg_node
            .attribute("width")
            .map(|width| Pt::try_from(width).unwrap());
        let svg_height = svg_node
            .attribute("height")
            .map(|height| Pt::try_from(height).unwrap());

        let viewbox = svg_node
            .attribute("viewbox")
            .map(|viewbox| -> Result<_, DocumentGenerationError> {
                let p: Vec<_> = viewbox
                    .split(" ")
                    .map(|unit| Pt::try_from(unit))
                    .collect::<Result<Vec<_>, _>>()?;

                assert_eq!(p.len(), 4);

                let [x_offset, y_offset, width, height] = p[..];

                Ok((x_offset, y_offset, width, height))
            })
            .transpose()?;

        let (width, height) = if let (Some(width), Some(height)) = (svg_width, svg_height) {
            (width, height)
        } else if let Some((x_offset, y_offset, viewbox_width, viewbox_height)) = viewbox {
            if let Some(width) = svg_width {
                let scale = width / viewbox_width;

                (width, viewbox_height * scale)
            } else if let Some(height) = svg_height {
                let scale = height / viewbox_height;

                (viewbox_width * scale, height)
            } else {
                (viewbox_width, viewbox_height)
            }
        } else {
            return Err(UserInputError::SvgParseError {
                message: "SVG node must have width & height OR viewBox to determine size"
                    .to_string(),
            }
            .into());
        };

        for node in doc.descendants().filter(|n| n.has_tag_name("text")) {
            // The SVG units are in px by default, and we're assuming that here.
            //  We have to convert that to Pt which printpdf has a method for, but it
            //  takes a usize, but the svg pixels can be fractions so... we replicate that
            //  here
            //

            // TODO: Document/warn that we currently do NOT support text nodes in nested transformations
            //  OR add support for it.
            for _ancestor in node.ancestors() {
                // Here check if any ancestors do transformations and warn about
                // them OR augment the x/y below based on them
            }

            let unsupported_attribute = node.attributes().iter().find(|a| {
                !SUPPORTED_SVG_TEXT_ATTRIBUTES.contains(&a.name().to_lowercase().as_str())
            });

            if let Some(unsupported_attribute) = unsupported_attribute {
                panic!(
                    "<text .../> attribute, {}, is not yet supported",
                    unsupported_attribute.name()
                );
            }

            let x = Pt::try_from(node.attribute("x").unwrap_or("0"))?;
            let y = Pt::try_from(node.attribute("y").unwrap_or("0"))?;
            let weight = FontWeight::from(node.attribute("font-weight").unwrap_or("regular"));
            let font_style = FontSlant::from(node.attribute("font-style").unwrap_or("normal"));
            let font_size = Pt::try_from(node.attribute("font-size").unwrap_or("12"))?;
            let fill = Color::try_from(node.attribute("fill").unwrap_or("#000000"))?;
            let anchor = node.attribute("text-anchor").unwrap_or("start");
            let font_stack = node.attribute("font-family").unwrap_or("sans-serif");
            let dominant_baseline = node.attribute("dominant-baseline").unwrap_or("auto");

            // TODO: Don't hardcode
            let found_font = "Inter"; // self.find_best_font_from_stack(font_stack)?;

            if !node.children().all(|n| n.is_text()) {
                panic!("For <text>, we only support all text child nodes for now");
            }

            let node_text = node.text().unwrap().trim();

            let text_block = RenderedTextBlock {
                lines: vec![RenderedTextLine {
                    rich_text: RichText(vec![RichTextSpan {
                        text: node_text.to_string(),
                        attributes: FontAttributes {
                            weight,
                            style: font_style,
                        },
                        font_family: found_font.to_string(),
                        size: font_size,
                        color: fill,
                        letter_spacing: Pt(0.),
                        line_height: 1.,
                    }]),
                    line_metrics: LineMetrics {
                        ascent: Pt(5.),
                        descent: Pt(5.),
                        baseline: Pt(5.),
                        height: Pt(5.),
                        width: Pt(5.),
                        left: Pt(5.),
                    },
                }],
            };

            // self.draw_text_block(
            //     &PaginatedNode {
            //             page_layout: NodeLayout {
            //                 left: paginated_node.page_layout.left + x,
            //                 right: paginated_node.page_layout.right + x,
            //                 top: paginated_node.page_layout.top + y,
            //                 ..paginated_node.page_layout.clone()
            //             },
            //             ..paginated_node.clone()
            //             // page_layout: (),
            //             // page_index: (),
            //             // drawable_node: (),
            //         },
            //     &Style::Unmergeable::default(),
            //     &text_block,
            // )
            // .unwrap();

            // let rich = RichText::new(
            //     node_text,
            //     RichTextStyle {
            //         font_family: String::from(found_font),
            //         font_size,
            //         weight,
            //         style: font_style,
            //         color: (fill.0 as f32, fill.1 as f32, fill.2 as f32),
            //     },
            // );

            // let paragraph = layout.compute_paragraph_layout(&rich, Pt(1000.0));
            // assert_eq!(paragraph.line_metrics.len(), 1);

            // let line_metric = paragraph.line_metrics.first().unwrap();

            // let x_offset = match anchor.to_lowercase().as_str() {
            //     "start" => Pt(0.0),
            //     "middle" => line_metric.width / 2.,
            //     "end" => line_metric.width,
            //     _ => panic!(""),
            // };

            // let y_offset = match dominant_baseline.to_lowercase().as_str() {
            //     "auto" => Pt(0.),
            //     // TODO: This is wrong, but good enough for initial testing...
            //     "middle" | "central" => (line_metric.ascent - line_metric.descent) / 2.,
            //     baseline => panic!("{} as dominant-baseline is not yet supported", baseline),
            // };

            // layer.set_text_matrix(TextMatrix::Translate(
            //     start.x + x - x_offset,
            //     start.y + y - y_offset,
            // ));

            // let font_lookup = FontLookup {
            //     family_name: found_font,
            //     weight,
            //     style: font_style,
            // };

            // let current_font = self.writer.lookup_font(&font_lookup)?;

            // layer.set_font(current_font, font_size.0);
            // layer.set_fill_color(printpdf::Color::Rgb(Rgb::new(fill.0, fill.1, fill.2, None)));

            // self.write_text(&layer, node_text, &font_lookup)?;
        }

        Ok(Self {
            parsed_content: content,
            width,
            height,
        })
    }

    pub fn text_from_dims(&self, width: f64, height: f64) {}
}
