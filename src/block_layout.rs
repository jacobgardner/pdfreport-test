use printpdf::DirectFontRef;
use stretch::{geometry::Rect, style::*, Stretch};

use crate::dom::{PdfLayout, MergeableStyle, Style};

impl From<Style> for stretch::style::Style {
    fn from(s: Style) -> Self {
        Self {
            display: Display::Flex,
            flex_direction: if s.flex.direction == crate::dom::Direction::Row {
                FlexDirection::Row
            } else {
                FlexDirection::Column
            },
            margin: Rect {
                top: Dimension::Points(s.margin.top),
                end: Dimension::Points(s.margin.right),
                bottom: Dimension::Points(s.margin.bottom),
                start: Dimension::Points(s.margin.left),
            },
            ..Default::default()
        }
    }
}

pub fn layout_pdf(pdf: &PdfLayout) -> Result<(), stretch::Error> {
    let mut stretch = Stretch::new();

    let style_stack = vec![Style::default()];

    let current_style = style_stack.last().unwrap().clone();
    let node = stretch.new_node(current_style.into(), vec![])?;

    let l = stretch.layout(node)?;

    Ok(())
}
