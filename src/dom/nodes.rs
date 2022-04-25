use std::{collections::HashMap, ops::Range, slice::Iter};

use crate::dom::style::Merges;
use crate::dom::MergeableStyle;
use itertools::Itertools;
use serde::Deserialize;

use super::Style;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TextChild {
    Content(String),
    TextNode(TextNode),
}

impl TextChild {
    pub fn raw_text(&self) -> String {
        match self {
            TextChild::Content(content) => content.clone(),
            TextChild::TextNode(node) => node.raw_text(),
        }
    }

    pub fn text_range(&self, start: usize) -> Range<usize> {
        match self {
            TextChild::Content(content) => start..(start + content.len()),
            TextChild::TextNode(node) => node.text_range(start),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<TextChild>,
}

pub struct TextNodeIterator<'a> {
    root_node: &'a TextNode,
    root_style: &'a Style,
    style_map: &'a HashMap<String, MergeableStyle>,
    iter_stack: Vec<Iter<'a, TextChild>>,
    style_stack: Vec<MergeableStyle>,
    start_index_stack: Vec<usize>,
}

impl TextNode {
    pub fn raw_text(&self) -> String {
        self.children.iter().map(|t| t.raw_text()).join("")
    }

    pub fn text_range(&self, start: usize) -> Range<usize> {
        self.children.iter().fold(start..start, |acc, child| {
            start..(child.text_range(acc.end).end)
        })
    }

    pub fn iter_rich_text<'a>(
        &'a self,
        current_style: &'a Style,
        styles: &'a HashMap<String, MergeableStyle>,
    ) -> TextNodeIterator<'a> {
        TextNodeIterator {
            root_node: self,
            root_style: current_style,
            style_map: styles,
            iter_stack: Vec::new(),
            style_stack: vec![MergeableStyle::default()],
            start_index_stack: vec![0],
        }
    }
}

#[derive(PartialEq)]
pub struct TextNodeIterItem(pub Range<usize>, pub MergeableStyle);

use core::fmt::Debug;
impl Debug for TextNodeIterItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("R").field(&self.0).field(&self.1).finish()
    }
}

impl<'a> Iterator for TextNodeIterator<'a> {
    type Item = TextNodeIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_stack.is_empty() {
            let root_iter = self.root_node.children.iter();

            self.iter_stack.push(root_iter);

            // The root_style has already had its merged style calculated
            return Some(TextNodeIterItem(
                self.root_node.text_range(0),
                MergeableStyle::default(),
            ));
        }

        while !self.iter_stack.is_empty() {
            let iter = self.iter_stack.last_mut().unwrap();

            if let Some(child) = iter.next() {
                let start_index = *self.start_index_stack.last().unwrap();
                match &child {
                    TextChild::Content(content) => {
                        let end_index = start_index + content.len();

                        *self.start_index_stack.last_mut().unwrap() = end_index;

                        return Some(TextNodeIterItem(
                            start_index..end_index,
                            self.style_stack.last().unwrap().clone(),
                        ));
                    }
                    TextChild::TextNode(node) => {
                        // FIXME: This is replicated from block_layout
                        let mut current_style = self.style_stack.last().unwrap().clone();
                        for style_name in node.styles.iter() {
                            // FIXME: Don't unwrap, return error
                            let merging_style = self.style_map.get(style_name).unwrap();
                            current_style = current_style.merge(merging_style);
                        }

                        self.start_index_stack
                            .push(*self.start_index_stack.last().unwrap());
                        self.iter_stack.push(node.children.iter());
                        self.style_stack.push(current_style.clone());

                        return Some(TextNodeIterItem(
                            node.text_range(start_index),
                            current_style,
                        ));
                    }
                }
            } else {
                self.iter_stack.pop();
                let final_index = self.start_index_stack.pop().unwrap();
                if let Some(index) = self.start_index_stack.last_mut() {
                    *index = final_index;
                }
                self.style_stack.pop();
            }
        }

        None
    }
}

#[derive(Deserialize, Debug)]
pub struct ImageNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct StyledNode {
    #[serde(default = "styles_list")]
    pub styles: Vec<String>,
    pub children: Vec<DomNode>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DomNode {
    Styled(StyledNode),
    Text(TextNode),
    Image(ImageNode),
}

pub fn styles_list() -> Vec<String> {
    Vec::new()
}

impl DomNode {
    pub fn styles(&self) -> &Vec<String> {
        match self {
            DomNode::Styled(node) => &node.styles,
            DomNode::Text(node) => &node.styles,
            DomNode::Image(node) => &node.styles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_range() {
        let node = TextNode {
            styles: vec![],
            children: vec![],
        };

        assert_eq!(node.text_range(0), 0..0);
        assert_eq!(node.text_range(20), 20..20);

        let node = TextNode {
            styles: vec![],
            children: vec![TextChild::Content(String::from("abcde"))],
        };

        assert_eq!(node.text_range(0), 0..5);
        assert_eq!(node.text_range(20), 20..25);

        let node = TextNode {
            styles: vec![],
            children: vec![
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
            ],
        };

        assert_eq!(node.text_range(0), 0..30);
        assert_eq!(node.text_range(20), 20..50);

        let node = TextNode {
            styles: vec![],
            children: vec![
                TextChild::Content(String::from("abcde")),
                TextChild::TextNode(TextNode {
                    styles: vec![],
                    children: vec![
                        TextChild::Content(String::from("abcde")),
                        TextChild::Content(String::from("abcde")),
                    ],
                }),
                TextChild::TextNode(TextNode {
                    styles: vec![],
                    children: vec![
                        TextChild::TextNode(TextNode {
                            styles: vec![],
                            children: vec![TextChild::Content(String::from("abcde"))],
                        }),
                        TextChild::Content(String::from("abcde")),
                    ],
                }),
                TextChild::Content(String::from("abcde")),
            ],
        };

        assert_eq!(node.text_range(0), 0..30);
        assert_eq!(node.text_range(20), 20..50);
    }

    #[test]
    fn style_iterator() {
        let root_style = Style {
            width: "15px".to_owned(),
            ..Style::default()
        };
        let style = root_style.clone();

        let styles = HashMap::from([
            (
                "s1".to_owned(),
                MergeableStyle {
                    width: Some("15px".to_owned()),
                    ..MergeableStyle::default()
                },
            ),
            (
                "s2".to_owned(),
                MergeableStyle {
                    height: Some("30px".to_owned()),
                    ..MergeableStyle::default()
                },
            ),
            (
                "s3".to_owned(),
                MergeableStyle {
                    width: Some("45px".to_owned()),
                    ..MergeableStyle::default()
                },
            ),
            (
                "s4".to_owned(),
                MergeableStyle {
                    height: Some("50px".to_owned()),
                    ..MergeableStyle::default()
                },
            ),
        ]);

        let node = TextNode {
            styles: vec![],
            children: vec![],
        };

        assert_eq!(
            &node.iter_rich_text(&style, &styles).collect::<Vec<_>>(),
            &[TextNodeIterItem(0..0, MergeableStyle::default())]
        );

        let node = TextNode {
            styles: vec![],
            children: vec![TextChild::Content(String::from("abcde"))],
        };

        // TODO: Add styles to tests from here down
        assert_eq!(
            &node.iter_rich_text(&style, &styles).collect::<Vec<_>>(),
            &[
                TextNodeIterItem(0..5, MergeableStyle::default()),
                TextNodeIterItem(0..5, MergeableStyle::default())
            ]
        );

        let node = TextNode {
            styles: vec![],
            children: vec![
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
                TextChild::Content(String::from("abcde")),
            ],
        };

        assert_eq!(
            &node.iter_rich_text(&style, &styles).collect::<Vec<_>>(),
            &[
                TextNodeIterItem(0..30, MergeableStyle::default()),
                TextNodeIterItem(0..5, MergeableStyle::default()),
                TextNodeIterItem(5..10, MergeableStyle::default()),
                TextNodeIterItem(10..15, MergeableStyle::default()),
                TextNodeIterItem(15..20, MergeableStyle::default()),
                TextNodeIterItem(20..25, MergeableStyle::default()),
                TextNodeIterItem(25..30, MergeableStyle::default()),
            ]
        );

        let node = TextNode {
            styles: vec!["s1".to_owned()],
            children: vec![
                TextChild::Content(String::from("abcde")),
                TextChild::TextNode(TextNode {
                    styles: vec!["s2".to_owned()],
                    children: vec![
                        TextChild::Content(String::from("abcde")),
                        TextChild::Content(String::from("abcde")),
                    ],
                }),
                TextChild::TextNode(TextNode {
                    styles: vec!["s3".to_owned()],
                    children: vec![
                        TextChild::TextNode(TextNode {
                            styles: vec!["s4".to_owned()],
                            children: vec![TextChild::Content(String::from("abcde"))],
                        }),
                        TextChild::Content(String::from("abcde")),
                    ],
                }),
                TextChild::Content(String::from("abcde")),
            ],
        };

        let items = node.iter_rich_text(&style, &styles).collect::<Vec<_>>();

        assert_eq!(items[0], TextNodeIterItem(0..30, MergeableStyle::default()));

        assert_eq!(items[1], TextNodeIterItem(0..5, MergeableStyle::default()));

        assert_eq!(
            items[2],
            TextNodeIterItem(
                5..15,
                MergeableStyle {
                    height: Some("30px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[3],
            TextNodeIterItem(
                5..10,
                MergeableStyle {
                    height: Some("30px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[4],
            TextNodeIterItem(
                10..15,
                MergeableStyle {
                    height: Some("30px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[5],
            TextNodeIterItem(
                15..25,
                MergeableStyle {
                    width: Some("45px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[6],
            TextNodeIterItem(
                15..20,
                MergeableStyle {
                    width: Some("45px".to_owned()),
                    height: Some("50px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[7],
            TextNodeIterItem(
                15..20,
                MergeableStyle {
                    width: Some("45px".to_owned()),
                    height: Some("50px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[8],
            TextNodeIterItem(
                20..25,
                MergeableStyle {
                    width: Some("45px".to_owned()),
                    ..MergeableStyle::default()
                }
            )
        );

        assert_eq!(
            items[9],
            TextNodeIterItem(25..30, MergeableStyle::default())
        );
    }
}
