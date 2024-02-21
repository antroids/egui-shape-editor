use crate::shape_editor::action::ShapeAction;
use crate::shape_editor::index::GridIndex;
use control_point::{ShapeControlPoint, ShapeControlPoints};
use egui::ahash::HashSet;
use egui::{
    Color32, Context, Id, Key, KeyboardShortcut, Modifiers, Pos2, Rect, Response, Sense, Shape,
    Stroke, Ui, Vec2,
};
use std::ops::Range;
use transform::Transform;

mod action;
mod canvas;
mod canvas_context_menu;
mod control_point;
mod grid;
mod index;
mod rulers;
mod snap;
pub mod style;
mod transform;
mod utils;
mod visitor;

pub struct ShapeEditor<'a> {
    pub id: Id,
    pub shape: &'a mut Shape,
    pub style: &'a dyn style::Style,
    pub options: ShapeEditorOptions,
}

#[derive(Clone)]
pub struct ShapeEditorOptions {
    pub scroll_factor: Vec2,
    pub zoom_factor: f32,
    pub undo_shortcut: KeyboardShortcut,
    pub scaling_range: Range<Vec2>,
    pub stroke: Stroke,
    pub snap_distance: f32,
    pub snap_enabled: bool,
}

impl Default for ShapeEditorOptions {
    fn default() -> Self {
        Self {
            scroll_factor: Vec2::new(0.1, 0.1),
            zoom_factor: 0.2,
            undo_shortcut: KeyboardShortcut::new(Modifiers::CTRL, Key::Z),
            scaling_range: Vec2::splat(0.01)..Vec2::splat(10.0),
            stroke: Stroke::new(1.0, Color32::BLACK),
            snap_distance: 5.0,
            snap_enabled: true,
        }
    }
}

#[derive(Clone)]
enum MouseDrag {
    MoveShapeControlPoints(Pos2, Pos2),
    Selection(Rect),
    Scroll(Pos2),
}

#[derive(Clone, Default)]
pub struct Selection {
    pub control_points: HashSet<usize>,
}

impl Selection {
    fn has_control_points(&self) -> bool {
        !self.control_points.is_empty()
    }

    fn select_control_point(&mut self, index: usize) {
        self.control_points.insert(index);
    }

    fn select_single_control_point(&mut self, index: usize) {
        self.clear_selected_control_points();
        self.select_control_point(index);
    }

    fn is_control_point_selected(&self, index: usize) -> bool {
        self.control_points.contains(&index)
    }

    fn clear_selected_control_points(&mut self) {
        self.control_points.clear();
    }

    fn single_control_point(&self) -> Option<usize> {
        if self.control_points.len() == 1 {
            self.control_points.iter().next().copied()
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ShapeEditorMemory {
    transform: Transform,
    shape_control_points: ShapeControlPoints,
    grid: Option<GridIndex>,
    mouse_drag: Option<MouseDrag>,
    action_history: Vec<Box<dyn ShapeAction>>,
    last_mouse_hover_pos: Pos2,
    last_canvas_mouse_hover_pos: Pos2,
    selection: Selection,
}

impl Default for ShapeEditorMemory {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            shape_control_points: Default::default(),
            grid: None,
            mouse_drag: None,
            action_history: Vec::new(),
            last_mouse_hover_pos: Pos2::ZERO,
            last_canvas_mouse_hover_pos: Pos2::ZERO,
            selection: Selection::default(),
        }
    }
}

impl ShapeEditorMemory {
    fn load(ctx: &Context, id: Id) -> Self {
        ctx.data(|data| data.get_temp(id)).unwrap_or_default()
    }

    fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| data.insert_temp(id, self))
    }

    fn undo(&mut self, shape: &mut Shape) {
        if let Some(action) = self.action_history.pop() {
            action.apply(shape);
        }
    }

    fn closest_selected_control_point(&self, pos: Pos2) -> Option<&ShapeControlPoint> {
        self.shape_control_points
            .control_points
            .iter()
            .enumerate()
            .filter_map(|(index, point)| {
                self.selection
                    .is_control_point_selected(index)
                    .then_some(point)
            })
            .min_by_key(|point| index::not_nan_f32(point.position().distance(pos)))
    }

    fn update_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.grid.take();
    }
}

pub struct ShapeEditorResponse {
    pub response: Response,
}

pub struct ShapeEditorCanvasResponse {
    pub response: Response,
}

impl<'a> ShapeEditor<'a> {
    pub fn show(mut self, ui: &mut Ui, ctx: &Context) -> ShapeEditorResponse {
        let rect = ui.available_rect_before_wrap();
        let outer_rect = rect;
        let response = ui.allocate_rect(outer_rect, Sense::drag());
        let mut memory = ShapeEditorMemory::load(ctx, self.id);

        self.show_canvas(ui, ctx, outer_rect, &mut memory);

        let ui_painter = ui.painter();
        rulers::paint_rulers(self.style, ui_painter, outer_rect, &memory);

        if ui.input_mut(|input| input.consume_shortcut(&self.options.undo_shortcut)) {
            memory.undo(self.shape);
        }

        memory.store(ctx, self.id);

        ShapeEditorResponse { response }
    }

    fn apply_action(&mut self, action: impl ShapeAction, memory: &mut ShapeEditorMemory) {
        memory
            .action_history
            .push(Box::new(action).apply(self.shape))
    }
}

fn memory_mut<R>(id: Id, ctx: &Context, func: impl FnOnce(&mut ShapeEditorMemory) -> R) -> R {
    ctx.data_mut(|data| {
        let memory = data.get_temp_mut_or_insert_with(id, Default::default);
        func(memory)
    })
}

impl<'a> ShapeEditor<'a> {
    pub fn undo(&mut self, ctx: &Context) -> usize {
        memory_mut(self.id, ctx, |mem| {
            mem.undo(self.shape);
            mem.action_history.len()
        })
    }

    pub fn scale(&self, ctx: &Context) -> Transform {
        memory_mut(self.id, ctx, |mem| mem.transform.clone())
    }

    pub fn set_scale(&self, ctx: &Context, transform: Transform) {
        memory_mut(self.id, ctx, |mem| mem.update_transform(transform));
    }

    pub fn options_mut(&mut self) -> &mut ShapeEditorOptions {
        &mut self.options
    }

    pub fn selection(&self, ctx: &Context) -> Selection {
        memory_mut(self.id, ctx, |mem| mem.selection.clone())
    }
}

pub struct ShapeEditorBuilder<'a> {
    id: Id,
    shape: &'a mut Shape,
    style: &'a dyn style::Style,
    options: Option<ShapeEditorOptions>,
}

impl<'a> ShapeEditorBuilder<'a> {
    pub fn new(id: Id, shape: &'a mut Shape, style: &'a dyn style::Style) -> Self {
        Self {
            id,
            shape,
            style,
            options: None,
        }
    }

    pub fn options(mut self, options: ShapeEditorOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build(self) -> ShapeEditor<'a> {
        ShapeEditor {
            id: self.id,
            shape: self.shape,
            style: self.style,
            options: self.options.unwrap_or_default(),
        }
    }
}
