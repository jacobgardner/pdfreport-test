use std::collections::HashMap;



use optional_merge_derive::mergable;
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

// macro_rules! nested_merge  {
//     ($name : ident) => {
//         impl Merges for Option<$name> {
//             fn merge(&self, rhs: &Self) -> Self {
//                 rhs.as_ref().or(self.as_ref()).map(|f| f.clone())
//             }           
//         }
//     };
//     ($name: ident, $($remain:ident),+) => {
//         nested_merge!($name);
//         nested_merge!($($remain),+);
//     }
// }

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

primitive_merge!(f32, String, Direction);


trait Merges: Sized + Clone {
    fn merge(&self, rhs: &Self) -> Self;

    fn merge_optional(&self, rhs: &Option<Self>) -> Option<Self> {
        if let Some(op) = rhs {
            Some(self.merge(&op))
        } else {
            Some(self.clone())
        }
    }
}

#[mergable]
#[derive(Deserialize, Clone)]
struct BorderRadiusStyle {
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
    top_left: f32,
}

impl Default for BorderRadiusStyle {
    fn default() -> Self {
        Self {
            top_right: Some(0.),
            bottom_right: Some(0.),
            bottom_left: Some(0.),
            top_left: Some(0.),
        }
    }
}

// impl Merges for BorderRadiusStyle {
//     fn merge(&self, rhs: &Self) -> Self {
//         Self {
//             top_right: rhs.top_right.or(self.top_right),
//             bottom_right: rhs.bottom_right.or(self.bottom_right),
//             bottom_left: rhs.bottom_left.or(self.bottom_left),
//             top_left: rhs.top_left.or(self.top_left),
//         }
//     }
// }

#[mergable]
#[derive(Deserialize, Clone)]
pub struct BorderStyle {
    width: f32,
    color: Color,
    radius: BorderRadiusStyle,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            width: Some(0.),
            color: Some(String::from("#000000")),
            radius: Some(BorderRadiusStyle::default()),
        }
    }
}

fn merge_clone<T: Clone>(lhs: &Option<T>, rhs: &Option<T>) -> Option<T> {
    rhs.as_ref().or(lhs.as_ref()).map(|f| f.clone())
}

// impl Merges for BorderStyle {
//     fn merge(&self, rhs: &Self) -> Self {
//         // let color_ref = rhs.color.as_ref().or(self.color.as_ref()).map(|f| f.clone());

//         Self {
//             width: rhs.width.or(self.width),
//             color: merge_clone(&self.color, &rhs.color),
//             radius: rhs.radius.clone().unwrap().merge_optional(&self.radius),
//         }
//     }
// }

#[derive(Deserialize, Clone, Copy, PartialEq)]
pub enum Direction {
    Column,
    Row,
}

#[mergable]
#[derive(Deserialize, Clone)]
pub struct FlexStyle {
    // size: Option<f32>,
    pub direction: Direction,
    // wrap: Option<bool>,
}

// impl Merges for FlexStyle {
//     fn merge(&self, rhs: &Self) -> Self {
//         Self {
//             direction: rhs.direction.or(self.direction), // direction: Some(Direction::Vertical),
//         }
//     }
// }

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction: Some(Direction::Column),
        }
    }
}

// impl<T: Clone + Merges> Merges for Option<T> {
//     fn merge(&self, rhs: &Self) -> Self {
//         rhs.as_ref().or(self.as_ref()).map(|f| f.clone())
//         // if let Some(lhs) = self {
//         //     rhs.or(lhs).clone()
//         // } else {
//         //     rhs.clone()
//         // }
//     }
// }


// impl<T: Clone> Merges for Option<T> {
//     fn merge(&self, rhs: &Self) -> Self {
//         rhs.as_ref().or(self.as_ref()).map(|f| f.clone())
//     }
// }

#[mergable]
#[derive(Deserialize, Clone)]
pub struct MarginStyle {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for MarginStyle {
    fn default() -> Self {
        Self {
            top: Some(0.),
            right: Some(0.),
            bottom: Some(0.),
            left: Some(0.),
        }
    }
}

#[mergable]
#[derive(Deserialize, Clone)]
pub struct Style {
    pub border: BorderStyle,
    pub color: Color,
    pub margin: MarginStyle,
    pub background_color: Color,
    pub flex: FlexStyle,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            border: Some(BorderStyle::default()),
            color: Some(String::from("#000000")),
            background_color: Some(String::from("#FFFFFF")),
            flex: Some(FlexStyle::default()),
            margin: Some(MarginStyle::default()),
        }
    }
}

impl Style {
    pub fn merge_style(&self, rhs: &Style) -> Style {
        Style {
            border: self.border.clone().unwrap().merge_optional(&rhs.border),
            flex: self.flex.clone().unwrap().merge_optional(&rhs.flex),
            color: merge_clone(&self.color, &rhs.color),
            background_color: merge_clone(&self.background_color, &rhs.background_color),
            margin: merge_clone(&self.margin, &rhs.margin),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TextChild {
    Content(String),
    TextNode(TextNode),
}

#[derive(Deserialize)]
pub struct TextNode {
    styles: Vec<String>,
    children: Vec<TextChild>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    StyledNode {
        styles: Vec<Style>,
        children: Vec<Node>,
    },
    Text(TextNode),
    ImageNode {},
}

#[derive(Deserialize)]
pub struct FontInformation {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfLayout {
    pub fonts: Vec<FontInformation>,
    pub styles: HashMap<String, Style>,
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
                "type": "StyledNode",
                "styles": [],
                "children": [{
                    "type": "Text",
                    "styles": ["h1"],
                    "children": ["This is some header text ", {"styles": ["italic"], "children": ["italic text"]}] 
                }]
            }
        }"##).unwrap();

        assert_eq!(dom.fonts.len(), 0);
        assert_eq!(
            dom.styles.get("h1").unwrap().color.as_ref().unwrap(),
            "#ABCDEF"
        );
        if let Node::StyledNode { styles, children } = dom.root {
            assert_eq!(styles.len(), 0);
            assert_eq!(children.len(), 1);
        } else {
            unreachable!()
        }
    }
}
