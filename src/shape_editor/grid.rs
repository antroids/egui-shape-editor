use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::index::{GridIndex, GridLineType};
use crate::shape_editor::style;
use egui::emath::Pos2;
use egui::epaint::Shape;
use egui::Vec2;

pub fn paint_grid(ctx: &CanvasContext, style: &dyn style::Style, index: &GridIndex) {
    puffin_egui::puffin::profile_function!();
    let canvas_viewport = ctx.transform.canvas_content_viewport();
    let ui_x_range = ctx
        .transform
        .canvas_content_to_ui
        .transform_x_rangef(&canvas_viewport.x_range());
    let ui_y_range = ctx
        .transform
        .canvas_content_to_ui
        .transform_y_rangef(&canvas_viewport.y_range());
    let mut vec = Vec::new();
    let grid_line_secondary_length = ctx
        .transform
        .ui_to_canvas_content
        .scale_vec(Vec2::new(style.grid_line_secondary_length(), 0.0))
        .x;
    let grid_line_secondary_gap = ctx
        .transform
        .ui_to_canvas_content
        .scale_vec(Vec2::new(style.grid_line_secondary_gap(), 0.0))
        .x;
    for (x, line_types) in &index.horizontal.0 {
        for line_type in line_types {
            let ui_x = ctx
                .transform
                .canvas_content_to_ui
                .transform_x(x.into_inner());
            let points = [
                Pos2::new(ui_x, ui_y_range.min),
                Pos2::new(ui_x, ui_y_range.max),
            ];
            match line_type {
                GridLineType::Zero => vec.push(Shape::LineSegment {
                    points,
                    stroke: style.grid_line_zero_stroke(),
                }),
                GridLineType::Primary => vec.push(Shape::LineSegment {
                    points,
                    stroke: style.grid_line_primary_stroke(),
                }),
                GridLineType::Secondary => vec.extend(Shape::dashed_line(
                    &points,
                    style.grid_line_secondary_stroke(),
                    grid_line_secondary_length,
                    grid_line_secondary_gap,
                )),
                _ => {}
            }
        }
    }
    for (y, line_types) in &index.vertical.0 {
        for line_type in line_types {
            let ui_y = ctx
                .transform
                .canvas_content_to_ui
                .transform_y(y.into_inner());
            let points = [
                Pos2::new(ui_x_range.min, ui_y),
                Pos2::new(ui_x_range.max, ui_y),
            ];
            match line_type {
                GridLineType::Zero => vec.push(Shape::LineSegment {
                    points,
                    stroke: style.grid_line_zero_stroke(),
                }),
                GridLineType::Primary => vec.push(Shape::LineSegment {
                    points,
                    stroke: style.grid_line_primary_stroke(),
                }),
                GridLineType::Secondary => vec.extend(Shape::dashed_line(
                    &points,
                    style.grid_line_secondary_stroke(),
                    grid_line_secondary_length,
                    grid_line_secondary_gap,
                )),
                _ => {}
            }
        }
    }
    ctx.painter.add(Shape::Vec(vec));
}
