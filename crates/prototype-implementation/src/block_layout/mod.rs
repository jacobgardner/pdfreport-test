use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

use printpdf::Pt;
use stretch::node::MeasureFunc;
use stretch2 as stretch;
use stretch2::prelude::*;

use crate::{
    dom::{
        nodes::{ImageNode, TextNode},
        DomNode, MergeableStyle, PdfDom, Style,
    },
    error::BadPdfLayout,
};

mod flex_style;

pub type TextComputeFn<'a> = Box<dyn Fn(Node, &'a TextNode, Rc<Style>) -> MeasureFunc + 'a>;
pub type ImageComputeFn<'a> = Box<dyn Fn(&'a ImageNode) -> MeasureFunc>;

#[derive(Clone, Debug)]
struct DrawOrder {
    depth: usize,
    z_order: usize,
    node: Node,
}

impl PartialEq for DrawOrder {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth && self.z_order == other.z_order && self.node == other.node
    }
}

impl Eq for DrawOrder {}

impl PartialOrd for DrawOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // println!("Partial??");
        //
        //
        Some(self.cmp(other))

        // match self.z_order.partial_cmp(&other.z_order) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }

        // self.depth.partial_cmp(&other.depth)
    }
}

impl Ord for DrawOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z_order.cmp(&other.z_order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        match self.depth.cmp(&other.depth) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        if self.node == other.node {
            core::cmp::Ordering::Equal
        } else {
            // This is completely arbitrary and doesn't matter as long as its not equal
            core::cmp::Ordering::Less
        }
    }
}

pub struct BlockLayout<'a> {
    pdf_dom: &'a PdfDom,
    stretch: Stretch,
    text_node_compute: TextComputeFn<'a>,
    image_node_compute: ImageComputeFn<'a>,
    layout_node_map: HashMap<Node, &'a DomNode>,
    layout_style_map: Rc<RefCell<HashMap<Node, Rc<Style>>>>,
    node_draw_order: BTreeSet<DrawOrder>,
    styles: Rc<HashMap<String, MergeableStyle>>,
}

impl<'a> BlockLayout<'a> {
    pub fn build_layout<T: Into<Pt>>(
        pdf_dom: &'a PdfDom,
        text_compute: TextComputeFn<'a>,
        image_compute: ImageComputeFn<'a>,
        page_dimensions: Size<T>,
    ) -> Result<Self, BadPdfLayout> {
        let mut layout = Self {
            pdf_dom,
            stretch: Stretch::new(),
            text_node_compute: text_compute,
            image_node_compute: image_compute,
            layout_node_map: HashMap::new(),
            layout_style_map: Rc::new(RefCell::new(HashMap::new())),
            node_draw_order: BTreeSet::new(),
            styles: Rc::new(pdf_dom.styles.clone()),
        };

        let current_style = Style::default();

        let page_width = page_dimensions.width.into().0 as f32;

        let page_size = Size {
            width: Number::Defined(page_width), // 8.5 inches
            height: Number::Undefined,
        };
        let root_layout_node = layout
            .stretch
            .new_node(
                stretch::style::Style {
                    size: Size {
                        width: Dimension::Points(page_width),
                        height: Dimension::Undefined,
                    },
                    ..current_style.clone().try_into()?
                },
                &[],
            )
            .expect("This should only be able to error if children are added.");

        let root_node = &pdf_dom.root;

        layout.build_layout_nodes(0, Rc::new(current_style), root_layout_node, root_node)?;

        println!("Compute Layout...");
        layout.stretch.compute_layout(root_layout_node, page_size)?;
        println!("Done Computing Layout!");

        // for &DrawOrder {
        //     node,
        //     depth,
        //     z_order,
        // } in &layout.node_draw_order
        // {
        //     let node_layout = layout.stretch.layout(node)?;
        //     println!("Z:{z_order} -> D:{depth} -> {node:?} {node_layout:?}");
        // }

        Ok(layout)
    }

    fn styles(&self) -> Rc<HashMap<String, MergeableStyle>> {
        self.styles.clone()
    }

    pub fn get_layout(&self, node: Node) -> Result<&Layout, BadPdfLayout> {
        Ok(self.stretch.layout(node)?)
    }

    // There may be a way to ensure that the node passed in came from
    //  this structure to make the expect even safer
    pub fn get_style(&self, node: Node) -> Rc<Style> {
        self.layout_style_map
            .borrow()
            .get(&node)
            .cloned()
            .expect("The provided node should have come from this layout")
    }

    pub fn get_dom_node(&self, node: Node) -> &DomNode {
        self.layout_node_map
            .get(&node)
            .expect("The provided node should have come from this layout")
    }

    pub fn draw_order(&self) -> impl Iterator<Item = Node> + '_ {
        self.node_draw_order
            .iter()
            .map(|draw_order_node| draw_order_node.node)
    }

    // pub fn layout_style_map(&self) -> &HashMap<Node, Style> {
    //     &self.layout_style_map
    // }

    fn build_layout_nodes(
        &mut self,
        depth: usize,
        mut current_style: Rc<Style>,
        parent_layout_node: stretch::node::Node,
        current_pdf_node: &'a DomNode,
    ) -> Result<(), BadPdfLayout> {
        let styles = self.styles.clone();

        for style_name in current_pdf_node.styles() {
            let mergeable_style =
                styles
                    .get(style_name)
                    .ok_or_else(|| BadPdfLayout::UnmatchedStyle {
                        style_name: style_name.clone(),
                    })?;

            current_style = Rc::new(current_style.merge_style(mergeable_style));
        }

        let child_node = match current_pdf_node {
            DomNode::Styled(_styled_node) => self
                .stretch
                .new_node((*current_style).clone().try_into()?, &[])?,
            DomNode::Text(text_node) => {
                let child = self
                    .stretch
                    .new_node((*current_style).clone().try_into()?, &[])?;

                // We would want to pass in a function called something like:
                //  compute_text_size which takes in the dom node, current style,
                //  etc. and returns the desired closure, if we can
                self.stretch.set_measure(
                    child,
                    Some((self.text_node_compute)(
                        child,
                        text_node,
                        current_style.clone(),
                    )),
                )?;

                child
            }
            DomNode::Image(image_node) => self.stretch.new_leaf(
                (*current_style).clone().try_into()?,
                (self.image_node_compute)(image_node),
            )?,
        };

        assert!(
            self.layout_style_map
                .borrow_mut()
                .insert(child_node, current_style.clone())
                .is_none(),
            "Layout engine should guarantee all nodes are unique"
        );

        assert!(
            self.layout_node_map
                .insert(child_node, current_pdf_node)
                .is_none(),
            "Layout engine should guarantee all nodes are unique"
        );

        self.node_draw_order.insert(DrawOrder {
            depth,
            // If we add z-index to style, we should be able to update it here
            // and it just work
            z_order: 0,
            node: child_node,
        });

        if let DomNode::Styled(styled_node) = current_pdf_node {
            for child in &styled_node.children {
                self.build_layout_nodes(depth + 1, current_style.clone(), child_node, child)?
            }
        }

        self.stretch.add_child(parent_layout_node, child_node)?;

        Ok(())
    }
}
