use polyhorn_yoga as yoga;

use yoga::{FlexDirection, Wrap};

use crate::stylesheet::{Direction, FlexWrap};


// TODO: Find a crate to do some of this for us
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