use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::index::{GridIndex, GridLineType};
use crate::shape_editor::ShapeEditorStyle;
use egui::emath::Pos2;
use egui::epaint::Shape;
use egui::Vec2;

pub fn paint_grid(ctx: &CanvasContext, style: &ShapeEditorStyle, index: &GridIndex) {
    let canvas_viewport = ctx.transform.canvas_content_viewport();
    let x_range = canvas_viewport.x_range();
    let y_range = canvas_viewport.y_range();
    let mut vec = Vec::new();
    let grid_line_secondary_length = ctx
        .transform
        .ui_to_canvas_content
        .scale_vec(Vec2::new(style.grid_line_secondary_length, 0.0))
        .x;
    let grid_line_secondary_gap = ctx
        .transform
        .ui_to_canvas_content
        .scale_vec(Vec2::new(style.grid_line_secondary_gap, 0.0))
        .x;
    for (x, line_types) in &index.horizontal.0 {
        for line_type in line_types {
            match line_type {
                GridLineType::Zero => vec.push(Shape::LineSegment {
                    points: [
                        Pos2::new(x.into_inner(), y_range.min),
                        Pos2::new(x.into_inner(), y_range.max),
                    ],
                    stroke: style.grid_line_zero_stroke,
                }),
                GridLineType::Primary => vec.push(Shape::LineSegment {
                    points: [
                        Pos2::new(x.into_inner(), y_range.min),
                        Pos2::new(x.into_inner(), y_range.max),
                    ],
                    stroke: style.grid_line_primary_stroke,
                }),
                GridLineType::Secondary => vec.extend(Shape::dashed_line(
                    &[
                        Pos2::new(x.into_inner(), y_range.min),
                        Pos2::new(x.into_inner(), y_range.max),
                    ],
                    style.grid_line_secondary_stroke,
                    grid_line_secondary_length,
                    grid_line_secondary_gap,
                )),
                _ => {}
            }
        }
    }
    for (y, line_types) in &index.vertical.0 {
        for line_type in line_types {
            match line_type {
                GridLineType::Zero => vec.push(Shape::LineSegment {
                    points: [
                        Pos2::new(x_range.min, y.into_inner()),
                        Pos2::new(x_range.max, y.into_inner()),
                    ],
                    stroke: style.grid_line_zero_stroke,
                }),
                GridLineType::Primary => vec.push(Shape::LineSegment {
                    points: [
                        Pos2::new(x_range.min, y.into_inner()),
                        Pos2::new(x_range.max, y.into_inner()),
                    ],
                    stroke: style.grid_line_primary_stroke,
                }),
                GridLineType::Secondary => vec.extend(Shape::dashed_line(
                    &[
                        Pos2::new(x_range.min, y.into_inner()),
                        Pos2::new(x_range.max, y.into_inner()),
                    ],
                    style.grid_line_secondary_stroke,
                    grid_line_secondary_length,
                    grid_line_secondary_gap,
                )),
                _ => {}
            }
        }
    }
    ctx.painter.add(
        ctx.transform
            .canvas_content_to_ui
            .transform_shape(&Shape::Vec(vec)),
    );
}
