use crate::shape_editor::action::{
    AddShapePoints, InsertShape, RemoveShapePoints, ShapeAction, ShapePoint,
};
use crate::shape_editor::{
    grid, style, utils, MouseDrag, ShapeEditor, ShapeEditorCanvasResponse, ShapeEditorMemory,
    ShapeEditorOptions,
};

use super::transform::Transform;
use crate::shape_editor::action::MoveShapeControlPoints;
use crate::shape_editor::control_point::{ShapeControlPoint, ShapeControlPoints};
use crate::shape_editor::index::GridIndex;
use crate::shape_editor::snap::paint_snap_point_highlight;
use crate::shape_editor::visitor::{LastShapePointIndex, ShapePointIndex, ShapeType};
use egui::ahash::HashMap;
use egui::{
    Color32, Context, Key, KeyboardShortcut, Modifiers, Painter, Pos2, Rect, Response, Sense,
    Shape, Stroke, Ui, Vec2,
};
use itertools::Itertools;
use std::ops::Mul;
use strum::EnumIter;
use strum::IntoEnumIterator;

pub(crate) struct ActionModifier(Modifiers);

impl ActionModifier {
    fn do_not_deselect_selected_points(&self) -> bool {
        self.0.shift
    }

    fn add_point_on_click(&self) -> bool {
        self.0.ctrl || self.0.command
    }

    fn snap_mouse_cursor(&self) -> bool {
        self.0.alt
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

#[derive(Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum KeyboardAction {
    AddPoint,
    DeletePoint,
    Undo,
}

impl KeyboardAction {
    const SHORTCUT_ADD_POINT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::I);
    const SHORTCUT_DELETE_POINT: KeyboardShortcut =
        KeyboardShortcut::new(Modifiers::NONE, Key::Delete);
    const SHORTCUT_UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Z);

    fn default_keyboard_shortcut(&self) -> &KeyboardShortcut {
        match self {
            KeyboardAction::AddPoint => &Self::SHORTCUT_ADD_POINT,
            KeyboardAction::DeletePoint => &Self::SHORTCUT_DELETE_POINT,
            KeyboardAction::Undo => &Self::SHORTCUT_UNDO,
        }
    }
}

pub(crate) struct CanvasInput {
    pub mouse_hover_pos: Option<Pos2>,
    pub mouse_pos: Pos2,
    pub canvas_content_mouse_pos: Pos2,
    pub canvas_mouse_hover_pos: Option<Pos2>,
    pub mouse_primary_pressed: bool,
    pub mouse_primary_clicked: bool,
    pub mouse_secondary_pressed: bool,
    pub drag_started: bool,
    pub drag_released: bool,
    pub action_modifier: ActionModifier,
    pub keyboard_action: Option<KeyboardAction>,
}

impl CanvasInput {
    fn new(
        options: &ShapeEditorOptions,
        response: &Response,
        ui: &Ui,
        transform: &CanvasTransform,
        last_mouse_hover_pos: Pos2,
    ) -> Self {
        let mouse_hover_pos = response.hover_pos();
        let mouse_pos = mouse_hover_pos.unwrap_or(last_mouse_hover_pos);
        let canvas_content_mouse_pos = transform.ui_to_canvas_content.transform_pos(mouse_pos);
        let canvas_mouse_hover_pos =
            mouse_hover_pos.map(|pos| transform.ui_to_canvas.transform_pos(pos));
        let (
            mouse_primary_pressed,
            mouse_secondary_pressed,
            action_modifier,
            mouse_primary_clicked,
            canvas_action,
        ) = ui.input_mut(|input| {
            (
                input.pointer.primary_pressed(),
                input.pointer.secondary_pressed(),
                ActionModifier(input.modifiers),
                input.pointer.primary_clicked(),
                KeyboardAction::iter().find(|canvas_action| {
                    input.consume_shortcut(
                        options
                            .keyboard_shortcuts
                            .get(canvas_action)
                            .unwrap_or(canvas_action.default_keyboard_shortcut()),
                    )
                }),
            )
        });
        let drag_started = response.drag_started();
        let drag_released = response.drag_released();

        Self {
            mouse_hover_pos,
            mouse_pos,
            canvas_content_mouse_pos,
            canvas_mouse_hover_pos,
            mouse_primary_pressed,
            mouse_primary_clicked,
            mouse_secondary_pressed,
            action_modifier,
            drag_started,
            drag_released,
            keyboard_action: canvas_action,
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
    fn new(
        canvas_rect: Rect,
        options: &ShapeEditorOptions,
        memory: &ShapeEditorMemory,
        response: &Response,
        ui: &Ui,
    ) -> Self {
        let transform = CanvasTransform::new(canvas_rect, &memory.transform);
        let input = CanvasInput::new(
            options,
            response,
            ui,
            &transform,
            memory.last_mouse_hover_pos,
        );
        let painter = ui.painter_at(canvas_rect);
        Self {
            transform,
            input,
            painter,
        }
    }
}

impl<'a> ShapeEditor<'a> {
    pub(crate) fn show_canvas(
        &mut self,
        ui: &mut Ui,
        egui_ctx: &Context,
        outer_rect: Rect,
        memory: &mut ShapeEditorMemory,
    ) -> ShapeEditorCanvasResponse {
        puffin_egui::puffin::profile_function!();

        let margins = self.style.rulers_margins();
        let canvas_rect = margins.shrink_rect(outer_rect);
        let response = ui.allocate_rect(canvas_rect, Sense::drag());
        let mut ctx = CanvasContext::new(canvas_rect, &self.options, memory, &response, ui);

        self.canvas_context_menu(response.clone(), memory, &ctx);
        paint_canvas_background(&ctx, self.style);
        self.handle_actions(memory, &ctx);
        update_snap_point(&ctx, memory, &self.options);
        handle_drag_in_progress(memory, self.shape, self.style, &ctx);
        handle_drag_released(memory, &ctx);
        handle_scroll_and_zoom(memory, ui, &self.options, ctx.input.canvas_mouse_hover_pos);
        ctx.transform = CanvasTransform::new(canvas_rect, &memory.transform);

        let mut ui_shape = ctx
            .transform
            .canvas_content_to_ui
            .transform_shape(self.shape);
        memory.shape_control_points = ShapeControlPoints::collect(self.shape);
        let ui_shape_control_points = ShapeControlPoints::collect(&mut ui_shape);
        let hovered_ui_shape_points = ctx
            .input
            .mouse_hover_pos
            .map(|pos| {
                ui_shape_control_points.points_in_radius(pos, self.style.control_point_radius())
            })
            .unwrap_or_default();

        let grid_index = memory
            .grid
            .get_or_insert_with(|| GridIndex::from_transform(&ctx.transform));
        grid::paint_grid(&ctx, self.style, grid_index);
        ctx.painter.add(ui_shape);
        handle_primary_pressed(&ctx, &hovered_ui_shape_points, memory);

        paint_shape_control_points(
            &ui_shape_control_points,
            &ctx,
            memory,
            self.style,
            &hovered_ui_shape_points,
        );
        paint_snap_point_highlight(&ctx, &memory.snap, self.style);
        paint_canvas_border(&ctx, self.style);

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

    fn handle_actions(&mut self, memory: &mut ShapeEditorMemory, ctx: &CanvasContext) {
        puffin_egui::puffin::profile_function!();
        if let Some(keyboard_action) = ctx.input.keyboard_action {
            match keyboard_action {
                KeyboardAction::AddPoint => self.handle_add_point(memory, ctx.input.mouse_pos),
                KeyboardAction::DeletePoint => {
                    self.apply_action(
                        RemoveShapePoints(memory.selection.control_points().clone()),
                        memory,
                    );
                }
                KeyboardAction::Undo => {
                    memory.undo(self.shape);
                }
            }
        } else if ctx.input.action_modifier.add_point_on_click() && ctx.input.mouse_primary_clicked
        {
            self.handle_add_point(
                memory,
                memory
                    .snap
                    .snap_point
                    .unwrap_or(ctx.input.canvas_content_mouse_pos),
            );
        }
    }

    fn handle_add_point(&mut self, memory: &mut ShapeEditorMemory, mouse_pos: Pos2) {
        let Some(selected_point) = memory.selection.single_control_point() else {
            return;
        };
        let Some(ShapeControlPoint::PathPoint { position, .. }) = memory
            .shape_control_points
            .by_index(selected_point)
            .cloned()
        else {
            return;
        };
        if let Some(shape_type) = memory
            .shape_control_points
            .shape_type_by_control_point(selected_point)
        {
            match shape_type {
                ShapeType::Circle => {}
                ShapeType::LineSegment => {}
                ShapeType::Path => {
                    let new_point_index = selected_point.next_point();
                    self.apply_action(
                        AddShapePoints::single_point(new_point_index, ShapePoint::Pos(mouse_pos)),
                        memory,
                    );
                    memory
                        .selection
                        .select_single_control_point(new_point_index);
                }
                ShapeType::Rect => {}
                ShapeType::Text => {}
                ShapeType::Mesh => {}
                ShapeType::QuadraticBezier => {
                    let control_point = memory
                        .shape_control_points
                        .connected_bezier_control_point(selected_point);
                    self.apply_action(
                        InsertShape::quadratic_bezier_from_two_points(
                            position,
                            control_point,
                            mouse_pos,
                            self.options.stroke,
                        ),
                        memory,
                    );
                    if let Some(last_index) = LastShapePointIndex::last_index(self.shape) {
                        memory.selection.select_single_control_point(last_index);
                    }
                }
                ShapeType::CubicBezier => {
                    let start_control_point = memory
                        .shape_control_points
                        .connected_bezier_control_point(selected_point);
                    self.apply_action(
                        InsertShape::cubic_bezier_from_two_points(
                            position,
                            start_control_point,
                            mouse_pos,
                            self.options.stroke,
                        ),
                        memory,
                    );
                    if let Some(last_index) = LastShapePointIndex::last_index(self.shape) {
                        memory.selection.select_single_control_point(last_index);
                    }
                }
                ShapeType::Callback => {}
            }
        }
    }
}

fn update_snap_point(
    ctx: &CanvasContext,
    memory: &mut ShapeEditorMemory,
    options: &ShapeEditorOptions,
) {
    puffin_egui::puffin::profile_function!();
    if options.snap_enabled_by_default != ctx.input.action_modifier.snap_mouse_cursor() {
        memory.calculate_snap_point(
            ctx.input.canvas_content_mouse_pos,
            ctx.transform.ui_to_canvas_content.scale().x * options.snap_distance,
        );
    } else {
        memory.clear_snap_point();
    }
}

fn handle_drag_started(memory: &mut ShapeEditorMemory, ctx: &CanvasContext) {
    puffin_egui::puffin::profile_function!();
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
    style: &dyn style::Style,
    ctx: &CanvasContext,
) {
    puffin_egui::puffin::profile_function!();
    let mouse_pos = ctx.input.mouse_pos;
    match &mut memory.mouse_drag {
        None => {}
        Some(MouseDrag::MoveShapeControlPoints(_, pos)) => {
            if *pos != ctx.input.canvas_content_mouse_pos {
                let snap_point = memory
                    .snap
                    .snap_point
                    .unwrap_or(ctx.input.canvas_content_mouse_pos);
                Box::new(MoveShapeControlPoints::from_index_and_translation(
                    memory.selection.control_points(),
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
                .find_points_in_rect(
                    &ctx.transform
                        .ui_to_canvas_content
                        .transform_rect(&utils::normalize_rect(rect)),
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
    puffin_egui::puffin::profile_function!();
    if ctx.input.drag_released || ctx.input.mouse_primary_pressed {
        if let Some(MouseDrag::MoveShapeControlPoints(start_pos, pos)) = memory.mouse_drag.take() {
            if pos != start_pos && memory.selection.has_control_points() {
                let move_action = MoveShapeControlPoints::from_index_and_translation(
                    memory.selection.control_points(),
                    &(pos - start_pos),
                );
                memory
                    .push_action_history(Box::new(move_action.invert()), move_action.short_name());
            }
        }
    }
}

fn handle_scroll_and_zoom(
    memory: &mut ShapeEditorMemory,
    ui: &mut Ui,
    options: &ShapeEditorOptions,
    mouse_hover_pos: Option<Pos2>,
) {
    puffin_egui::puffin::profile_function!();
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
    hovered_ui_shape_points: &HashMap<ShapePointIndex, ShapeControlPoint>,
    memory: &mut ShapeEditorMemory,
) {
    puffin_egui::puffin::profile_function!();
    if ctx.input.canvas_mouse_hover_pos.is_some()
        && ctx.input.mouse_primary_pressed
        && memory.mouse_drag.is_none()
    {
        let next_selected =
            memory
                .selection
                .single_control_point()
                .and_then(|single_selected_index| {
                    hovered_ui_shape_points
                        .keys()
                        .skip_while(|hovered_index| **hovered_index != *single_selected_index)
                        .skip(1)
                        .next()
                        .copied()
                });
        if !(ctx.input.action_modifier.do_not_deselect_selected_points()
            || ctx.input.action_modifier.add_point_on_click()
            || ctx.input.drag_started
                && hovered_ui_shape_points
                    .keys()
                    .any(|hovered_index| memory.selection.is_control_point_selected(hovered_index)))
        {
            memory.selection.clear_selected_control_points();
        }

        if let Some(index_to_select) =
            next_selected.or(hovered_ui_shape_points.keys().next().copied())
        {
            memory.selection.select_control_point(index_to_select);
        }
    }
}

fn paint_canvas_border(ctx: &CanvasContext, style: &dyn style::Style) {
    puffin_egui::puffin::profile_function!();
    ctx.painter.rect(
        *ctx.transform.ui_canvas_rect(),
        0.0,
        Color32::TRANSPARENT,
        style.border_stroke(),
    );
}

fn paint_canvas_background(ctx: &CanvasContext, style: &dyn style::Style) {
    puffin_egui::puffin::profile_function!();
    ctx.painter.rect(
        *ctx.transform.ui_canvas_rect(),
        0.0,
        style.canvas_bg_color(),
        Stroke::NONE,
    );
}

fn paint_shape_control_points(
    ui_shape_control_points: &ShapeControlPoints,
    ctx: &CanvasContext,
    memory: &ShapeEditorMemory,
    style: &dyn style::Style,
    hovered: &HashMap<ShapePointIndex, ShapeControlPoint>,
) {
    puffin_egui::puffin::profile_function!();
    for (index, ui_shape_point) in ui_shape_control_points
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
    {
        let hovered = hovered.contains_key(index);
        let selected = memory.selection.is_control_point_selected(index);
        ctx.painter
            .add(ui_shape_point.to_shape(hovered, selected, style));
    }
}
