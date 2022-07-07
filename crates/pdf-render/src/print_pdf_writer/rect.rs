use printpdf::{Line, PdfLayerReference, Point};

use crate::{
    stylesheet::{BorderRadiusStyle, EdgeStyle},
    values::{Color, Pt, Rect},
};

use super::PrintPdfWriter;

impl<'a> PrintPdfWriter<'a> {
    pub(super) fn draw_rect(
        &mut self,
        page_index: usize,
        rect: Rect<Pt>,
        border_width: EdgeStyle::Unmergeable,
        border_color: Option<Color>,
        background_color: Option<Color>,
        border_radius: Option<BorderRadiusStyle::Unmergeable>,
    ) {
        let layer = self.get_base_layer(page_index);

        // bottom-left
        let start = Point {
            x: rect.left.into(),
            y: rect.top.into(),
        };

        // top-right
        let end = Point {
            x: (rect.left + rect.width).into(),
            y: (rect.top + rect.height).into(),
        };

        let mut edge_ranges = [0..1, 1..2, 2..3, 3..4];

        let corners = &[
            Point {
                x: start.x,
                y: end.y,
            },
            Point {
                x: end.x,
                y: end.y,
            },
            Point { x: end.x, y: start.y },
            Point {
                x: start.x,
                y: start.y,
            },
        ];

        // let offsets = &[(1., -1.), (-1., -1.), (-1., 1.), (1., 1.)];

        let lines = &[
            (
                Point {
                    x: start.x,
                    y: end.y,
                },
                false,
            ),
            (
                Point {
                    x: end.x,
                    y: end.y,
                },
                false,
            ),
            (Point { x: end.x, y: start.y }, false),
            (
                Point {
                    x: start.x,
                    y: start.y,
                },
                false,
            ),
        ];

        let points = match border_radius {
            Some(border_radius) if border_radius != BorderRadiusStyle::Unmergeable::default() => {
                // 4 points per corner & 2 points per edge
                let mut points: Vec<(printpdf::Point, bool)> = Vec::with_capacity(4 * 4 + 4 * 2);

                let radii = &[
                    border_radius.top_left.0,
                    border_radius.top_right.0,
                    border_radius.bottom_right.0,
                    border_radius.bottom_left.0,
                ];

                let circle_corners: Vec<_> = radii
                    .iter()
                    .map(|&r| self.circle_cache.get(Pt(r)))
                    .collect();

                let corner_points = &[
                    circle_corners[0].top_left(),
                    circle_corners[1].top_right(),
                    circle_corners[2].bottom_right(),
                    circle_corners[3].bottom_left(),
                ];

                for edge_idx in 0..4 {
                    let current_corner = corner_points[edge_idx];

                    let first_point = current_corner.first().map(|(pt, _)| pt).unwrap();
                    let last_point = current_corner.last().map(|(pt, _)| pt).unwrap();

                    let circle_offset = Point {
                        x: first_point.x + last_point.x,
                        y: first_point.y + last_point.y,
                    };

                    let start_idx = points.len();

                    points.extend(current_corner.iter().map(|&(pt, is_curve)| {
                        (
                            printpdf::Point {
                                x: pt.x + corners[edge_idx].x - circle_offset.x,
                                y: pt.y + corners[edge_idx].y - circle_offset.y,
                            },
                            is_curve,
                        )
                    }));

                    edge_ranges[edge_idx] = start_idx..points.len();
                }

                points
            }
            _ => lines.to_vec(),
        };

        layer.save_graphics_state();

        if let Some(background_color) = background_color {
            let line = Line {
                points: points.clone(),
                is_closed: true,
                has_fill: true,
                has_stroke: false,
                is_clipping_path: false,
            };

            layer.set_outline_thickness(1.0);
            // layer.set_outline_color(Color::black().into());
            layer.set_fill_color(background_color.into());
            layer.add_shape(line);
        }

        if let Some(color) = border_color {
            layer.set_outline_color(color.into());

            draw_border_edge(
                &layer,
                points[edge_ranges[0].start..edge_ranges[1].end].iter(),
                border_width.top,
                Point {
                    x: printpdf::Pt(0.),
                    y: printpdf::Pt(border_width.top.0 / -2.),
                },
            );
            draw_border_edge(
                &layer,
                points[edge_ranges[1].start..edge_ranges[2].end].iter(),
                border_width.right,
                Point {
                    x: printpdf::Pt(border_width.right.0 / -2.),
                    y: printpdf::Pt(0.),
                },
            );
            draw_border_edge(
                &layer,
                points[edge_ranges[2].start..edge_ranges[3].end].iter(),
                border_width.bottom,
                Point {
                    x: printpdf::Pt(0.),
                    y: printpdf::Pt(border_width.bottom.0 / 2.),
                },
            );
            draw_border_edge(
                &layer,
                points[edge_ranges[3].start..]
                    .iter()
                    .chain(points[..edge_ranges[0].end].iter()),
                border_width.left,
                Point {
                    x: printpdf::Pt(border_width.left.0 / 2.),
                    y: printpdf::Pt(0.),
                },
            );
        }

        layer.restore_graphics_state();
    }
}

fn draw_border_edge<'a>(
    layer: &PdfLayerReference,
    points: impl Iterator<Item = &'a (printpdf::Point, bool)>,
    width: Pt,
    offset: Point,
) {
    if width != Pt(0.) {
        layer.set_outline_thickness(width.0);
        let line = Line {
            points: points
                .cloned()
                .map(|(pt, is_curve)| {
                    (
                        Point {
                            x: pt.x + offset.x,
                            y: pt.y + offset.y,
                        },
                        is_curve,
                    )
                })
                .collect(),
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        layer.add_shape(line);
    }
}
