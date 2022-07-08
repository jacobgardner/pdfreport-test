use crate::{
    error::{DocumentGenerationError, UserInputError},
    fonts::{FontAttributes, FontSlant, FontWeight},
    paragraph_layout::{ParagraphLayout, ParagraphStyle, RenderedTextBlock, TextAlign},
    rich_text::{RichText, RichTextSpan},
    values::{Color, Pt},
};

#[derive(Debug, Clone)]
pub struct Svg {
    pub content: String,
    width: Pt,
    height: Pt,
    text_blocks: Vec<((Pt, Pt), RenderedTextBlock)>,
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
    pub fn new(
        content: String,
        paragraph_layout: &ParagraphLayout,
    ) -> Result<Self, DocumentGenerationError> {
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
            .attribute("viewBox")
            .map(|viewbox| -> Result<_, DocumentGenerationError> {
                let p: Vec<_> = viewbox
                    .split(" ")
                    .map(|unit| Pt::try_from(unit))
                    .collect::<Result<Vec<_>, _>>()?;

                if let [x_offset, y_offset, width, height] = p[..] {
                    Ok((x_offset, y_offset, width, height))
                } else {
                    Err(UserInputError::SvgParseError {
                        message: "viewBox must have exactly 4 elements".to_string(),
                    }
                    .into())
                }
            })
            .transpose()?;

        let (width, height) = if let (Some(width), Some(height)) = (svg_width, svg_height) {
            (width, height)
        } else if let Some((_, _, viewbox_width, viewbox_height)) = viewbox {
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

        let mut text_blocks = vec![];

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
            let mut y = Pt::try_from(node.attribute("y").unwrap_or("0"))?;
            let weight = FontWeight::from(node.attribute("font-weight").unwrap_or("regular"));
            let font_style = FontSlant::from(node.attribute("font-style").unwrap_or("normal"));
            let font_size = Pt::try_from(node.attribute("font-size").unwrap_or("12"))?;
            let fill = Color::try_from(node.attribute("fill").unwrap_or("#000000"))?;
            let anchor = TextAlign::from_anchor(node.attribute("text-anchor").unwrap_or("start"));
            let font_stack = node.attribute("font-family").unwrap_or("sans-serif");
            let dominant_baseline = node.attribute("dominant-baseline").unwrap_or("auto");

            // TODO: Don't hardcode
            let found_font = "Inter"; // self.find_best_font_from_stack(font_stack)?;

            if !node.children().all(|n| n.is_text()) {
                panic!("For <text>, we only support all text child nodes for now");
            }

            let node_text = node.text().unwrap().trim();

            let rich_text = RichText(vec![RichTextSpan {
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
            }]);

            let mut text_block = paragraph_layout.calculate_layout(
                ParagraphStyle::left(),
                &rich_text,
                // Large enough to not wrap, hopefully
                Pt(f64::MAX),
            )?;

            let line_metric = text_block.lines.first().unwrap().line_metrics.clone();

            // I have no maths proving these. Mostly these are the values that
            // ended up working
            y = match dominant_baseline.to_lowercase().as_ref() {
                "auto" => y - line_metric.ascent,
                "central" => y - (line_metric.ascent + line_metric.descent) / 2.,
                "middle" => y - line_metric.ascent + line_metric.descent,
                "hanging" => y - line_metric.descent,
                _ => y,
            };

            text_block.lines.iter_mut().for_each(|line| {
                line.line_metrics.left -= match anchor {
                    TextAlign::Left => Pt(0.),
                    TextAlign::Right => line.line_metrics.width,
                    TextAlign::Center => line.line_metrics.width / 2.,
                };
            });

            text_blocks.push(((x, y), text_block));
        }

        Ok(Self {
            content,
            width,
            height,
            text_blocks,
        })
    }

    pub fn computed_scale(&self, width: Pt, height: Pt) -> (f64, f64) {
        (width / self.width, height / self.height)
    }

    pub fn text_from_dims(
        &self,
        width: Pt,
        height: Pt,
    ) -> impl Iterator<Item = ((Pt, Pt), RenderedTextBlock)> + '_ {
        let (scale_x, scale_y) = self.computed_scale(width, height);

        self.text_blocks
            .iter()
            .cloned()
            .map(move |(mut point, mut block)| {
                for line in block.lines.iter_mut() {
                    for rich_text_span in line.rich_text.0.iter_mut() {
                        rich_text_span.line_height *= scale_y;
                        rich_text_span.size *= scale_y;
                    }

                    line.line_metrics.ascent *= scale_y;
                    line.line_metrics.baseline *= scale_y;
                    line.line_metrics.descent *= scale_y;
                    line.line_metrics.height *= scale_y;
                    line.line_metrics.left *= scale_x;
                    line.line_metrics.width *= scale_x;
                }

                ((point.0 * scale_x, point.1 * scale_y), block)
            })
    }
}
