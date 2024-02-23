use crate::shape_editor::action::InsertShape;
use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::visitor::LastShapePointIndex;
use crate::shape_editor::{MouseDrag, ShapeEditor, ShapeEditorMemory};
use egui::epaint::CircleShape;
use egui::{Response, Shape};

impl<'a> ShapeEditor<'a> {
    pub(crate) fn canvas_context_menu(
        &mut self,
        response: Response,
        memory: &mut ShapeEditorMemory,
        ctx: &CanvasContext,
    ) {
        response.context_menu(|ui| {
            ui.menu_button("Add shape", |ui| {
                let point = ctx.input.canvas_content_mouse_pos;
                let stroke = self.options.stroke;

                if ui.button("Quadratic Bezier").clicked() {
                    self.apply_action(
                        InsertShape::quadratic_bezier_from_two_points(point, None, point, stroke),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }

                if ui.button("Cubic Bezier").clicked() {
                    self.apply_action(
                        InsertShape::cubic_bezier_from_two_points(point, None, point, stroke),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }

                if ui.button("Path").clicked() {
                    self.apply_action(
                        InsertShape::path_from_two_points(point, point, stroke),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }

                if ui.button("Line Segment").clicked() {
                    self.apply_action(
                        InsertShape::line_segment_from_two_points(point, point, stroke),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }

                if ui.button("Circle").clicked() {
                    self.apply_action(
                        InsertShape::from_shape(Shape::Circle(CircleShape::stroke(
                            point, 0.0, stroke,
                        ))),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }

                if ui.button("Rect").clicked() {
                    self.apply_action(
                        InsertShape::rect_from_two_points(point, point, stroke),
                        memory,
                    );
                    self.place_last_shape_point(memory, ctx);
                    ui.close_menu();
                }
            });

            if let Some(last_action_name) = memory
                .action_history
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

    fn place_last_shape_point(&mut self, memory: &mut ShapeEditorMemory, ctx: &CanvasContext) {
        if let Some(index) = LastShapePointIndex::last_index(self.shape) {
            let point = ctx.input.canvas_content_mouse_pos;
            memory.selection.select_single_control_point(index);
            memory.mouse_drag = Some(MouseDrag::MoveShapeControlPoints(point, point));
        }
    }
}
