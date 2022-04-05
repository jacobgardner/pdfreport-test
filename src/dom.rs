use std::collections::HashMap;

use serde::Deserialize;

type Color = String;

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

#[derive(Deserialize, Clone)]
struct BorderRadiusStyle {
    top_right: Option<f32>,
    bottom_right: Option<f32>,
    bottom_left: Option<f32>,
    top_left: Option<f32>,
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

impl Merges for BorderRadiusStyle {
    fn merge(&self, rhs: &Self) -> Self {
        Self {
            top_right: rhs.top_right.or(self.top_right),
            bottom_right: rhs.bottom_right.or(self.bottom_right),
            bottom_left: rhs.bottom_left.or(self.bottom_left),
            top_left: rhs.top_left.or(self.top_left),
        }
    }
}

#[derive(Deserialize, Clone)]
struct BorderStyle {
    width: Option<f32>,
    color: Option<Color>,
    radius: Option<BorderRadiusStyle>,
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

impl Merges for BorderStyle {
    fn merge(&self, rhs: &Self) -> Self {
        // let color_ref = rhs.color.as_ref().or(self.color.as_ref()).map(|f| f.clone());

        Self {
            width: rhs.width.or(self.width),
            color: merge_clone(&self.color, &rhs.color),
            radius: rhs.radius.clone().unwrap().merge_optional(&self.radius),
        }
    }
}

#[derive(Deserialize, Clone)]
struct FlexStyle {
    // size: Option<f32>,
// direction: Option<bool>,
// wrap: Option<bool>,
}

impl Merges for FlexStyle {
    fn merge(&self, rhs: &Self) -> Self {
        Self {}
    }
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Deserialize, Clone)]
struct Style {
    border: Option<BorderStyle>,
    color: Option<Color>,
    background_color: Option<Color>,
    flex: Option<FlexStyle>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            border: Some(BorderStyle::default()),
            color: Some(String::from("#000000")),
            background_color: Some(String::from("#FFFFFF")),
            flex: Some(FlexStyle::default()),
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
struct TextNode {
    styles: Vec<String>,
    children: Vec<TextChild>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Node {
    StyledNode {
        styles: Vec<Style>,
        children: Vec<Node>,
    },
    Text(TextNode),
    ImageNode {},
}

#[derive(Deserialize)]
struct FontInformation {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PdfLayout {
    fonts: Vec<FontInformation>,
    styles: HashMap<String, Style>,
    root: Node,
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
