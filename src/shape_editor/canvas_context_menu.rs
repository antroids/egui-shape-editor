use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::memory::ShapeEditorMemory;
use crate::shape_editor::{interaction, utils, ShapeEditor};
use egui::epaint::{CubicBezierShape, PathShape, QuadraticBezierShape, Vertex};
use egui::{Color32, Mesh, Pos2, Rect, Response, Shape};

impl<'a> ShapeEditor<'a> {
    pub(crate) fn canvas_context_menu(
        &mut self,
        response: Response,
        memory: &mut ShapeEditorMemory,
        ctx: &CanvasContext,
    ) {
        puffin_egui::puffin::profile_function!();
        response.context_menu(|ui| {
            ui.menu_button("Add shape", |ui| {
                let point = ctx.input.canvas_content_mouse_pos;

                if ui.button("Quadratic Bezier").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        3,
                        |points, options| {
                            if let &[p0, p1, p2, ..] = points.as_slice() {
                                Some(Shape::QuadraticBezier(
                                    QuadraticBezierShape::from_points_stroke(
                                        [p0, p1, p2],
                                        false,
                                        Color32::TRANSPARENT,
                                        options.stroke,
                                    ),
                                ))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Cubic Bezier").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        4,
                        |points, options| {
                            if let &[p0, p1, p2, p3, ..] = points.as_slice() {
                                Some(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                                    [p0, p1, p2, p3],
                                    false,
                                    Color32::TRANSPARENT,
                                    options.stroke,
                                )))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Path").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        2,
                        |points, options| {
                            if let &[p0, p1, ..] = points.as_slice() {
                                Some(Shape::Path(PathShape::line(vec![p0, p1], options.stroke)))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Line Segment").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        2,
                        |points, options| {
                            if let &[p0, p1, ..] = points.as_slice() {
                                Some(Shape::line_segment([p0, p1], options.stroke))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Circle").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        2,
                        |points, options| {
                            if let &[p0, p1, ..] = points.as_slice() {
                                Some(Shape::circle_stroke(p0, p0.distance(p1), options.stroke))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Rect").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        2,
                        |points, options| {
                            if let &[p0, p1, ..] = points.as_slice() {
                                let rect = utils::normalize_rect(&Rect::from_two_pos(p0, p1));
                                Some(Shape::rect_stroke(rect, 0.0, options.stroke))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }

                if ui.button("Mesh").clicked() {
                    memory.begin_interaction(interaction::AddPointsThanShape::with_start_point(
                        point,
                        3,
                        |points, options| {
                            if let &[p0, p1, p2, ..] = points.as_slice() {
                                Some(Shape::mesh(Mesh {
                                    indices: vec![0, 1, 2],
                                    vertices: vec![
                                        Vertex {
                                            pos: p0,
                                            uv: Pos2::ZERO,
                                            color: options.stroke.color,
                                        },
                                        Vertex {
                                            pos: p1,
                                            uv: Pos2::ZERO,
                                            color: options.stroke.color,
                                        },
                                        Vertex {
                                            pos: p2,
                                            uv: Pos2::ZERO,
                                            color: options.stroke.color,
                                        },
                                    ],
                                    texture_id: Default::default(),
                                }))
                            } else {
                                None
                            }
                        },
                    ));
                    ui.close_menu();
                }
            });

            if let Some(last_action_name) = memory
                .action_history()
                .last()
                .map(|(_, short_name)| short_name)
            {
                if ui.button(format!("Undo '{}'", last_action_name)).clicked() {
                    memory.undo(self.shape);
                    ui.close_menu();
                }
            }
        });
    }
}
