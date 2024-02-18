use crate::shape_editor::action::{Action, InsertShape, ShapeAction};
use crate::shape_editor::index::{GridIndex, ShapeControlPointsIndex};
use crate::shape_editor::visitor::{IndexedShapeControlPointsVisitorAdapter, ShapeVisitor};
use egui::ahash::{HashMap, HashSet};
use egui::epaint::CircleShape;
use egui::{
    Color32, Context, Id, Key, KeyboardShortcut, Margin, Modifiers, Pos2, Rangef, Rect, Response,
    Sense, Shape, Stroke, Ui, Vec2,
};
use std::ops::Range;
use transform::Transform;

mod action;
mod canvas;
mod grid;
mod index;
mod rulers;
mod snap;
mod transform;
mod visitor;

pub struct ShapeEditor<'shape> {
    pub id: Id,
    pub shape: &'shape mut Shape,
    pub style: ShapeEditorStyle,
    pub options: ShapeEditorOptions,
}

#[derive(Clone)]
pub struct ShapeEditorStyle {
    path_point_stroke: Stroke,
    control_point_radius: f32,
    bezier_control_point_stroke: Stroke,

    canvas_bg_color: Color32,

    border_stroke: Stroke,

    selection_stroke: Stroke,
    selection_dash_length: f32,
    selection_gap_length: f32,

    rulers_width: f32,
    rulers_stroke: Stroke,
    rulers_half_stroke: Stroke,
    rulers_sub_stroke: Stroke,
    rulers_font: egui::FontId,
    rulers_font_color: Color32,
    rulers_div_height: f32,
    rulers_half_div_height: f32,
    rulers_sub_div_height: f32,
    rulers_text_position: Pos2,

    grid_line_zero_stroke: Stroke,
    grid_line_primary_stroke: Stroke,
    grid_line_secondary_stroke: Stroke,
    grid_line_secondary_gap: f32,
    grid_line_secondary_length: f32,

    snap_highlight_stroke: Stroke,
    snap_highlight_dash_length: f32,
    snap_highlight_gap_length: f32,
    snap_highlight_point_mark_size: f32,
}

impl Default for ShapeEditorStyle {
    fn default() -> Self {
        Self {
            path_point_stroke: Stroke::new(2.0, Color32::RED),
            control_point_radius: 5.0,
            bezier_control_point_stroke: Stroke::new(2.0, Color32::GREEN),
            canvas_bg_color: Color32::WHITE,
            border_stroke: Stroke::new(3.0, Color32::GRAY),

            selection_stroke: Stroke::new(1.0, Color32::GRAY),
            selection_dash_length: 2.0,
            selection_gap_length: 2.0,

            rulers_width: 16.0,
            rulers_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_half_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_sub_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_font: egui::FontId::monospace(8.0),
            rulers_font_color: Color32::GRAY,
            rulers_div_height: 3.0,
            rulers_half_div_height: 2.0,
            rulers_sub_div_height: 1.0,
            rulers_text_position: Pos2::new(0.0, 2.0),

            grid_line_zero_stroke: Stroke::new(2.0, Color32::LIGHT_GRAY),
            grid_line_primary_stroke: Stroke::new(0.5, Color32::LIGHT_GRAY),
            grid_line_secondary_stroke: Stroke::new(0.5, Color32::LIGHT_GRAY),
            grid_line_secondary_gap: 3.0,
            grid_line_secondary_length: 3.0,

            snap_highlight_stroke: Stroke::new(1.0, Color32::GRAY),
            snap_highlight_dash_length: 5.0,
            snap_highlight_gap_length: 5.0,
            snap_highlight_point_mark_size: 20.0,
        }
    }
}

impl ShapeEditorStyle {
    fn selection_shape(&self, min: Pos2, max: Pos2) -> Shape {
        let mut vec = Shape::dashed_line(
            &[min, Pos2::new(max.x, min.y)],
            self.selection_stroke.clone(),
            self.selection_dash_length,
            self.selection_gap_length,
        );
        vec.extend(Shape::dashed_line(
            &[min, Pos2::new(min.x, max.y)],
            self.selection_stroke.clone(),
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        vec.extend(Shape::dashed_line(
            &[Pos2::new(min.x, max.y), max],
            self.selection_stroke.clone(),
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        vec.extend(Shape::dashed_line(
            &[Pos2::new(max.x, min.y), max],
            self.selection_stroke.clone(),
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        Shape::Vec(vec)
    }

    fn rulers_margins(&self) -> Margin {
        Margin {
            left: self.rulers_width,
            right: 0.0,
            top: self.rulers_width,
            bottom: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct ShapeEditorOptions {
    scroll_factor: Vec2,
    zoom_factor: f32,
    undo_shortcut: KeyboardShortcut,
    scaling_range: Range<Vec2>,
    stroke: Stroke,
}

impl Default for ShapeEditorOptions {
    fn default() -> Self {
        Self {
            scroll_factor: Vec2::new(0.1, 0.1),
            zoom_factor: 0.2,
            undo_shortcut: KeyboardShortcut::new(Modifiers::CTRL, Key::Z),
            scaling_range: Vec2::splat(0.01)..Vec2::splat(10.0),
            stroke: Stroke::new(1.0, Color32::BLACK),
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

    fn deselect_control_point(&mut self, index: usize) {
        self.control_points.remove(&index);
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
    action_history: Vec<Action>,
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum ShapeControlPoint {
    PathPoint(Pos2),
    BezierControlPoint(Pos2, HashMap<usize, Pos2>),
}

impl ShapeControlPoint {
    fn position(&self) -> Pos2 {
        match self {
            ShapeControlPoint::PathPoint(pos) => *pos,
            ShapeControlPoint::BezierControlPoint(pos, ..) => *pos,
        }
    }

    fn stroke(&self, style: &ShapeEditorStyle) -> Stroke {
        match self {
            ShapeControlPoint::PathPoint(_) => style.path_point_stroke,
            ShapeControlPoint::BezierControlPoint(..) => style.bezier_control_point_stroke,
        }
    }

    fn to_shape(&self, hovered: bool, selected: bool, style: &ShapeEditorStyle) -> Shape {
        let stroke = self.stroke(style);
        let radius = style.control_point_radius;
        let pos = self.position();
        let mut vec_shape = if let Self::BezierControlPoint(pos, path_point) = self {
            path_point
                .values()
                .map(|connected_pos| Shape::LineSegment {
                    points: [*pos, *connected_pos],
                    stroke: Stroke::new(1.0, Color32::GRAY),
                })
                .collect()
        } else {
            Vec::default()
        };

        vec_shape.push(Shape::circle_stroke(pos, radius, stroke));
        if hovered {
            vec_shape.push(Shape::circle_filled(
                pos,
                radius,
                stroke.color.linear_multiply(0.5),
            ));
        }
        if selected {
            vec_shape.push(Shape::circle_stroke(pos, radius + 2.0, stroke))
        }

        Shape::Vec(vec_shape)
    }
}

#[derive(Default, Clone, Debug)]
struct ShapeControlPoints {
    control_points: Vec<ShapeControlPoint>,
    index: ShapeControlPointsIndex,
}

impl ShapeControlPoints {
    fn collect(shape: &mut Shape) -> Self {
        let mut slf = Self::default();
        IndexedShapeControlPointsVisitorAdapter(&mut slf).visit(shape);
        slf.rebuild_index();
        slf
    }

    fn points_in_radius(&self, pos: Pos2, radius: f32) -> HashMap<usize, ShapeControlPoint> {
        self.index
            .find_points_in_distance(pos, radius)
            .iter()
            .map(|(_, index)| (*index, self.control_points[*index].clone()))
            .collect()
    }

    fn connected_bezier_control_point(&self, path_point_index: usize) -> Option<Pos2> {
        self.control_points.iter().find_map(|point| {
            if let ShapeControlPoint::BezierControlPoint(pos, connected) = point {
                connected.contains_key(&path_point_index).then_some(*pos)
            } else {
                None
            }
        })
    }

    fn rebuild_index(&mut self) {
        self.index.clear();
        self.control_points
            .iter()
            .enumerate()
            .for_each(|(index, point)| self.index.insert(point.position(), index));
    }

    fn by_index(&self, index: usize) -> Option<&ShapeControlPoint> {
        self.control_points.get(index)
    }

    fn pos_by_index(&self, index: usize) -> Option<Pos2> {
        self.by_index(index).map(|p| p.position())
    }
}

impl PartialEq for ShapeControlPoints {
    fn eq(&self, other: &Self) -> bool {
        self.control_points.eq(&other.control_points)
    }
}

impl IntoIterator for ShapeControlPoints {
    type Item = ShapeControlPoint;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.control_points.into_iter()
    }
}

fn grid_step(scale: f32) -> f32 {
    50f32 * 5f32.powi(-scale.log(5.0).round() as i32)
}

fn step_by(range: Rangef, step: f32) -> impl Iterator<Item = f32> {
    let min = (range.min / step).floor() as i32;
    let max = (range.max / step).ceil() as i32;
    (min..max).map(move |i| i as f32 * step)
}

impl<'shape> ShapeEditor<'shape> {
    pub fn show(mut self, ui: &mut Ui, ctx: &Context) -> ShapeEditorResponse {
        let rect = ui.available_rect_before_wrap();
        let outer_rect = rect;
        let response = ui.allocate_rect(outer_rect, Sense::drag());
        let mut memory = ShapeEditorMemory::load(ctx, self.id);

        self.show_canvas(ui, ctx, outer_rect, &mut memory);

        let ui_painter = ui.painter();
        rulers::paint_rulers(&self.style, &ui_painter, outer_rect, &memory);

        if ui.input_mut(|input| input.consume_shortcut(&self.options.undo_shortcut)) {
            memory.undo(self.shape);
        }

        memory.store(ctx, self.id);

        ShapeEditorResponse { response }
    }

    fn apply_action(&mut self, action: Action, memory: &mut ShapeEditorMemory) {
        memory.action_history.push(action.apply(self.shape))
    }

    fn canvas_context_menu(&mut self, response: Response, memory: &mut ShapeEditorMemory) {
        response.context_menu(|ui| {
            ui.menu_button("Add shape", |ui| {
                if ui.button("Circle").clicked() {
                    self.apply_action(
                        Action::InsertShape(InsertShape {
                            replace: None,
                            shape: Some(Shape::Circle(CircleShape::stroke(
                                memory.last_canvas_mouse_hover_pos,
                                50.0,
                                Stroke::new(1.0, Color32::RED),
                            ))),
                        }),
                        memory,
                    );
                    ui.close_menu();
                }
            });
        });
    }
}

fn memory_mut<R>(id: Id, ctx: &Context, func: impl FnOnce(&mut ShapeEditorMemory) -> R) -> R {
    ctx.data_mut(|data| {
        let memory = data.get_temp_mut_or_insert_with(id, Default::default);
        func(memory)
    })
}

impl<'shape> ShapeEditor<'shape> {
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

pub struct ShapeEditorBuilder<'shape> {
    id: Id,
    shape: &'shape mut Shape,
    style: Option<ShapeEditorStyle>,
    options: Option<ShapeEditorOptions>,
}

impl<'shape> ShapeEditorBuilder<'shape> {
    pub fn new(id: Id, shape: &'shape mut Shape) -> Self {
        Self {
            id,
            shape,
            style: None,
            options: None,
        }
    }

    pub fn style(mut self, style: ShapeEditorStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn options(mut self, options: ShapeEditorOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build(self) -> ShapeEditor<'shape> {
        ShapeEditor {
            id: self.id,
            shape: self.shape,
            style: self.style.unwrap_or_default(),
            options: self.options.unwrap_or_default(),
        }
    }
}
