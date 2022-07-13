use polyhorn_yoga as yoga;

use yoga::{Align, Edge, FlexDirection, StyleUnit, Wrap};

use crate::{
    stylesheet::{Direction, FlexAlign, FlexWrap, Style},
    values::Pt,
};

impl From<Direction> for FlexDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Column => FlexDirection::Column,
            Direction::Row => FlexDirection::Row,
        }
    }
}

impl From<FlexWrap> for Wrap {
    fn from(wrap: FlexWrap) -> Self {
        match wrap {
            FlexWrap::NoWrap => Wrap::NoWrap,
            FlexWrap::Wrap => Wrap::Wrap,
            FlexWrap::WrapReverse => Wrap::WrapReverse,
        }
    }
}

impl From<FlexAlign> for Align {
    fn from(align: FlexAlign) -> Self {
        match align {
            FlexAlign::Auto => Align::Auto,
            FlexAlign::Baseline => Align::Baseline,
            FlexAlign::Center => Align::Center,
            FlexAlign::FlexEnd => Align::FlexEnd,
            FlexAlign::FlexStart => Align::FlexStart,
            FlexAlign::Stretch => Align::Stretch,
        }
    }
}

impl From<Style> for yoga::Node {
    fn from(style: Style) -> Self {
        let mut layout_node = yoga::Node::new();

        layout_node.set_border(Edge::Top, style.border.width.top.0 as f32);
        layout_node.set_border(Edge::Right, style.border.width.right.0 as f32);
        layout_node.set_border(Edge::Bottom, style.border.width.bottom.0 as f32);
        layout_node.set_border(Edge::Left, style.border.width.left.0 as f32);

        layout_node.set_margin(Edge::Top, style.margin.top.into());
        layout_node.set_margin(Edge::Right, style.margin.right.into());
        layout_node.set_margin(Edge::Bottom, style.margin.bottom.into());
        layout_node.set_margin(Edge::Left, style.margin.left.into());

        layout_node.set_padding(Edge::Top, style.padding.top.into());
        layout_node.set_padding(Edge::Right, style.padding.right.into());
        layout_node.set_padding(Edge::Bottom, style.padding.bottom.into());
        layout_node.set_padding(Edge::Left, style.padding.left.into());

        layout_node.set_flex_direction(style.flex.direction.into());
        layout_node.set_flex_wrap(style.flex.wrap.into());
        layout_node.set_align_items(style.flex.align_items.into());
        layout_node.set_align_self(style.flex.align_self.into());
        layout_node.set_flex_grow(style.flex.grow);
        layout_node.set_flex_shrink(style.flex.shrink);
        layout_node.set_flex_basis(StyleUnit::Auto);

        layout_node
    }
}

impl From<Pt> for StyleUnit {
    fn from(pt: Pt) -> Self {
        StyleUnit::Point((pt.0 as f32).into())
    }
}
