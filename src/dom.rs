use optional_merge_derive::MergeOptional;

use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(MergeOptional, Clone)]
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

#[derive(MergeOptional, Clone)]
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

#[derive(MergeOptional, Clone)]
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

#[derive(MergeOptional, Clone)]
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

#[derive(MergeOptional, Clone)]
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

#[derive(Deserialize)]
#[serde(untagged)]
enum TextChild {
    Content(String),
    TextNode(TextNode),
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TextNode {
    styles: Vec<String>,
    children: Vec<TextChild>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ImageNode {
    content: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Styled {
        styles: Vec<String>,
        children: Vec<Node>,
    },
    Text(TextNode),
    Image(ImageNode),
}

#[derive(Deserialize)]
pub struct FontInformation {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PdfLayout {
    pub fonts: Vec<FontInformation>,
    pub styles: HashMap<String, MergeableStyle>,
    pub root: Node,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_json() {
        let dom: PdfLayout =  serde_json::from_str(r##"{
            "fonts": [],
            "styles": {
                "h1": {
                    "color": "#ABCDEF",
                    "flex": {
                        "direction": "Column"
                    },
                    "margin": {
                        "bottom": 4
                    },
                    "border": {
                        "width": 1,  
                        "color": "#ABCDEF",
                        "radius": {
                            "topRight": 5,
                            "bottomRight": 5
                        }
                    }
                },
                "italic": {
                     
                }
            },
            "root": {
                "type": "Styled",
                "styles": [],
                "children": [{
                    "type": "Text",
                    "styles": ["h1"],
                    "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text"]}] 
                }, {
                    "type": "Image",
                    "content": "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\" role=\"img\" aria-label=\"22\" width=\"73\" height=\"73\" viewBox=\"0 0 73 73\" xmlns=\"http://www.w3.org/2000/svg\"><circle class=\"donutMetric__innerCircle\" cx=\"36.5\" cy=\"36.5\" r=\"25\" fill=\"#D3D1E6\" /></svg>"
                }]
            }
        }"##).unwrap();

        assert_eq!(dom.fonts.len(), 0);

        let style = Style::default().merge_style(dom.styles.get("h1").unwrap());

        assert_eq!(style.color, "#ABCDEF");
        assert_eq!(style.flex.direction, Direction::Column);
        assert_eq!(style.background_color, "#FFFFFF");
        assert_eq!(style.border.radius.top_right, 5.);
        assert_eq!(style.border.radius.bottom_right, 5.);
        assert_eq!(style.border.radius.top_left, 0.);
        assert_eq!(style.border.radius.bottom_left, 0.);

        if let Node::Styled { styles, children } = dom.root {
            assert_eq!(styles.len(), 0);
            assert_eq!(children.len(), 2);

            if let Node::Text(node) = &children[0] {
                assert!(node.styles.contains(&String::from("h1")));
            } else {
                unreachable!();
            }

            if let Node::Image(node) = &children[1] {
                assert!(node.content.contains("<svg"));
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }
    }
}
