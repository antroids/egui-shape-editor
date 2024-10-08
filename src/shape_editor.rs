use crate::shape_editor::canvas::{CanvasContext, KeyboardAction};
use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::shape_params::ApplyShapeParams;
pub use crate::shape_editor::shape_params::{ParamType, ParamValue, ShapesParams};
pub use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType};
use egui::ahash::{HashMap, HashSet};
use egui::{Color32, Context, Id, KeyboardShortcut, Response, Sense, Shape, Stroke, Ui, Vec2};
use memory::ShapeEditorMemory;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use transform::Transform;

mod canvas;
mod canvas_context_menu;
pub mod constraints;
mod control_point;
mod grid;
mod index;
mod interaction;
mod memory;
mod rulers;
mod shape_action;
mod shape_params;
mod shape_visitor;
mod snap;
pub mod style;
mod transform;
mod utils;

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
    pub scaling_range: Range<Vec2>,
    pub stroke: Stroke,
    pub snap_distance: f32,
    pub snap_enabled_by_default: bool,
    pub keyboard_shortcuts: HashMap<KeyboardAction, KeyboardShortcut>,
    pub context_menu_add_shapes: Vec<ShapeType>,
    pub connect_chained_shapes: bool,
}

impl Default for ShapeEditorOptions {
    fn default() -> Self {
        let context_menu_add_shapes = vec![
            ShapeType::Path,
            ShapeType::LineSegment,
            ShapeType::Circle,
            ShapeType::Ellipse,
            ShapeType::Rect,
            ShapeType::QuadraticBezier,
            ShapeType::CubicBezier,
            ShapeType::Mesh,
        ];
        Self {
            scroll_factor: Vec2::new(0.1, 0.1),
            zoom_factor: 0.2,
            scaling_range: Vec2::splat(0.01)..Vec2::splat(10.0),
            stroke: Stroke::new(1.0, Color32::BLACK),
            snap_distance: 5.0,
            snap_enabled_by_default: true,
            keyboard_shortcuts: Default::default(),
            context_menu_add_shapes,
            connect_chained_shapes: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct Selection {
    control_points: BTreeSet<ShapePointIndex>,
}

impl Selection {
    pub fn has_control_points(&self) -> bool {
        !self.control_points.is_empty()
    }

    pub fn select_control_point(&mut self, index: ShapePointIndex) {
        self.control_points.insert(index);
    }

    pub fn select_single_control_point(&mut self, index: ShapePointIndex) {
        self.clear_selected_control_points();
        self.select_control_point(index);
    }

    pub fn is_control_point_selected(&self, index: &ShapePointIndex) -> bool {
        self.control_points.contains(index)
    }

    pub fn clear_selected_control_points(&mut self) {
        self.control_points.clear();
    }

    pub fn single_control_point(&self) -> Option<&ShapePointIndex> {
        if self.control_points.len() == 1 {
            self.control_points.iter().next()
        } else {
            None
        }
    }

    pub fn control_points(&self) -> &BTreeSet<ShapePointIndex> {
        &self.control_points
    }

    pub(crate) fn control_points_mut(&mut self) -> &mut BTreeSet<ShapePointIndex> {
        &mut self.control_points
    }

    pub fn shapes(&self) -> HashSet<usize> {
        self.control_points
            .iter()
            .map(|point| point.shape_index)
            .collect()
    }

    pub fn deselect_control_points(&mut self, control_points: &[ShapePointIndex]) {
        control_points.iter().for_each(|index| {
            self.control_points.remove(index);
        });
    }
}

pub struct ShapeEditorResponse {
    pub response: Response,
}

pub struct ShapeEditorCanvasResponse {
    pub response: Response,
}

impl<'a> ShapeEditor<'a> {
    pub fn show(mut self, ui: &mut Ui, egui_ctx: &Context) -> ShapeEditorResponse {
        let rect = ui.available_rect_before_wrap();
        let outer_rect = rect;
        let mut memory = ShapeEditorMemory::load(egui_ctx, self.id);
        let margins = self.style.rulers_margins();
        let canvas_rect = margins.shrink_rect(outer_rect);
        let response = ui.allocate_rect(canvas_rect, Sense::click_and_drag());
        let ctx = CanvasContext::new(
            self.shape,
            canvas_rect,
            &self.options,
            &mut memory,
            &response,
            ui,
            self.style,
        );

        self.show_canvas(response.clone(), egui_ctx, &ctx, &mut memory);

        let ui_painter = ui.painter();
        rulers::paint_rulers(self.style, ui_painter, outer_rect, &ctx);

        memory.store(egui_ctx, self.id);

        ShapeEditorResponse { response }
    }

    fn apply_action(&mut self, action: impl ShapeAction + 'static, memory: &mut ShapeEditorMemory) {
        self.apply_boxed_action(Box::new(action), memory)
    }

    fn apply_boxed_action(&mut self, action: Box<dyn ShapeAction>, memory: &mut ShapeEditorMemory) {
        memory.apply_boxed_action(action, self.shape);
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
            mem.action_history().len()
        })
    }

    pub fn scale(&self, ctx: &Context) -> Transform {
        memory_mut(self.id, ctx, |mem| mem.transform().clone())
    }

    pub fn set_scale(&self, ctx: &Context, transform: Transform) {
        memory_mut(self.id, ctx, |mem| mem.set_transform(transform));
    }

    pub fn options_mut(&mut self) -> &mut ShapeEditorOptions {
        &mut self.options
    }

    pub fn selection(&self, ctx: &Context) -> Selection {
        memory_mut(self.id, ctx, |mem| mem.selection().clone())
    }

    pub fn selection_shapes_params(&mut self, ctx: &Context) -> ShapesParams {
        ShapesParams::extract(self.shape, self.selection(ctx).shapes())
    }

    pub fn apply_shapes_params(&mut self, ctx: &Context, params: ShapesParams) {
        memory_mut(self.id, ctx, |mem| {
            self.apply_action(ApplyShapeParams(params.0), mem)
        })
    }

    pub fn apply_common_shapes_params(
        &mut self,
        ctx: &Context,
        params: BTreeMap<ParamType, ParamValue>,
    ) {
        memory_mut(self.id, ctx, |mem| {
            self.apply_action(
                ApplyShapeParams::from_common(params, mem.selection().shapes()),
                mem,
            )
        })
    }

    pub fn with_constraints_mut<R>(
        &self,
        ctx: &Context,
        func: impl FnOnce(&mut Constraints) -> R,
    ) -> R {
        memory_mut(self.id, ctx, |mem| func(&mut mem.constraints))
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
