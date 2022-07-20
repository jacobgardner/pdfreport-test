use std::{cell::RefCell, collections::HashMap, ops::Range, rc::Rc};

use printpdf::calculate_points_for_circle;

use crate::values::Pt;

const TOP_LEFT_CORNER: Range<usize> = 12..16;
const TOP_RIGHT_CORNER: Range<usize> = 0..4;
const BOTTOM_RIGHT_CORNER: Range<usize> = 4..8;
const BOTTOM_LEFT_CORNER: Range<usize> = 8..12;

const ORIGIN: printpdf::Point = printpdf::Point {
    x: printpdf::Pt(0.),
    y: printpdf::Pt(0.),
};

type LinePoint = (printpdf::Point, bool);

const ZERO_RADIUS_CORNER: &[LinePoint; 1] = &[(ORIGIN, false)];

#[derive(Default)]
pub(super) struct Circles(RefCell<HashMap<i32, Rc<Circle>>>);

pub(super) struct Circle(Vec<LinePoint>, bool);

impl Circle {
    pub fn top_left(&self) -> &[LinePoint] {
        if self.1 {
            &self.0[TOP_LEFT_CORNER]
        } else {
            ZERO_RADIUS_CORNER
        }
    }

    pub fn top_right(&self) -> &[LinePoint] {
        if self.1 {
            &self.0[TOP_RIGHT_CORNER]
        } else {
            ZERO_RADIUS_CORNER
        }
    }

    pub fn bottom_left(&self) -> &[LinePoint] {
        if self.1 {
            &self.0[BOTTOM_LEFT_CORNER]
        } else {
            ZERO_RADIUS_CORNER
        }
    }

    pub fn bottom_right(&self) -> &[LinePoint] {
        if self.1 {
            &self.0[BOTTOM_RIGHT_CORNER]
        } else {
            ZERO_RADIUS_CORNER
        }
    }
}

impl Circles {
    pub fn get(&self, radius: Pt) -> Rc<Circle> {
        let approx_radius: i32 = (radius.0 * 100.) as i32;

        let mut cache = self.0.borrow_mut();

        if let Some(circle) = cache.get(&approx_radius) {
            circle.clone()
        } else {
            let circle = Rc::new(Circle(
                calculate_points_for_circle(radius, Pt(0.), Pt(0.)),
                approx_radius != 0,
            ));

            cache.insert(approx_radius, circle.clone());

            circle
        }
    }
}
