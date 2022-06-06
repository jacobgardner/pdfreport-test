use std::ops::Range;

use printpdf::{calculate_points_for_circle, Point, Line};

use crate::{
    stylesheet::BorderRadiusStyle,
    values::{Color, Pt, Rect},
};

use super::PrintPdfWriter;

const TOP_LEFT_CORNER: Range<usize> = 12..16;
const TOP_RIGHT_CORNER: Range<usize> = 0..4;
const BOTTOM_RIGHT_CORNER: Range<usize> = 4..8;
const BOTTOM_LEFT_CORNER: Range<usize> = 8..12;

impl<'a> PrintPdfWriter<'a> {
    pub(super) fn draw_rect(
        &mut self,
        page_index: usize,
        rect: Rect<Pt>,
        border_width: Pt,
        border_color: Option<Color>,
        background_color: Option<Color>,
        border_radius: Option<BorderRadiusStyle::Unmergeable>,
    ) {
        let layer = self.get_base_layer(page_index);

        let start = Point {
            x: rect.left.into(),
            y: rect.top.into(),
        };

        let end = Point {
            x: (rect.left + rect.width).into(),
            y: (rect.top - rect.height).into(),
        };

        #[rustfmt::skip]
        let points = match border_radius {
            Some(border_radius) if border_radius != BorderRadiusStyle::Unmergeable::default() => {
                // 4 points per corner & 2 points per edge
                let mut points: Vec<(printpdf::Point, bool)> = Vec::with_capacity(4 * 4 + 4 * 2);

                // TODO: Skip any corners where the radius is 0
                // TODO: Optimization: Don't recalculate corners that have matching radius
                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.top_left), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[TOP_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + printpdf::Pt(border_radius.top_left) + start.x, y: pt.y - printpdf::Pt(border_radius.top_left) + start.y}, b)));
                points.push((Point { x: start.x + printpdf::Pt(border_radius.top_left), y: start.y, }, false));
                points.push((Point { x: end.x - printpdf::Pt(border_radius.top_left), y: start.y, }, false));

                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.top_right), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[TOP_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - printpdf::Pt(border_radius.top_right) + end.x, y: pt.y - printpdf::Pt(border_radius.top_right) + start.y}, b)));
                points.push((Point { x: end.x, y: start.y - printpdf::Pt(border_radius.top_right), }, false));
                points.push((Point { x: end.x, y: end.y + printpdf::Pt(border_radius.top_right), }, false));

                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.bottom_right), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[BOTTOM_RIGHT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x - printpdf::Pt(border_radius.bottom_right) + end.x, y: pt.y + printpdf::Pt(border_radius.bottom_right) + end.y}, b)));
                points.push((Point { x: end.x - printpdf::Pt(border_radius.bottom_right), y: end.y, }, false));
                points.push((Point { x: start.x + printpdf::Pt(border_radius.bottom_right), y: end.y , }, false));

                let circle_points = calculate_points_for_circle(printpdf::Pt(border_radius.bottom_left), printpdf::Pt(0.), printpdf::Pt(0.));
                points.extend(circle_points[BOTTOM_LEFT_CORNER].iter().map(|&(pt, b)| (Point { x: pt.x + printpdf::Pt(border_radius.bottom_left) + start.x, y: pt.y + printpdf::Pt(border_radius.bottom_left) + end.y}, b)));

                points
            },
            _ => {
                vec![
                    (Point { x: start.x, y: start.y, }, false),
                    (Point { x: end.x,   y: start.y, }, false),
                    (Point { x: end.x,   y: end.y    }, false),
                    (Point { x: start.x, y: end.y,   }, false),
                ]
            }
        };

        layer.save_graphics_state();
        let line = Line {
            points,
            is_closed: true,
            has_fill: background_color.is_some(),
            has_stroke: border_color.is_some(),
            is_clipping_path: false,
        };

        if let Some(color) = border_color {
            layer.set_outline_color(color.into());
        }

        if let Some(color) = background_color {
            layer.set_fill_color(color.into());
        }

        layer.set_outline_thickness(border_width.0);

        layer.add_shape(line);

        layer.restore_graphics_state();
    }
}
