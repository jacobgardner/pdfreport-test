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
    dom::{nodes::ImageNode, DomNode, MergeableStyle, PdfDom, Style},
    error::BadPdfLayout,
};

mod flex_style;

pub type TextComputeFn = Box<dyn Fn(Node, Rc<RefCell<BlockLayout>>) -> MeasureFunc>;
pub type ImageComputeFn = Box<dyn Fn(Node) -> MeasureFunc>;

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

pub struct BlockLayout {
    pdf_dom: Rc<PdfDom>,
    stretch: Stretch,
    text_node_compute: TextComputeFn,
    image_node_compute: ImageComputeFn,
    layout_node_map: HashMap<Node, DomNode>,
    layout_style_map: HashMap<Node, Style>,
    node_draw_order: BTreeSet<DrawOrder>,
}

impl BlockLayout {
    pub fn build_layout<T: Into<Pt>>(
        pdf_dom: Rc<PdfDom>,
        text_compute: TextComputeFn,
        image_compute: ImageComputeFn,
        page_dimensions: Size<T>,
    ) -> Result<Rc<RefCell<Self>>, BadPdfLayout> {
        let layout = Rc::new(RefCell::new(Self {
            pdf_dom: pdf_dom.clone(),
            stretch: Stretch::new(),
            text_node_compute: text_compute,
            image_node_compute: image_compute,
            layout_node_map: HashMap::new(),
            layout_style_map: HashMap::new(),
            node_draw_order: BTreeSet::new(),
        }));

        let current_style = Style::default();

        let page_width = page_dimensions.width.into().0 as f32;

        let page_size = Size {
            width: Number::Defined(page_width), // 8.5 inches
            height: Number::Undefined,
        };

        let root_layout_node = (*layout.borrow_mut())
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

        BlockLayout::build_layout_nodes(
            layout.clone(),
            0,
            current_style,
            root_layout_node,
            root_node,
        )?;

        println!("Compute Layout...");
        (*layout.borrow_mut())
            .stretch
            .compute_layout(root_layout_node, page_size)?;
        println!("Done Computing Layout!");

        for &DrawOrder {
            node,
            depth,
            z_order,
        } in &layout.borrow().node_draw_order
        {
            let layout = (*layout).borrow();
            let node_layout = layout.stretch.layout(node)?;
            println!("Z:{z_order} -> D:{depth} -> {node:?} {node_layout:?}");
        }

        Ok(layout)
    }

    fn styles(&self) -> &HashMap<String, MergeableStyle> {
        &self.pdf_dom.styles
    }

    // There may be a way to ensure that the node passed in came from
    //  this structure to make the expect even safer
    pub fn get_style(&self, node: Node) -> &Style {
        self.layout_style_map
            .get(&node)
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
        layout: Rc<RefCell<BlockLayout>>,
        depth: usize,
        mut current_style: Style,
        parent_layout_node: stretch::node::Node,
        current_pdf_node: &DomNode,
    ) -> Result<(), BadPdfLayout> {
        for style_name in current_pdf_node.styles() {
            let layout = (*layout).borrow();
            let mergeable_style =
                layout
                    .styles()
                    .get(style_name)
                    .ok_or_else(|| BadPdfLayout::UnmatchedStyle {
                        style_name: style_name.clone(),
                    })?;

            current_style = current_style.merge_style(mergeable_style);
        }

        let child_node = match current_pdf_node {
            DomNode::Styled(_styled_node) => (*layout.borrow_mut())
                .stretch
                .new_node(current_style.clone().try_into()?, &[])?,
            DomNode::Text(text_node) => {
                // We would want to pass in a function called something like:
                //  compute_text_size which takes in the dom node, current style,
                //  etc. and returns the desired closure, if we can
                let mut layout_ref = (*layout).borrow_mut();
                let new_child = layout_ref.stretch.new_node(current_style.clone().try_into()?, &[])?;

                let f = (layout_ref.text_node_compute)(new_child, layout.clone());

                layout_ref.stretch.set_measure(new_child, Some(f))?;

                new_child
                // self.stretch.new_leaf(
                //     current_style.clone().try_into()?,
                //     (self.text_node_compute)(text_node, current_style.clone()),
                // )?
            }
            DomNode::Image(image_node) => (*layout.borrow_mut()).stretch.new_leaf(
                current_style.clone().try_into()?,
                MeasureFunc::Raw(|sz| Size { width: 0., height: 0. }),
            )?,
        };

        assert!(
            (*layout.borrow_mut())
                .layout_style_map
                .insert(child_node, current_style.clone())
                .is_none(),
            "Layout engine should guarantee all nodes are unique"
        );

        assert!(
            (*layout.borrow_mut())
                .layout_node_map
                .insert(child_node, current_pdf_node.clone())
                .is_none(),
            "Layout engine should guarantee all nodes are unique"
        );

        (*layout.borrow_mut()).node_draw_order.insert(DrawOrder {
            depth,
            // If we add z-index to style, we should be able to update it here
            // and it just work
            z_order: 0,
            node: child_node,
        });

        if let DomNode::Styled(styled_node) = current_pdf_node {
            for child in &styled_node.children {
                BlockLayout::build_layout_nodes(
                    layout.clone(),
                    depth + 1,
                    current_style.clone(),
                    child_node,
                    child,
                )?
            }
        }
        (*layout.borrow_mut())
            .stretch
            .add_child(parent_layout_node, child_node)?;

        Ok(())
    }
}
