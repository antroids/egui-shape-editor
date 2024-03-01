use crate::shape_editor::action::ShapeAction;
use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::style::Style;
use crate::shape_editor::{action, utils, ShapeEditorMemory, ShapeEditorOptions};
use dyn_clone::DynClone;
use egui::{Pos2, Rect, Shape, Vec2};
use std::mem;
use std::ops::Mul;

impl ShapeEditorMemory {
    pub(crate) fn next_frame_interactions(&mut self, ctx: &CanvasContext) {
        puffin_egui::puffin::profile_function!();
        let mouse_pos = ctx.input.mouse_pos;
        if ctx.input.primary_drag_started() {
            if !ctx.input.action_modifier.do_not_deselect_selected_points() {
                let canvas_mouse_pos = ctx.transform.ui_to_canvas_content.transform_pos(mouse_pos);
                if let Some(closest_selected_control_point) =
                    self.closest_selected_control_point(canvas_mouse_pos)
                {
                    self.begin_interaction(MoveShapeControlPoints {
                        start_pos: closest_selected_control_point.position(),
                        end_pos: closest_selected_control_point.position(),
                    });
                    return;
                }
            }
            self.begin_interaction(Selection {
                rect: Rect::from_min_max(mouse_pos, mouse_pos),
            });
        } else if ctx.input.secondary_drag_started() {
            self.begin_interaction(Pan {
                start_pos: mouse_pos,
            });
        }
    }

    pub(crate) fn current_frame_interactions(&mut self, ctx: &CanvasContext) {
        puffin_egui::puffin::profile_function!();
        if ctx.input.mouse_zoom_delta != 1.0 {
            self.begin_interaction(Zoom);
        } else if ctx.input.mouse_scroll_delta != Vec2::ZERO {
            self.begin_interaction(Scroll);
        }
    }

    pub(crate) fn update_interaction(
        &mut self,
        shape: &mut Shape,
        style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) {
        let interactions = mem::replace(&mut self.interaction, Vec::new());
        for interaction in interactions {
            if let Some(result) = interaction.update(self, shape, style, options, ctx) {
                self.interaction.push(result)
            }
        }
    }

    pub(crate) fn begin_interaction<T: Interaction + 'static>(&mut self, interaction: T) {
        self.interaction.push(Box::new(interaction));
    }
}

pub(crate) trait Interaction: DynClone + Send + Sync {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>>;
}
dyn_clone::clone_trait_object!(Interaction);

#[derive(Clone)]
pub(crate) struct MoveShapeControlPoints {
    pub start_pos: Pos2,
    pub end_pos: Pos2,
}

#[derive(Clone)]
struct Selection {
    rect: Rect,
}

#[derive(Clone)]
struct Pan {
    start_pos: Pos2,
}

#[derive(Clone)]
pub(crate) struct AddPointsThanShape {
    points: Vec<Pos2>,
    points_count: usize,
    shape_fn: fn(&Vec<Pos2>, &ShapeEditorOptions) -> Option<Shape>,
}

#[derive(Clone)]
struct Scroll;

#[derive(Clone)]
struct Zoom;

impl Interaction for MoveShapeControlPoints {
    fn update(
        mut self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        _style: &dyn Style,
        _options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        puffin_egui::puffin::profile_function!();
        if ctx.input.drag_released || ctx.input.mouse_primary_pressed {
            if self.end_pos != self.start_pos && memory.selection.has_control_points() {
                let move_action = action::MoveShapeControlPoints::from_index_and_translation(
                    memory.selection.control_points(),
                    &(self.end_pos - self.start_pos),
                );
                memory
                    .push_action_history(Box::new(move_action.invert()), move_action.short_name());
            }
            None
        } else {
            if self.end_pos != ctx.input.canvas_content_mouse_pos {
                let snap_point = memory
                    .snap
                    .snap_point
                    .unwrap_or(ctx.input.canvas_content_mouse_pos);
                Box::new(action::MoveShapeControlPoints::from_index_and_translation(
                    memory.selection.control_points(),
                    &(snap_point - self.end_pos),
                ))
                .apply(shape);
                self.end_pos = snap_point;
            }
            Some(self)
        }
    }
}

impl Interaction for Selection {
    fn update(
        mut self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        _shape: &mut Shape,
        style: &dyn Style,
        _options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        puffin_egui::puffin::profile_function!();
        if ctx.input.drag_released || ctx.input.mouse_primary_pressed {
            None
        } else {
            if !ctx.input.action_modifier.do_not_deselect_selected_points() {
                memory.selection.clear_selected_control_points();
            }
            memory
                .shape_control_points
                .find_points_in_rect(
                    &ctx.transform
                        .ui_to_canvas_content
                        .transform_rect(&utils::normalize_rect(&self.rect)),
                )
                .iter()
                .for_each(|(_, index)| {
                    memory.selection.select_control_point(*index);
                });
            let selection_shape = style.selection_shape(self.rect.min, self.rect.max);
            ctx.painter.add(selection_shape);
            self.rect.max = ctx.input.mouse_pos;
            Some(self)
        }
    }
}

impl Interaction for Pan {
    fn update(
        mut self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        _shape: &mut Shape,
        _style: &dyn Style,
        _options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        puffin_egui::puffin::profile_function!();
        if ctx.input.drag_released || ctx.input.mouse_primary_pressed {
            None
        } else {
            // Cannot use ::update_transform method there due to borrow checks
            memory.transform = memory
                .transform
                .translate(ctx.input.mouse_pos - self.start_pos);
            memory.grid.take();
            self.start_pos = ctx.input.mouse_pos;
            Some(self)
        }
    }
}

impl Interaction for AddPointsThanShape {
    fn update(
        mut self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        let mouse_pos = memory
            .snap
            .snap_point
            .unwrap_or(ctx.input.canvas_content_mouse_pos);
        if ctx.input.mouse_primary_clicked {
            self.points.push(mouse_pos);
        }
        if self.points.len() < self.points_count {
            let mut preview_points = self.points.clone();
            preview_points.push(mouse_pos);
            let mut preview_vec_shape: Vec<Shape> = preview_points
                .iter()
                .map(|p| {
                    Shape::circle_stroke(
                        *p,
                        style.control_point_radius(),
                        style.preview_point_stroke(),
                    )
                })
                .collect();
            if let Some(shape_preview) = (self.shape_fn)(&preview_points, options) {
                preview_vec_shape.insert(0, shape_preview);
            }
            ctx.painter.add(
                ctx.transform
                    .canvas_content_to_ui
                    .transform_shape(&Shape::Vec(preview_vec_shape)),
            );
            Some(self)
        } else {
            if let Some(new_shape) = (self.shape_fn)(&self.points, options) {
                let action = action::InsertShape::from_shape(new_shape);
                memory.apply_boxed_action(Box::new(action), shape);
            }
            None
        }
    }
}

impl Interaction for Scroll {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        _shape: &mut Shape,
        _style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        memory.update_transform(
            memory
                .transform
                .translate(ctx.input.mouse_scroll_delta.mul(options.scroll_factor)),
        );
        None
    }
}

impl Interaction for Zoom {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        _shape: &mut Shape,
        _style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        if let Some(canvas_hover_pos) = ctx.input.canvas_mouse_hover_pos {
            let new_transform = memory.transform.resize_at(
                ctx.input.mouse_zoom_delta.powf(options.zoom_factor),
                canvas_hover_pos,
            );
            let new_transform_scale = new_transform.scale();
            let range = &options.scaling_range;
            if range.start.x <= new_transform_scale.x
                && range.start.y <= new_transform_scale.y
                && range.end.x >= new_transform_scale.x
                && range.end.y >= new_transform_scale.y
            {
                memory.update_transform(new_transform);
            }
        }
        None
    }
}

impl AddPointsThanShape {
    pub fn new(
        points_count: usize,
        shape_fn: fn(&Vec<Pos2>, &ShapeEditorOptions) -> Option<Shape>,
    ) -> Self {
        Self {
            points: Vec::default(),
            points_count,
            shape_fn,
        }
    }

    pub fn with_start_point(
        start_point: Pos2,
        points_count: usize,
        shape_fn: fn(&Vec<Pos2>, &ShapeEditorOptions) -> Option<Shape>,
    ) -> Self {
        Self {
            points: vec![start_point],
            points_count,
            shape_fn,
        }
    }
}
