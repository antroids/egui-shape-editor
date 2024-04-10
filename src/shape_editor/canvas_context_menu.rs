use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::memory::ShapeEditorMemory;
use crate::shape_editor::{interaction, ShapeEditor};
use egui::Response;

impl<'a> ShapeEditor<'a> {
    pub(crate) fn canvas_context_menu(
        &mut self,
        response: Response,
        memory: &mut ShapeEditorMemory,
        ctx: &CanvasContext,
    ) {
        puffin_egui::puffin::profile_function!();
        response.context_menu(|ui| {
            if !self.options.context_menu_add_shapes.is_empty() {
                ui.menu_button("Add shape", |ui| {
                    let point = ctx.input.canvas_content_mouse_pos;
                    for shape_type in &self.options.context_menu_add_shapes {
                        if ui.button(shape_type.to_string()).clicked() {
                            memory.begin_interaction(
                                interaction::AddPointsThanShape::with_shape_type_and_start_point(
                                    *shape_type,
                                    point,
                                ),
                            );
                            ui.close_menu();
                        }
                    }
                });
            }

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
