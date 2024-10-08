use crate::shape_editor::{
    grid, index, style, Selection, ShapeEditor, ShapeEditorCanvasResponse, ShapeEditorOptions,
};

use super::transform::Transform;
use crate::shape_editor::control_point::{ShapeControlPoint, ShapeControlPoints};
use crate::shape_editor::index::GridIndex;
use crate::shape_editor::memory::ShapeEditorMemory;
use crate::shape_editor::shape_visitor::ShapePointIndex;
use crate::shape_editor::snap::{paint_snap_point_highlight, SnapInfo};
use egui::ahash::HashMap;
use egui::emath::One;
use egui::{
    Color32, Context, Key, KeyboardShortcut, Modifiers, Painter, PointerButton, Pos2, Rect,
    Response, Shape, Stroke, Ui, Vec2,
};
use itertools::Itertools;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Default, Debug)]
pub(crate) struct ActionModifier(Modifiers);

impl ActionModifier {
    pub fn do_not_deselect_selected_points(&self) -> bool {
        self.0.shift
    }

    pub fn add_point_on_click(&self) -> bool {
        self.0.ctrl || self.0.command
    }

    pub fn snap_mouse_cursor(&self) -> bool {
        self.0.alt
    }
}

#[derive(Debug)]
pub(crate) struct CanvasTransform {
    pub ui_to_canvas: Transform,
    pub canvas_content_to_canvas: Transform,
    pub canvas_content_to_ui: Transform,
    pub ui_to_canvas_content: Transform,
}

impl CanvasTransform {
    fn new(canvas_rect: Rect, content_transform: &Transform) -> Self {
        let canvas_to_ui = Transform::from_to(
            Rect::from_min_size(Pos2::ZERO, canvas_rect.size()),
            canvas_rect,
        );
        let canvas_content_to_canvas = content_transform.clone();
        let ui_to_canvas = canvas_to_ui.inverse();
        let canvas_content_to_ui = Transform::combine(&canvas_to_ui, content_transform);
        let ui_to_canvas_content = canvas_content_to_ui.inverse();
        Self {
            ui_to_canvas,
            canvas_content_to_canvas,
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

#[derive(Clone, Copy, EnumIter, PartialEq, Eq, Hash, Debug)]
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

#[derive(Debug)]
pub(crate) struct CanvasInput {
    pub mouse_hover_pos: Option<Pos2>,
    pub mouse_pos: Pos2,
    pub canvas_content_mouse_pos: Pos2,
    pub canvas_mouse_hover_pos: Option<Pos2>,
    pub mouse_primary_pressed: bool,
    pub mouse_primary_clicked: bool,
    pub mouse_primary_down: bool,
    pub mouse_secondary_pressed: bool,
    pub mouse_secondary_down: bool,
    pub drag_started: bool,
    pub drag_stopped: bool,
    pub action_modifier: ActionModifier,
    pub keyboard_action: Option<KeyboardAction>,
    pub mouse_scroll_delta: Vec2,
    pub mouse_zoom_delta: f32,
    pub drag_delta: Vec2,
}

impl CanvasInput {
    fn new(
        options: &ShapeEditorOptions,
        response: &Response,
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
            mouse_primary_down,
            mouse_secondary_down,
            action_modifier,
            mouse_primary_clicked,
            canvas_action,
            mouse_scroll_delta,
            mouse_zoom_delta,
        ) = if response.context_menu_opened() {
            (
                false,
                false,
                false,
                false,
                ActionModifier::default(),
                false,
                None,
                Vec2::ZERO,
                f32::ONE,
            )
        } else {
            response.ctx.input_mut(|input| {
                (
                    input.pointer.button_pressed(PointerButton::Primary),
                    input.pointer.button_pressed(PointerButton::Secondary),
                    input.pointer.button_down(PointerButton::Primary),
                    input.pointer.button_down(PointerButton::Secondary),
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
                    input.smooth_scroll_delta,
                    input.zoom_delta(),
                )
            })
        };
        let drag_started = response.drag_started();
        let drag_stopped = response.drag_stopped();
        let drag_delta = response.drag_delta();

        Self {
            mouse_hover_pos,
            mouse_pos,
            canvas_content_mouse_pos,
            canvas_mouse_hover_pos,
            mouse_primary_pressed,
            mouse_primary_clicked,
            mouse_primary_down,
            mouse_secondary_pressed,
            mouse_secondary_down,
            action_modifier,
            drag_started,
            drag_stopped,
            keyboard_action: canvas_action,
            mouse_scroll_delta,
            mouse_zoom_delta,
            drag_delta,
        }
    }

    pub fn primary_drag_started(&self) -> bool {
        self.drag_started && self.mouse_primary_down
    }

    pub fn secondary_drag_started(&self) -> bool {
        self.drag_started && self.mouse_secondary_down
    }
}

pub(crate) struct CanvasContext {
    pub(crate) transform: CanvasTransform,
    pub(crate) input: CanvasInput,
    pub(crate) painter: Painter,
    pub(crate) grid_index: GridIndex,
    pub(crate) hovered_ui_shape_points: HashMap<ShapePointIndex, ShapeControlPoint>,
    pub(crate) ui_shape: Shape,
    pub(crate) ui_shape_control_points: ShapeControlPoints,
    pub(crate) shape_control_points: ShapeControlPoints,
}

impl CanvasContext {
    pub(crate) fn new(
        shape: &mut Shape,
        canvas_rect: Rect,
        options: &ShapeEditorOptions,
        memory: &mut ShapeEditorMemory,
        response: &Response,
        ui: &Ui,
        style: &dyn style::Style,
    ) -> Self {
        let transform = CanvasTransform::new(canvas_rect, &memory.transform());
        let input = CanvasInput::new(options, response, &transform, memory.last_mouse_hover_pos());
        let painter = ui.painter_at(canvas_rect);
        let grid_index = GridIndex::from_transform(&transform);
        let mut ui_shape = transform.canvas_content_to_ui.transform_shape(shape);
        let ui_shape_control_points = ShapeControlPoints::collect(&mut ui_shape);
        let shape_control_points = ShapeControlPoints::collect(shape);
        let hovered_ui_shape_points = input
            .mouse_hover_pos
            .map(|pos| ui_shape_control_points.points_in_radius(pos, style.control_point_radius()))
            .unwrap_or_default();
        let selection = memory.selection().clone();
        if options.snap_enabled_by_default != input.action_modifier.snap_mouse_cursor() {
            SnapInfo::update_snap_info(
                &mut memory.snap,
                input.canvas_content_mouse_pos,
                transform.ui_to_canvas_content.scale().x * options.snap_distance,
                &grid_index,
                &shape_control_points,
                &selection,
            );
        } else {
            memory.snap.clear();
        }

        Self {
            transform,
            input,
            painter,
            grid_index,
            ui_shape,
            hovered_ui_shape_points,
            ui_shape_control_points,
            shape_control_points,
        }
    }

    pub(crate) fn closest_selected_control_point(
        &self,
        selection: &Selection,
    ) -> Option<&ShapeControlPoint> {
        self.shape_control_points
            .iter()
            .filter_map(|(index, point)| {
                selection.is_control_point_selected(index).then_some(point)
            })
            .min_by_key(|point| {
                index::not_nan_f32(
                    point
                        .position()
                        .distance(self.input.canvas_content_mouse_pos),
                )
            })
    }
}

impl<'a> ShapeEditor<'a> {
    pub(crate) fn show_canvas(
        &mut self,
        response: Response,
        egui_ctx: &Context,
        ctx: &CanvasContext,
        memory: &mut ShapeEditorMemory,
    ) -> ShapeEditorCanvasResponse {
        puffin_egui::puffin::profile_function!();
        self.canvas_context_menu(response.clone(), memory, &ctx);
        paint_canvas_background(&ctx, self.style);

        grid::paint_grid(&ctx, self.style);
        ctx.painter.add(ctx.ui_shape.clone());

        memory.current_frame_interactions(&ctx);
        memory.update_interaction(self.shape, self.style, &self.options, &ctx);

        paint_shape_control_points(&ctx, memory, self.style);
        paint_snap_point_highlight(&ctx, &memory.snap(), self.style);
        paint_canvas_border(&ctx, self.style);

        memory.next_frame_interactions(&ctx);

        if !egui_ctx.is_context_menu_open() {
            if let Some(mouse_hover_pos) = ctx.input.mouse_hover_pos {
                memory.set_last_mouse_hover_pos(mouse_hover_pos);
            }
            if let Some(canvas_mouse_hover_pos) = ctx.input.canvas_mouse_hover_pos {
                memory.set_last_canvas_mouse_hover_pos(canvas_mouse_hover_pos);
            }
        }
        ShapeEditorCanvasResponse { response }
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
    ctx: &CanvasContext,
    memory: &ShapeEditorMemory,
    style: &dyn style::Style,
) {
    puffin_egui::puffin::profile_function!();
    for (index, ui_shape_point) in ctx
        .ui_shape_control_points
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
    {
        let hovered = ctx.hovered_ui_shape_points.contains_key(index);
        let selected = memory.selection().is_control_point_selected(index);
        ctx.painter
            .add(ui_shape_point.to_shape(hovered, selected, style));
    }
}
