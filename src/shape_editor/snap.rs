use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::control_point::ShapeControlPoints;
use crate::shape_editor::index::{GridLineType, SnapComponent};
use crate::shape_editor::{style, ShapeEditorMemory};
use egui::ahash::{HashSet, HashSetExt};
use egui::{Pos2, Rect, Shape, Vec2};
use std::cmp::Ordering;
use std::ops::Sub;

#[derive(Clone, Debug)]
pub enum SnapTarget {
    ShapeControlPoint(Pos2),
    GridHorizontal(f32),
    GridVertical(f32),
}

#[derive(Clone, Debug, Default)]
pub struct SnapInfo {
    pub targets: Vec<SnapTarget>,
    manual_snap_x: Option<f32>,
    manual_snap_y: Option<f32>,
    pub snap_point: Option<Pos2>,
}

impl SnapInfo {}

impl ShapeEditorMemory {
    fn calculate_snap_point_x(
        &mut self,
        pos: Pos2,
        max_distance: f32,
        ignored_grid_line_types: &HashSet<GridLineType>,
    ) -> Option<f32> {
        let control_point_snap =
            self.shape_control_points
                .snap_x(pos, max_distance, &self.selection.control_points());
        let grid_snap = self
            .grid
            .as_ref()
            .map(|grid| grid.snap_x(pos, max_distance, ignored_grid_line_types))
            .unwrap_or_default();
        calculate_snap_point_component(
            &mut self.snap,
            pos.x,
            &self.shape_control_points,
            control_point_snap,
            grid_snap,
            SnapTarget::GridHorizontal,
        )
    }

    fn calculate_snap_point_y(
        &mut self,
        pos: Pos2,
        max_distance: f32,
        ignored_grid_line_types: &HashSet<GridLineType>,
    ) -> Option<f32> {
        let control_point_snap =
            self.shape_control_points
                .snap_y(pos, max_distance, &self.selection.control_points());
        let grid_snap = self
            .grid
            .as_ref()
            .map(|grid| grid.snap_y(pos, max_distance, ignored_grid_line_types))
            .unwrap_or_default();
        calculate_snap_point_component(
            &mut self.snap,
            pos.y,
            &self.shape_control_points,
            control_point_snap,
            grid_snap,
            SnapTarget::GridVertical,
        )
    }

    pub(crate) fn calculate_snap_point(&mut self, pos: Pos2, max_distance: f32) {
        let mut ignored_grid_line_types = HashSet::with_capacity(1);
        ignored_grid_line_types.insert(GridLineType::Sub);
        let max_distance_x = if self.snap.manual_snap_x.is_some() {
            0.0
        } else {
            max_distance
        };
        let max_distance_y = if self.snap.manual_snap_y.is_some() {
            0.0
        } else {
            max_distance
        };
        self.snap.targets.clear();
        let snap_x = self.calculate_snap_point_x(pos, max_distance_x, &ignored_grid_line_types);
        let snap_y = self.calculate_snap_point_y(pos, max_distance_y, &ignored_grid_line_types);

        if snap_x.is_some()
            || snap_y.is_some()
            || self.snap.manual_snap_x.is_some()
            || self.snap.manual_snap_y.is_some()
        {
            self.snap.snap_point.replace(Pos2::new(
                snap_x.or(self.snap.manual_snap_x).unwrap_or(pos.x),
                snap_y.or(self.snap.manual_snap_y).unwrap_or(pos.y),
            ));
        } else {
            self.snap.snap_point = None;
        }
    }

    pub(crate) fn clear_snap_point(&mut self) {
        self.snap.targets.clear();
        self.snap.manual_snap_x.take();
        self.snap.manual_snap_y.take();
        self.snap.snap_point.take();
    }
}

fn calculate_snap_point_component<F: FnOnce(f32) -> SnapTarget>(
    snap: &mut SnapInfo,
    component_pos: f32,
    shape_control_points: &ShapeControlPoints,
    control_point_snap: Option<SnapComponent>,
    grid_snap: Option<f32>,
    grid_snap_target: F,
) -> Option<f32> {
    match (control_point_snap, grid_snap) {
        (Some((component_value, index_set)), None) => {
            snap.targets.extend(index_set.iter().filter_map(|index| {
                shape_control_points
                    .pos_by_index(index)
                    .map(SnapTarget::ShapeControlPoint)
            }));
            Some(component_value)
        }
        (None, Some(component_value)) => {
            snap.targets.push(grid_snap_target(component_value));
            Some(component_value)
        }
        (Some((point_component_value, index_set)), Some(grid_component_value)) => {
            let point_distance = point_component_value.sub(component_pos).abs();
            let grid_distance = grid_component_value.sub(component_pos).abs();
            match point_distance.total_cmp(&grid_distance) {
                Ordering::Less => {
                    snap.targets.extend(index_set.iter().filter_map(|index| {
                        shape_control_points
                            .pos_by_index(index)
                            .map(SnapTarget::ShapeControlPoint)
                    }));
                    Some(point_component_value)
                }
                Ordering::Equal => {
                    snap.targets.extend(index_set.iter().filter_map(|index| {
                        shape_control_points
                            .pos_by_index(index)
                            .map(SnapTarget::ShapeControlPoint)
                    }));
                    snap.targets.push(grid_snap_target(grid_component_value));
                    Some(point_component_value)
                }
                Ordering::Greater => {
                    snap.targets.push(grid_snap_target(grid_component_value));
                    Some(grid_component_value)
                }
            }
        }
        _ => None,
    }
}

pub fn paint_snap_point_highlight(
    ctx: &CanvasContext,
    snap_info: &SnapInfo,
    style: &dyn style::Style,
) {
    puffin_egui::puffin::profile_function!();
    if let Some(snap_point) = snap_info.snap_point {
        let ui_snap_point = ctx.transform.canvas_content_to_ui.transform_pos(snap_point);
        let canvas_rect = ctx.transform.ui_canvas_rect();
        for snap_target in &snap_info.targets {
            let shape = match snap_target {
                SnapTarget::ShapeControlPoint(pos) => {
                    let pos = ctx.transform.canvas_content_to_ui.transform_pos(*pos);
                    let pos_rect = Rect::from_center_size(
                        pos,
                        Vec2::splat(style.snap_highlight_point_mark_size()),
                    );
                    let mut shape = Shape::dashed_line(
                        &[ui_snap_point, pos],
                        style.snap_highlight_stroke(),
                        style.snap_highlight_dash_length(),
                        style.snap_highlight_gap_length(),
                    );
                    shape.extend(Shape::dashed_line(
                        &[pos_rect.left_top(), pos_rect.right_bottom()],
                        style.snap_highlight_stroke(),
                        style.snap_highlight_dash_length(),
                        style.snap_highlight_gap_length(),
                    ));
                    shape.extend(Shape::dashed_line(
                        &[pos_rect.right_top(), pos_rect.left_bottom()],
                        style.snap_highlight_stroke(),
                        style.snap_highlight_dash_length(),
                        style.snap_highlight_gap_length(),
                    ));
                    Shape::Vec(shape)
                }
                SnapTarget::GridHorizontal(x) => {
                    let x = ctx.transform.canvas_content_to_ui.transform_x(*x);
                    Shape::Vec(Shape::dashed_line(
                        &[
                            Pos2::new(x, canvas_rect.top()),
                            Pos2::new(x, canvas_rect.bottom()),
                        ],
                        style.snap_highlight_stroke(),
                        style.snap_highlight_dash_length(),
                        style.snap_highlight_gap_length(),
                    ))
                }
                SnapTarget::GridVertical(y) => {
                    let y = ctx.transform.canvas_content_to_ui.transform_y(*y);
                    Shape::Vec(Shape::dashed_line(
                        &[
                            Pos2::new(canvas_rect.left(), y),
                            Pos2::new(canvas_rect.right(), y),
                        ],
                        style.snap_highlight_stroke(),
                        style.snap_highlight_dash_length(),
                        style.snap_highlight_gap_length(),
                    ))
                }
            };
            ctx.painter.add(shape);
        }
    }
}
