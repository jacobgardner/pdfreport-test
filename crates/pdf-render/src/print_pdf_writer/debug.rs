use printpdf::{Line, Point, Rgb};

use crate::{
    block_layout::paginated_layout::PaginatedNode,
    fonts::FontAttributes,
    stylesheet::{BorderRadiusStyle, EdgeStyle, Style},
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

        let mut margin_rect = Rect {
            left: layout.left - style.margin.left + self.page_margins.left,
            top: layout.top - style.margin.top + self.page_margins.top,
            width: layout.width + style.margin.horizontal(),
            height: layout.height + style.margin.vertical(),
        };

        let mut border_rect = Rect {
            left: layout.left + self.page_margins.left,
            top: layout.top + self.page_margins.top,
            width: layout.width,
            height: layout.height,
        };

        let mut content_rect = Rect {
            left: border_rect.left + style.padding.left,
            top: border_rect.top + style.padding.top,
            width: border_rect.width - style.padding.horizontal(),
            height: border_rect.height - style.padding.vertical(),
        };

        margin_rect.top = self.page_size.height - margin_rect.top;
        border_rect.top = self.page_size.height - border_rect.top;
        content_rect.top = self.page_size.height - content_rect.top;

        self.draw_rect(
            page_index,
            margin_rect,
            EdgeStyle::new(Pt(1.)),
            Some(Color::try_from("green").unwrap()),
            None,
            Some(BorderRadiusStyle::new(Pt(10.))),
        );

        self.draw_rect(
            page_index,
            border_rect,
            EdgeStyle::new(Pt(1.)),
            Some(Color::try_from("red").unwrap()),
            None,
            Some(BorderRadiusStyle::new(Pt(7.5))),
        );

        self.draw_rect(
            page_index,
            content_rect,
            EdgeStyle::new(Pt(1.)),
            Some(Color::try_from("blue").unwrap()),
            None,
            Some(BorderRadiusStyle::new(Pt(5.))),
        );
    }

    pub fn draw_debug_cursors(&mut self, debug_cursors: &[DebugCursor]) {
        let font = self
            .font_collection
            .lookup_font("Inter", &FontAttributes::bold())
            .unwrap();

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
