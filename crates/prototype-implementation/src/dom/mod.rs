// It's fun that DOM is the reverse of MOD, I guess

use serde::Deserialize;
use std::collections::HashMap;

pub mod nodes;
pub mod style;

pub use nodes::DomNode;
pub use style::{MergeableStyle, Style};

use crate::rich_text::{FontStyle, FontWeight};

#[derive(Debug, Deserialize)]
pub struct FontInfo {
    pub source: String,
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontStyle,
}

#[derive(Debug, Deserialize)]
pub struct FontFamilyInfo {
    pub family_name: String,
    pub fonts: Vec<FontInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PdfDom {
    pub fonts: Vec<FontFamilyInfo>,
    pub styles: HashMap<String, MergeableStyle>,
    pub root: DomNode,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_json() {
        let dom: PdfDom =  serde_json::from_str(r##"{
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
        assert_eq!(style.flex.direction, style::Direction::Column);
        assert_eq!(style.background_color, "#FFFFFF");
        assert_eq!(style.border.radius.top_right, 5.);
        assert_eq!(style.border.radius.bottom_right, 5.);
        assert_eq!(style.border.radius.top_left, 0.);
        assert_eq!(style.border.radius.bottom_left, 0.);

        if let DomNode::Styled(nodes::StyledNode { styles, children }) = dom.root {
            assert_eq!(styles.len(), 0);
            assert_eq!(children.len(), 2);

            if let DomNode::Text(node) = &children[0] {
                assert!(node.styles.contains(&String::from("h1")));
            } else {
                unreachable!();
            }

            if let DomNode::Image(node) = &children[1] {
                assert!(node.content.contains("<svg"));
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }
    }
}









