use crate::shape_editor::index::GridLineType;
use crate::shape_editor::{style, ShapeEditorMemory};
use egui::emath::{Pos2, Rect, Vec2};
use egui::epaint::{Color32, Shape, TextShape};
use egui::Painter;
use std::ops::Add;

pub(crate) fn paint_rulers(
    style: &dyn style::Style,
    ui_painter: &Painter,
    rulers_rect: Rect,
    memory: &ShapeEditorMemory,
) {
    let Some(grid_index) = &memory.grid else {
        return;
    };

    let transform = &memory.transform;

    let ruler_rect = Rect::from_min_max(
        Pos2::new(rulers_rect.min.x + style.rulers_width(), rulers_rect.min.y),
        Pos2::new(rulers_rect.max.x, rulers_rect.min.y + style.rulers_width()),
    );
    let painter = ui_painter.with_clip_rect(ruler_rect);
    let mut vec = vec![Shape::rect_filled(ruler_rect, 0.0, Color32::WHITE)];
    for (x, line_types) in &grid_index.horizontal.0 {
        let ui_x = transform.transform_pos(Pos2::new(x.into_inner(), 0.0)).x + ruler_rect.left();
        for line_type in line_types {
            match line_type {
                GridLineType::Zero | GridLineType::Primary => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ui_x, ruler_rect.max.y - style.rulers_div_height()),
                            Pos2::new(ui_x, ruler_rect.max.y),
                        ],
                        style.rulers_stroke(),
                    ));
                    let label = TextShape::new(
                        Pos2::new(ui_x, ruler_rect.min.y)
                            .add(style.rulers_text_position().to_vec2()),
                        painter.fonts(|fonts| {
                            fonts.layout_no_wrap(
                                format!("{}", x),
                                style.rulers_font().clone(),
                                style.rulers_font_color(),
                            )
                        }),
                        style.rulers_font_color(),
                    );
                    vec.push(Shape::Text(label));
                }
                GridLineType::Secondary => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ui_x, ruler_rect.max.y - style.rulers_half_div_height()),
                            Pos2::new(ui_x, ruler_rect.max.y),
                        ],
                        style.rulers_half_stroke(),
                    ));
                }
                GridLineType::Sub => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ui_x, ruler_rect.max.y - style.rulers_sub_div_height()),
                            Pos2::new(ui_x, ruler_rect.max.y),
                        ],
                        style.rulers_sub_stroke(),
                    ));
                }
            }
        }
    }
    painter.extend(vec);

    let ruler_rect = Rect::from_min_max(
        Pos2::new(rulers_rect.min.x, rulers_rect.min.y + style.rulers_width()),
        Pos2::new(rulers_rect.min.x + style.rulers_width(), rulers_rect.max.y),
    );
    let painter = ui_painter.with_clip_rect(ruler_rect);
    let mut vec = vec![Shape::rect_filled(ruler_rect, 0.0, Color32::WHITE)];
    for (y, line_types) in &grid_index.vertical.0 {
        let ui_y = transform.transform_pos(Pos2::new(0.0, y.into_inner())).y + ruler_rect.top();
        for line_type in line_types {
            match line_type {
                GridLineType::Zero | GridLineType::Primary => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ruler_rect.max.x - style.rulers_div_height(), ui_y),
                            Pos2::new(ruler_rect.max.x, ui_y),
                        ],
                        style.rulers_stroke(),
                    ));
                    let label = TextShape::new(
                        Pos2::new(ruler_rect.min.x, ui_y).add(Vec2::new(
                            style.rulers_text_position().y,
                            style.rulers_text_position().x,
                        )),
                        painter.fonts(|fonts| {
                            fonts.layout_no_wrap(
                                format!("{}", y),
                                style.rulers_font().clone(),
                                style.rulers_font_color(),
                            )
                        }),
                        style.rulers_font_color(),
                    )
                    .with_angle(-std::f32::consts::FRAC_PI_2);
                    vec.push(Shape::Text(label));
                }
                GridLineType::Secondary => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ruler_rect.max.x - style.rulers_half_div_height(), ui_y),
                            Pos2::new(ruler_rect.max.x, ui_y),
                        ],
                        style.rulers_half_stroke(),
                    ));
                }
                GridLineType::Sub => {
                    vec.push(Shape::line_segment(
                        [
                            Pos2::new(ruler_rect.max.x - style.rulers_sub_div_height(), ui_y),
                            Pos2::new(ruler_rect.max.x, ui_y),
                        ],
                        style.rulers_sub_stroke(),
                    ));
                }
            }
        }
    }
    painter.extend(vec);
}
