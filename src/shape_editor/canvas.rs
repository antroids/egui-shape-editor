use crate::shape_editor::action::{InsertShape, ShapeAction};
use crate::shape_editor::{
    grid, MouseDrag, ShapeControlPoint, ShapeControlPoints, ShapeEditor, ShapeEditorCanvasResponse,
    ShapeEditorMemory, ShapeEditorOptions, ShapeEditorStyle,
};

use super::transform::Transform;
use crate::shape_editor::action::{Action, MoveShapeControlPoints};
use crate::shape_editor::index::GridIndex;
use crate::shape_editor::snap::{paint_snap_point_highlight, SnapInfo};
use crate::shape_editor::visitor::{CountShapeControlPoints, GetShapeTypeByPointIndex, ShapeType};
use egui::ahash::HashMap;
use egui::{
    Color32, Context, Modifiers, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2,
};
use std::ops::Mul;

pub(crate) struct ActionModifier(Modifiers);

impl ActionModifier {
    fn do_not_deselect_selected_points(&self) -> bool {
        self.0.shift
    }

    fn add_point_on_click(&self) -> bool {
        self.0.ctrl || self.0.command
    }
}

#[derive(Debug)]
pub(crate) struct CanvasTransform {
    pub ui_to_canvas: Transform,
    pub canvas_content_to_ui: Transform,
    pub ui_to_canvas_content: Transform,
}

impl CanvasTransform {
    fn new(canvas_rect: Rect, content_transform: &Transform) -> Self {
        let canvas_to_ui = Transform::from_to(
            Rect::from_min_size(Pos2::ZERO, canvas_rect.size()),
            canvas_rect,
        );
        let ui_to_canvas = canvas_to_ui.inverse();
        let canvas_content_to_ui = Transform::combine(&canvas_to_ui, content_transform);
        let ui_to_canvas_content = canvas_content_to_ui.inverse();
        Self {
            ui_to_canvas,
            canvas_content_to_ui,
            ui_to_canvas_content,
        }
    }

    pub fn canvas_content_viewport(&self) -> Rect {
        self.ui_to_canvas_content
            .transform_rect(self.ui_to_canvas.0.from())
    }

    pub fn ui_canvas_rect(&self) -> &Rect {
        self.ui_to_canvas.0.from()
    }
}

pub(crate) struct CanvasInput {
    pub mouse_hover_pos: Option<Pos2>,
    pub mouse_pos: Pos2,
    pub canvas_mouse_hover_pos: Option<Pos2>,
    pub mouse_primary_pressed: bool,
    pub mouse_primary_clicked: bool,
    pub mouse_secondary_pressed: bool,
    pub drag_started: bool,
    pub drag_released: bool,
    pub drag_delta: Vec2,
    pub action_modifier: ActionModifier,
}

impl CanvasInput {
    fn new(
        response: &Response,
        ui: &Ui,
        transform: &CanvasTransform,
        last_mouse_hover_pos: Pos2,
    ) -> Self {
        let mouse_hover_pos = response.hover_pos();
        let mouse_pos = mouse_hover_pos.unwrap_or(last_mouse_hover_pos);
        let canvas_mouse_hover_pos =
            mouse_hover_pos.map(|pos| transform.ui_to_canvas.transform_pos(pos));
        let (
            mouse_primary_pressed,
            mouse_secondary_pressed,
            action_modifier,
            mouse_primary_clicked,
        ) = ui.input(|input| {
            (
                input.pointer.primary_pressed(),
                input.pointer.secondary_pressed(),
                ActionModifier(input.modifiers),
                input.pointer.primary_clicked(),
            )
        });
        let drag_started = response.drag_started();
        let drag_released = response.drag_released();
        let drag_delta = response.drag_delta();

        Self {
            mouse_hover_pos,
            mouse_pos,
            canvas_mouse_hover_pos,
            mouse_primary_pressed,
            mouse_primary_clicked,
            mouse_secondary_pressed,
            action_modifier,
            drag_started,
            drag_released,
            drag_delta,
        }
    }

    fn primary_drag_started(&self) -> bool {
        self.drag_started && self.mouse_primary_pressed
    }

    fn secondary_drag_started(&self) -> bool {
        self.drag_started && self.mouse_secondary_pressed
    }
}

pub(crate) struct CanvasContext {
    pub(crate) transform: CanvasTransform,
    pub(crate) input: CanvasInput,
    pub(crate) painter: Painter,
}

impl CanvasContext {
    fn new(canvas_rect: Rect, memory: &ShapeEditorMemory, response: &Response, ui: &Ui) -> Self {
        let transform = CanvasTransform::new(canvas_rect, &memory.transform);
        let input = CanvasInput::new(&response, ui, &transform, memory.last_mouse_hover_pos);
        let painter = ui.painter_at(canvas_rect);
        Self {
            transform,
            input,
            painter,
        }
    }
}

impl<'shape> ShapeEditor<'shape> {
    pub(crate) fn show_canvas(
        &mut self,
        ui: &mut Ui,
        egui_ctx: &Context,
        outer_rect: Rect,
        memory: &mut ShapeEditorMemory,
    ) -> ShapeEditorCanvasResponse {
        let margins = self.style.rulers_margins();
        let canvas_rect = margins.shrink_rect(outer_rect);
        let response = ui.allocate_rect(canvas_rect, Sense::drag());
        let mut ctx = CanvasContext::new(canvas_rect, memory, &response, ui);
        let mut snap_info = SnapInfo::default();

        self.canvas_context_menu(response.clone(), memory);

        ctx.painter
            .rect(canvas_rect, 0.0, self.style.canvas_bg_color, Stroke::NONE);

        if ctx.input.action_modifier.add_point_on_click() && ctx.input.mouse_primary_clicked {
            if let Some(mouse_hover_pos) = ctx.input.mouse_hover_pos {
                self.handle_add_point(
                    memory,
                    ctx.transform
                        .ui_to_canvas_content
                        .transform_pos(mouse_hover_pos),
                );
            }
        }

        handle_drag_in_progress(memory, self.shape, &self.style, &ctx, &mut snap_info);
        handle_drag_released(memory, &ctx);
        handle_scroll_and_zoom(memory, ui, &self.options, ctx.input.canvas_mouse_hover_pos);
        ctx.transform = CanvasTransform::new(canvas_rect, &memory.transform);

        let mut ui_shape = ctx
            .transform
            .canvas_content_to_ui
            .transform_shape(self.shape);
        memory.shape_control_points = ShapeControlPoints::collect(self.shape);
        let ui_shape_points = ShapeControlPoints::collect(&mut ui_shape);
        let hovered_shape_points = ctx
            .input
            .mouse_hover_pos
            .map(|pos| {
                memory.shape_control_points.points_in_radius(
                    ctx.transform.ui_to_canvas_content.transform_pos(pos),
                    self.style.control_point_radius,
                )
            })
            .unwrap_or_default();

        let grid_index = memory
            .grid
            .get_or_insert_with(|| GridIndex::from_transform(&ctx.transform));
        grid::paint_grid(&ctx, &self.style, grid_index);
        ctx.painter.add(ui_shape);
        if ctx.input.canvas_mouse_hover_pos.is_some()
            && ctx.input.mouse_primary_pressed
            && hovered_shape_points.is_empty()
            && !ctx.input.action_modifier.do_not_deselect_selected_points()
            && !ctx.input.action_modifier.add_point_on_click()
        {
            memory.selection.clear_selected_control_points();
        }

        handle_primary_pressed(&ctx, &hovered_shape_points, memory);

        for (index, ui_shape_point) in ui_shape_points.into_iter().enumerate() {
            let hovered = hovered_shape_points.contains_key(&index);
            let selected = memory.selection.is_control_point_selected(index);
            ctx.painter
                .add(ui_shape_point.to_shape(hovered, selected, &self.style));
        }
        paint_snap_point_highlight(&ctx, &snap_info, &self.style);
        ctx.painter.rect(
            canvas_rect,
            0.0,
            Color32::TRANSPARENT,
            self.style.border_stroke,
        );
        handle_drag_started(memory, &ctx);

        if !egui_ctx.is_context_menu_open() {
            if let Some(mouse_hover_pos) = ctx.input.mouse_hover_pos {
                memory.last_mouse_hover_pos = mouse_hover_pos;
            }
            if let Some(canvas_mouse_hover_pos) = ctx.input.canvas_mouse_hover_pos {
                memory.last_canvas_mouse_hover_pos = canvas_mouse_hover_pos;
            }
        }
        ShapeEditorCanvasResponse { response }
    }

    fn handle_add_point(&mut self, memory: &mut ShapeEditorMemory, mouse_pos: Pos2) {
        let Some(start_index) = memory.selection.single_control_point() else {
            return;
        };
        let Some(ShapeControlPoint::PathPoint(start_pos)) =
            memory.shape_control_points.by_index(start_index).cloned()
        else {
            return;
        };
        if let Some(shape_type) = GetShapeTypeByPointIndex::shape_type(self.shape, start_index) {
            match shape_type {
                ShapeType::Circle => {}
                ShapeType::LineSegment => {}
                ShapeType::Path => {}
                ShapeType::Rect => {}
                ShapeType::Text => {}
                ShapeType::Mesh => {}
                ShapeType::QuadraticBezier => {}
                ShapeType::CubicBezier => {
                    let start_control_point = memory
                        .shape_control_points
                        .connected_bezier_control_point(start_index);
                    self.apply_action(
                        Action::InsertShape(InsertShape::cubic_bezier_by_two_points(
                            start_pos,
                            start_control_point,
                            mouse_pos,
                            self.options.stroke,
                        )),
                        memory,
                    );
                    if let Some(last_index) = CountShapeControlPoints::last_index(self.shape) {
                        memory.selection.select_single_control_point(last_index);
                    }
                }
                ShapeType::Callback => {}
            }
        }
    }
}

fn handle_drag_started(memory: &mut ShapeEditorMemory, ctx: &CanvasContext) {
    let mouse_pos = ctx.input.mouse_pos;
    if ctx.input.primary_drag_started() {
        if !ctx.input.action_modifier.do_not_deselect_selected_points() {
            let canvas_mouse_pos = ctx.transform.ui_to_canvas_content.transform_pos(mouse_pos);
            if let Some(closest_selected_control_point) =
                memory.closest_selected_control_point(canvas_mouse_pos)
            {
                memory.mouse_drag = Some(MouseDrag::MoveShapeControlPoints(
                    closest_selected_control_point.position(),
                    closest_selected_control_point.position(),
                ));
                return;
            }
        }
        memory.mouse_drag = Some(MouseDrag::Selection(Rect::from_min_max(
            mouse_pos, mouse_pos,
        )));
    } else if ctx.input.secondary_drag_started() {
        memory.mouse_drag = Some(MouseDrag::Scroll(mouse_pos));
    }
}

fn handle_drag_in_progress(
    memory: &mut ShapeEditorMemory,
    shape: &mut Shape,
    style: &ShapeEditorStyle,
    ctx: &CanvasContext,
    snap_info: &mut SnapInfo,
) {
    let mouse_pos = ctx.input.mouse_pos;
    let canvas_mouse_pos = ctx.transform.ui_to_canvas_content.transform_pos(mouse_pos);
    if matches!(
        memory.mouse_drag,
        Some(MouseDrag::MoveShapeControlPoints(..))
    ) {
        snap_info.calculate_snap_point(canvas_mouse_pos, memory, 5.0);
    }
    match &mut memory.mouse_drag {
        None => {}
        Some(MouseDrag::MoveShapeControlPoints(_, pos)) => {
            if ctx.input.drag_delta != Vec2::ZERO {
                let snap_point = snap_info.snap_point.unwrap_or(canvas_mouse_pos);
                Action::MoveShapeControlPoints(MoveShapeControlPoints::from_index_and_translation(
                    &memory.selection.control_points,
                    &(snap_point - *pos),
                ))
                .apply(shape);
                *pos = snap_point;
            }
        }
        Some(MouseDrag::Selection(rect)) => {
            if !ctx.input.action_modifier.do_not_deselect_selected_points() {
                memory.selection.clear_selected_control_points();
            }
            memory
                .shape_control_points
                .index
                .find_points_in_rect(
                    &ctx.transform
                        .ui_to_canvas_content
                        .transform_rect(&normalize_rect(&rect)),
                )
                .iter()
                .for_each(|(_, index)| {
                    memory.selection.select_control_point(*index);
                });
            let selection_shape = style.selection_shape(rect.min, rect.max);
            ctx.painter.add(selection_shape);
            rect.max = mouse_pos;
        }
        Some(MouseDrag::Scroll(pos)) => {
            // Cannot use ::update_transform method there due to borrow checks
            memory.transform = memory.transform.translate(mouse_pos - *pos);
            memory.grid.take();
            *pos = mouse_pos;
        }
    }
}

fn handle_drag_released(memory: &mut ShapeEditorMemory, ctx: &CanvasContext) {
    if ctx.input.drag_released {
        match memory.mouse_drag.take() {
            Some(MouseDrag::MoveShapeControlPoints(start_pos, pos)) => {
                if pos != start_pos && memory.selection.has_control_points() {
                    memory.action_history.push(Action::MoveShapeControlPoints(
                        MoveShapeControlPoints::from_index_and_translation(
                            &memory.selection.control_points,
                            &(pos - start_pos),
                        )
                        .invert(),
                    ));
                }
            }
            _ => {}
        }
    }
}

fn handle_scroll_and_zoom(
    memory: &mut ShapeEditorMemory,
    ui: &mut Ui,
    options: &ShapeEditorOptions,
    mouse_hover_pos: Option<Pos2>,
) {
    let (scroll_delta, zoom_delta) =
        ui.input(|input| (input.smooth_scroll_delta, input.zoom_delta()));
    if scroll_delta != Vec2::ZERO {
        memory.update_transform(
            memory
                .transform
                .translate(scroll_delta.mul(options.scroll_factor)),
        );
    }
    if let Some(canvas_hover_pos) = mouse_hover_pos {
        if zoom_delta != 1.0 {
            let new_transform = memory
                .transform
                .resize_at(zoom_delta.powf(options.zoom_factor), canvas_hover_pos);
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
    }
}

fn handle_primary_pressed(
    ctx: &CanvasContext,
    hovered_shape_points: &HashMap<usize, ShapeControlPoint>,
    memory: &mut ShapeEditorMemory,
) {
    if ctx.input.canvas_mouse_hover_pos.is_some()
        && ctx.input.mouse_primary_pressed
        && !memory.mouse_drag.is_some()
    {
        let next_selected =
            memory
                .selection
                .single_control_point()
                .and_then(|single_selected_index| {
                    hovered_shape_points
                        .keys()
                        .skip_while(|hovered_index| **hovered_index != single_selected_index)
                        .next()
                        .copied()
                });
        if !ctx.input.action_modifier.do_not_deselect_selected_points()
            && !ctx.input.action_modifier.add_point_on_click()
            && !(ctx.input.drag_started
                && hovered_shape_points.keys().any(|hovered_index| {
                    memory.selection.is_control_point_selected(*hovered_index)
                }))
        {
            memory.selection.clear_selected_control_points();
        }

        if let Some(index_to_select) = next_selected.or(hovered_shape_points.keys().next().copied())
        {
            memory.selection.select_control_point(index_to_select);
        }
    }
}

fn normalize_rect(rect: &Rect) -> Rect {
    let mut rect = rect.clone();
    if rect.left() > rect.right() {
        let temp = rect.left();
        *rect.left_mut() = rect.right();
        *rect.right_mut() = temp;
    }
    if rect.top() > rect.bottom() {
        let temp = rect.top();
        *rect.top_mut() = rect.bottom();
        *rect.bottom_mut() = temp;
    }
    rect
}
