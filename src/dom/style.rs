use optional_merge_derive::MergeOptional;
use serde::Deserialize;

type Color = String;

macro_rules! primitive_merge  {
    ($name : ident) => {
        impl Merges for Option<$name> {
            fn merge(&self, rhs: &Self) -> Self {
                rhs.as_ref().or(self.as_ref()).map(|f| f.clone())
            }
        }
    };
    ($name: ident, $($remain:ident),+) => {
        primitive_merge!($name);
        primitive_merge!($($remain),+);
    }
}

impl<T: Merges + Clone> Merges for Option<T> {
    fn merge(&self, rhs: &Self) -> Self {
        if let Some(lhs) = self {
            if let Some(rhs) = rhs {
                Some(lhs.merge(rhs))
            } else {
                self.clone()
            }
        } else {
            rhs.clone()
        }
    }
}

primitive_merge!(f32, String, Direction, FlexWrap, FlexAlign);

trait Merges: Sized + Clone {
    fn merge(&self, rhs: &Self) -> Self;

    fn merge_optional(&self, rhs: &Option<Self>) -> Option<Self> {
        if let Some(op) = rhs {
            Some(self.merge(op))
        } else {
            Some(self.clone())
        }
    }
}

#[derive(MergeOptional, Clone, Debug)]
pub struct BorderRadiusStyle {
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
    pub top_left: f32,
}

impl Default for BorderRadiusStyle {
    fn default() -> Self {
        Self {
            top_right: 0.,
            bottom_right: 0.,
            bottom_left: 0.,
            top_left: 0.,
        }
    }
}

#[derive(MergeOptional, Clone, Debug)]
pub struct BorderStyle {
    pub width: f32,
    pub color: Color,
    #[nested]
    pub radius: BorderRadiusStyle,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            width: 0.,
            color: String::from("#000000"),
            radius: BorderRadiusStyle::default(),
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Column,
    Row,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum FlexAlign {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(MergeOptional, Clone, Debug)]
pub struct FlexStyle {
    pub direction: Direction,
    pub wrap: FlexWrap,
    pub align_items: FlexAlign,
    pub align_self: FlexAlign,
    pub grow: f32,
    pub shrink: f32,
    pub basis: String,
    // TODO: Add other attributes as needed
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction: Direction::Column,
            wrap: FlexWrap::NoWrap,
            align_items: FlexAlign::Auto,
            align_self: FlexAlign::Auto,
            grow: 0.,
            shrink: 1.,
            basis: String::from("auto"),
        }
    }
}

#[derive(MergeOptional, Clone, Debug)]
pub struct EdgeStyle {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            top: 0.,
            right: 0.,
            bottom: 0.,
            left: 0.,
        }
    }
}

#[derive(MergeOptional, Clone, Debug)]
pub struct Style {
    #[nested]
    pub border: BorderStyle,
    pub color: Color,
    #[nested]
    pub margin: EdgeStyle,
    #[nested]
    pub padding: EdgeStyle,
    pub background_color: Color,
    #[nested]
    pub flex: FlexStyle,
    pub width: String,
    pub height: String,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            border: BorderStyle::default(),
            color: String::from("#000000"),
            background_color: String::from("#FFFFFF"),
            flex: FlexStyle::default(),
            margin: EdgeStyle::default(),
            padding: EdgeStyle::default(),
            width: String::from("undefined"),
            height: String::from("undefined"),
        }
    }
}

impl Style {
    pub fn merge_style(&self, rhs: &MergeableStyle) -> Style {
        let base: MergeableStyle = MergeableStyle::from(self.clone());

        let merged: MergeableStyle = base.merge(rhs);

        merged.into()
    }
}
