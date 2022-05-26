use polyhorn_yoga as yoga;

use yoga::{FlexDirection, Wrap, Edge, StyleUnit, Align, FlexStyle::Flex};

use crate::stylesheet::{Direction, FlexWrap, Style, FlexAlign};

impl From<Direction> for FlexDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Column => FlexDirection::Column,
            Direction::Row => FlexDirection::Row 
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

impl From<Style::Unmergeable> for yoga::Node {
    fn from(style: Style::Unmergeable) -> Self {
        let mut layout_node = yoga::Node::new();

        layout_node.set_border(Edge::All, style.border.width);

        layout_node.set_margin(Edge::Top, StyleUnit::Point(style.margin.top.into()));
        layout_node.set_margin(Edge::Right, StyleUnit::Point(style.margin.right.into()));
        layout_node.set_margin(Edge::Bottom, StyleUnit::Point(style.margin.bottom.into()));
        layout_node.set_margin(Edge::Left, StyleUnit::Point(style.margin.left.into()));

        layout_node.set_padding(Edge::Top, StyleUnit::Point(style.padding.top.into()));
        layout_node.set_padding(Edge::Right, StyleUnit::Point(style.padding.right.into()));
        layout_node.set_padding(Edge::Bottom, StyleUnit::Point(style.padding.bottom.into()));
        layout_node.set_padding(Edge::Left, StyleUnit::Point(style.padding.left.into()));

        layout_node.set_flex_direction(style.flex.direction.into());
        layout_node.set_flex_wrap(style.flex.wrap.into());
        layout_node.set_align_items(style.flex.align_items.into());
        layout_node.set_align_self(style.flex.align_self.into());
        layout_node.set_flex_grow(style.flex.grow);
        layout_node.set_flex_shrink(style.flex.shrink);
        // layout_node.set_flex_basis(style.flex.basis);

        layout_node
    }
}