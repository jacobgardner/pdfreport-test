use stretch2::{
    geometry::{Rect, Size},
    style::*,
};

use stretch2 as stretch;

use crate::{
    dom::style::{Direction, FlexAlign},
    dom::Style,
    units::{percent_to_num, unit_to_pt, MeasurementParseError},
};

fn string_to_dim(s: &str) -> Result<Dimension, MeasurementParseError> {
    Ok(match s.to_lowercase().as_str() {
        "undefined" => Dimension::Undefined,
        "auto" => Dimension::Auto,
        s => {
            if s.contains('%') {
                Dimension::Percent(percent_to_num(s)? as f32)
            } else {
                let pt = unit_to_pt(s)?;

                Dimension::Points(pt.0 as f32)
            }
        }
    })
}

impl TryFrom<Style> for stretch::style::Style {
    type Error = MeasurementParseError;
    // impl From<Style> for stretch::style::Style {
    fn try_from(s: Style) -> Result<Self, Self::Error> {
        Ok(Self {
            display: Display::Flex,
            flex_direction: if s.flex.direction == Direction::Row {
                FlexDirection::Row
            } else {
                FlexDirection::Column
            },
            align_items: match s.flex.align_items {
                FlexAlign::Auto | FlexAlign::Stretch => AlignItems::Stretch,
                FlexAlign::FlexStart => AlignItems::FlexStart,
                FlexAlign::FlexEnd => AlignItems::FlexEnd,
                FlexAlign::Center => AlignItems::Center,
                FlexAlign::Baseline => AlignItems::Baseline,
            },
            align_self: match s.flex.align_items {
                FlexAlign::Auto => AlignSelf::Auto,
                FlexAlign::Stretch => AlignSelf::Stretch,
                FlexAlign::FlexStart => AlignSelf::FlexStart,
                FlexAlign::FlexEnd => AlignSelf::FlexEnd,
                FlexAlign::Center => AlignSelf::Center,
                FlexAlign::Baseline => AlignSelf::Baseline,
            },
            margin: Rect {
                top: Dimension::Points(s.margin.top),
                end: Dimension::Points(s.margin.right),
                bottom: Dimension::Points(s.margin.bottom),
                start: Dimension::Points(s.margin.left),
            },
            padding: Rect {
                top: Dimension::Points(s.padding.top),
                end: Dimension::Points(s.padding.right),
                bottom: Dimension::Points(s.padding.bottom),
                start: Dimension::Points(s.padding.left),
            },
            border: Rect {
                top: Dimension::Points(s.border.width),
                end: Dimension::Points(s.border.width),
                bottom: Dimension::Points(s.border.width),
                start: Dimension::Points(s.border.width),
            },
            size: Size {
                width: string_to_dim(&s.width)?,
                height: string_to_dim(&s.height)?,
            },
            flex_grow: s.flex.grow,
            flex_shrink: s.flex.shrink,
            flex_basis: string_to_dim(&s.flex.basis)?,
            ..Default::default()
        })
    }
}
