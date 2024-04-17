use crate::shape_editor::canvas::{CanvasContext, KeyboardAction};
use crate::shape_editor::control_point::ShapeControlPoint;
use crate::shape_editor::memory::ShapeEditorMemory;
use crate::shape_editor::shape_action::{
    AddShapePoints, InsertShape, RemoveShapePoints, ShapeAction, ShapePoint,
};
use crate::shape_editor::style::Style;
use crate::shape_editor::visitor::{LastShapePointIndex, ShapeType};
use crate::shape_editor::{shape_action, utils, ShapeEditorOptions};
use dyn_clone::DynClone;
use egui::epaint::{CubicBezierShape, PathShape, QuadraticBezierShape, Vertex};
use egui::{Color32, Mesh, Pos2, Rect, Shape, Vec2};
use std::fmt::Debug;
use std::mem;
use std::ops::Mul;

impl ShapeEditorMemory {
    pub(crate) fn next_frame_interactions(&mut self, ctx: &CanvasContext) {
        puffin_egui::puffin::profile_function!();
        let mouse_pos = ctx.input.mouse_pos;
        if ctx.input.primary_drag_started() && !ctx.input.action_modifier.add_point_on_click() {
            if !ctx.input.action_modifier.do_not_deselect_selected_points() {
                if let Some(closest_selected_control_point) =
                    ctx.closest_selected_control_point(self.selection())
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
        if let Some(keyboard_action) = ctx.input.keyboard_action {
            match keyboard_action {
                KeyboardAction::AddPoint => self.begin_interaction(AddPoint),
                KeyboardAction::DeletePoint => self.begin_interaction(DeletePoints),
                KeyboardAction::Undo => self.begin_interaction(Undo),
            }
        } else if ctx.input.mouse_primary_clicked {
            if ctx.input.action_modifier.add_point_on_click() {
                self.begin_interaction(AddPoint);
            } else {
                self.begin_interaction(ChangeSelectionOnPrimary)
            }
        }

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
        let interactions = mem::replace(self.interaction_mut(), Vec::new());
        if !interactions.is_empty() {
            println!("Interactions for current frame: {:?}", interactions);
        }
        for interaction in interactions {
            if let Some(result) = interaction.update(self, shape, style, options, ctx) {
                self.interaction_mut().push(result)
            }
        }
    }

    pub(crate) fn begin_interaction<T: Interaction + 'static>(&mut self, interaction: T) {
        self.interaction_mut().push(Box::new(interaction));
    }
}

pub(crate) trait Interaction: DynClone + Send + Sync + Debug {
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

#[derive(Clone, Debug)]
pub(crate) struct MoveShapeControlPoints {
    pub start_pos: Pos2,
    pub end_pos: Pos2,
}

#[derive(Clone, Debug)]
struct Selection {
    rect: Rect,
}

#[derive(Clone, Debug)]
struct Pan {
    start_pos: Pos2,
}

#[derive(Clone, Debug)]
pub(crate) struct AddPointsThanShape {
    points: Vec<Pos2>,
    points_count: usize,
    shape_fn: fn(&Vec<Pos2>, &ShapeEditorOptions) -> Option<Shape>,
}

#[derive(Clone, Debug)]
struct Scroll;

#[derive(Clone, Debug)]
struct Zoom;

#[derive(Clone, Debug)]
struct AddPoint;

#[derive(Clone, Debug)]
struct DeletePoints;

#[derive(Clone, Debug)]
struct Undo;

#[derive(Clone, Debug)]
struct ChangeSelectionOnPrimary;

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
            if self.end_pos != self.start_pos && memory.selection().has_control_points() {
                let move_action = shape_action::MoveShapeControlPoints::from_index_and_translation(
                    memory.selection().control_points(),
                    &(self.end_pos - self.start_pos),
                );
                memory
                    .push_action_history(Box::new(move_action.invert()), move_action.short_name());
            }
            None
        } else {
            if (ctx.input.drag_delta != Vec2::ZERO
                && self.end_pos != ctx.input.canvas_content_mouse_pos)
                || self.end_pos != self.start_pos
            {
                let snap_point = memory
                    .snap()
                    .snap_point
                    .unwrap_or(ctx.input.canvas_content_mouse_pos);
                Box::new(
                    shape_action::MoveShapeControlPoints::from_index_and_translation(
                        memory.selection().control_points(),
                        &(snap_point - self.end_pos),
                    ),
                )
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
            self.rect.max = ctx.input.mouse_pos;

            if self.rect.max != self.rect.min {
                if !ctx.input.action_modifier.do_not_deselect_selected_points() {
                    memory.selection_mut().clear_selected_control_points();
                }
                ctx.shape_control_points
                    .find_points_in_rect(
                        &ctx.transform
                            .ui_to_canvas_content
                            .transform_rect(&utils::normalize_rect(&self.rect)),
                    )
                    .iter()
                    .for_each(|(_, index)| {
                        memory.selection_mut().select_control_point(*index);
                    });
                let selection_shape = style.selection_shape(self.rect.min, self.rect.max);
                ctx.painter.add(selection_shape);
            }
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
            memory.set_transform(
                memory
                    .transform()
                    .translate(ctx.input.mouse_pos - self.start_pos),
            );
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
            .snap()
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
                        ctx.transform.ui_to_canvas_content.scale().x * style.control_point_radius(),
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
                let action = InsertShape::from_shape(new_shape);
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
        memory.set_transform(
            memory
                .transform()
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
            let new_transform = memory.transform().resize_at(
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
                memory.set_transform(new_transform);
            }
        }
        None
    }
}

impl Interaction for AddPoint {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        _style: &dyn Style,
        options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        let Some(selected_point) = memory.selection().single_control_point() else {
            return None;
        };
        let Some(ShapeControlPoint::PathPoint { position, .. }) =
            ctx.shape_control_points.by_index(selected_point).cloned()
        else {
            return None;
        };
        if let Some(shape_type) = ctx
            .shape_control_points
            .shape_type_by_control_point(selected_point)
        {
            let mouse_pos = memory
                .snap()
                .snap_point
                .unwrap_or(ctx.input.canvas_content_mouse_pos);
            match shape_type {
                ShapeType::Circle => {}
                ShapeType::LineSegment => {
                    memory.apply_boxed_action(
                        Box::new(InsertShape::from_shape(Shape::LineSegment {
                            points: [position, mouse_pos],
                            stroke: options.stroke,
                        })),
                        shape,
                    );
                    if let Some(last_index) = LastShapePointIndex::last_index(shape) {
                        memory
                            .selection_mut()
                            .select_single_control_point(last_index);
                    }
                }
                ShapeType::Path => {
                    let new_point_index = selected_point.next_point();
                    memory.apply_boxed_action(
                        Box::new(AddShapePoints::single_point(
                            new_point_index,
                            ShapePoint::Pos(mouse_pos),
                        )),
                        shape,
                    );
                    memory
                        .selection_mut()
                        .select_single_control_point(new_point_index);
                }
                ShapeType::Rect => {}
                ShapeType::Text => {}
                ShapeType::Mesh => {}
                ShapeType::QuadraticBezier => {
                    let control_point = ctx
                        .shape_control_points
                        .connected_bezier_control_point(selected_point);
                    memory.apply_boxed_action(
                        Box::new(InsertShape::quadratic_bezier_from_two_points(
                            position,
                            control_point,
                            mouse_pos,
                            options.stroke,
                        )),
                        shape,
                    );
                    if let Some(last_index) = LastShapePointIndex::last_index(shape) {
                        memory
                            .selection_mut()
                            .select_single_control_point(last_index);
                    }
                }
                ShapeType::CubicBezier => {
                    let start_control_point = ctx
                        .shape_control_points
                        .connected_bezier_control_point(selected_point);
                    memory.apply_boxed_action(
                        Box::new(InsertShape::cubic_bezier_from_two_points(
                            position,
                            start_control_point,
                            mouse_pos,
                            options.stroke,
                        )),
                        shape,
                    );
                    if let Some(last_index) = LastShapePointIndex::last_index(shape) {
                        memory
                            .selection_mut()
                            .select_single_control_point(last_index);
                    }
                }
                ShapeType::Callback => {}
            }
        }
        None
    }
}

impl Interaction for DeletePoints {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        _style: &dyn Style,
        _options: &ShapeEditorOptions,
        _ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        memory.apply_boxed_action(
            Box::new(RemoveShapePoints(
                memory.selection().control_points().clone(),
            )),
            shape,
        );
        None
    }
}

impl Interaction for Undo {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        shape: &mut Shape,
        _style: &dyn Style,
        _options: &ShapeEditorOptions,
        _ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        memory.undo(shape);
        None
    }
}

impl Interaction for ChangeSelectionOnPrimary {
    fn update(
        self: Box<Self>,
        memory: &mut ShapeEditorMemory,
        _shape: &mut Shape,
        _style: &dyn Style,
        _options: &ShapeEditorOptions,
        ctx: &CanvasContext,
    ) -> Option<Box<dyn Interaction>> {
        puffin_egui::puffin::profile_function!();
        if ctx.input.canvas_mouse_hover_pos.is_some()
            && ctx.input.mouse_primary_pressed
            && memory.interaction().is_empty()
        {
            let next_selected =
                memory
                    .selection()
                    .single_control_point()
                    .and_then(|single_selected_index| {
                        ctx.hovered_ui_shape_points
                            .keys()
                            .skip_while(|hovered_index| **hovered_index != *single_selected_index)
                            .skip(1)
                            .next()
                            .copied()
                    });
            if !(ctx.input.action_modifier.do_not_deselect_selected_points()
                || ctx.input.action_modifier.add_point_on_click()
                || ctx.input.drag_started
                    && ctx.hovered_ui_shape_points.keys().any(|hovered_index| {
                        memory.selection().is_control_point_selected(hovered_index)
                    }))
            {
                memory.selection_mut().clear_selected_control_points();
            }

            if let Some(index_to_select) =
                next_selected.or(ctx.hovered_ui_shape_points.keys().next().copied())
            {
                memory.selection_mut().select_control_point(index_to_select);
            }
        }
        None
    }
}

impl AddPointsThanShape {
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

    pub fn with_shape_type_and_start_point(shape_type: ShapeType, point: Pos2) -> Self {
        match shape_type {
            ShapeType::Circle => Self::with_start_point(point, 2, |points, options| {
                if let &[p0, p1, ..] = points.as_slice() {
                    Some(Shape::circle_stroke(p0, p0.distance(p1), options.stroke))
                } else {
                    None
                }
            }),
            ShapeType::LineSegment => Self::with_start_point(point, 2, |points, options| {
                if let &[p0, p1, ..] = points.as_slice() {
                    Some(Shape::line_segment([p0, p1], options.stroke))
                } else {
                    None
                }
            }),
            ShapeType::Path => Self::with_start_point(point, 2, |points, options| {
                if let &[p0, p1, ..] = points.as_slice() {
                    Some(Shape::Path(PathShape::line(vec![p0, p1], options.stroke)))
                } else {
                    None
                }
            }),
            ShapeType::Rect => Self::with_start_point(point, 2, |points, options| {
                if let &[p0, p1, ..] = points.as_slice() {
                    let rect = utils::normalize_rect(&Rect::from_two_pos(p0, p1));
                    Some(Shape::rect_stroke(rect, 0.0, options.stroke))
                } else {
                    None
                }
            }),
            ShapeType::Text => {
                todo!()
            }
            ShapeType::Mesh => Self::with_start_point(point, 3, |points, options| {
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
            }),
            ShapeType::QuadraticBezier => Self::with_start_point(point, 3, |points, options| {
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
            }),
            ShapeType::CubicBezier => Self::with_start_point(point, 4, |points, options| {
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
            }),
            ShapeType::Callback => {
                todo!()
            }
        }
    }
}
