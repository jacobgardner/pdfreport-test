use printpdf::{Line, Point, Rgb};

use crate::{
    block_layout::paginated_layout::PaginatedNode,
    stylesheet::{EdgeStyle, Style},
    utils::debug_cursor::DebugCursor,
    values::{Color, Mm, Pt, Rect},
};

use super::PrintPdfWriter;

impl<'a> PrintPdfWriter<'a> {
    pub(super) fn draw_debug_outlines(&mut self, node: &PaginatedNode, style: &Style) {
        let PaginatedNode {
            page_layout: layout,
            page_index,
            ..
        } = node;

        let page_index = *page_index;

        let coords = self.get_placement_coords(layout);

        let margin_rect = Rect {
            left: coords.0 - style.margin.left,
            top: coords.1 - style.margin.top,
            width: layout.width + style.margin.horizontal(),
            height: layout.height + style.margin.vertical(),
        };

        let border_rect = Rect {
            left: coords.0,
            top: coords.1,
            width: layout.width,
            height: layout.height,
        };

        let content_rect = Rect {
            left: border_rect.left + style.padding.left + style.border.width.left,
            top: border_rect.top + (style.padding.bottom + style.border.width.bottom),
            width: border_rect.width - style.padding.horizontal() - style.border.width.horizontal(),
            height: border_rect.height - style.padding.vertical() - style.border.width.vertical(),
        };

        self.draw_rect(
            page_index,
            margin_rect,
            EdgeStyle::new(Pt(0.1)),
            Some(Color::try_from("green").expect("We know green is a valid color.")),
            None,
            None,
        );

        self.draw_rect(
            page_index,
            border_rect,
            EdgeStyle::new(Pt(0.4)),
            Some(Color::try_from("red").expect("We know red is a valid color")),
            None,
            None,
        );

        self.draw_rect(
            page_index,
            content_rect,
            EdgeStyle::new(Pt(0.8)),
            Some(Color::try_from("blue").expect("We know blue is a valid color")),
            None,
            None,
        );
    }

    pub fn draw_debug_cursors(&mut self, debug_cursors: &[DebugCursor]) {
        let font = self
            .font_collection
            .default_font()
            .expect("If you are debugging, you must provide at least one font.");

        let font = self.get_font(font.font_id()).unwrap();

        for (idx, cursor) in debug_cursors.iter().enumerate() {
            let layer = self.get_base_layer(cursor.page_index);
            layer.set_outline_color(Color::black().into());
            layer.set_fill_color(printpdf::Color::Rgb(Rgb {
                r: 0.2,
                g: 0.,
                b: 0.,
                icc_profile: None,
            }));

            let x_position = Pt((idx % 6) as f64 * 90. + 10.);

            let cursor_points = vec![
                (
                    Point::new(
                        Mm::from(x_position).into(),
                        Mm::from(self.page_size.height - cursor.position.y).into(),
                    ),
                    false,
                ),
                (
                    Point::new(
                        Mm::from(x_position + Pt(20.)).into(),
                        Mm::from(self.page_size.height - cursor.position.y).into(),
                    ),
                    false,
                ),
            ];

            let line = Line {
                points: cursor_points,
                is_closed: false,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            };

            layer.set_outline_thickness(0.5);
            layer.add_shape(line);

            layer.begin_text_section();
            layer.set_text_cursor(
                Mm::from(x_position + Pt(20.)).into(),
                Mm::from(self.page_size.height - cursor.position.y - Pt(15.)).into(),
            );

            layer.set_font(&font, 12.);
            layer.write_text(
                &format!("{}:{} - {}", idx, cursor.position.y, cursor.label),
                &font,
            );

            layer.end_text_section();
        }
    }
}
