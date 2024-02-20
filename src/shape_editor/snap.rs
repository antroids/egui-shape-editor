use crate::shape_editor::canvas::CanvasContext;
use crate::shape_editor::index::GridLineType;
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
    pub snap_point: Option<Pos2>,
}

impl SnapInfo {
    pub fn calculate_snap_point(
        &mut self,
        pos: Pos2,
        memory: &ShapeEditorMemory,
        max_distance: f32,
    ) {
        let control_point_snap = memory.shape_control_points.index.snap_point(
            pos,
            max_distance,
            &memory.selection.control_points,
        );
        let mut ignored_grid_line_types = HashSet::with_capacity(1);
        ignored_grid_line_types.insert(GridLineType::Sub);
        let grid_snap = memory
            .grid
            .as_ref()
            .map(|grid| grid.snap_point(pos, max_distance, ignored_grid_line_types))
            .unwrap_or_default();
        self.targets.clear();
        let snap_x = match (control_point_snap.0, grid_snap.0) {
            (Some((x, index_set)), None) => {
                self.targets.extend(index_set.iter().filter_map(|index| {
                    memory
                        .shape_control_points
                        .pos_by_index(*index)
                        .map(|pos| SnapTarget::ShapeControlPoint(pos))
                }));
                Some(x)
            }
            (None, Some(x)) => {
                self.targets.push(SnapTarget::GridHorizontal(x));
                Some(x)
            }
            (Some((px, index_set)), Some(gx)) => {
                let px_distance = px.sub(pos.x).abs();
                let gx_distance = gx.sub(pos.x).abs();
                match px_distance.total_cmp(&gx_distance) {
                    Ordering::Less => {
                        self.targets.extend(index_set.iter().filter_map(|index| {
                            memory
                                .shape_control_points
                                .pos_by_index(*index)
                                .map(|pos| SnapTarget::ShapeControlPoint(pos))
                        }));
                        Some(px)
                    }
                    Ordering::Equal => {
                        self.targets.extend(index_set.iter().filter_map(|index| {
                            memory
                                .shape_control_points
                                .pos_by_index(*index)
                                .map(|pos| SnapTarget::ShapeControlPoint(pos))
                        }));
                        self.targets.push(SnapTarget::GridHorizontal(gx));
                        Some(px)
                    }
                    Ordering::Greater => {
                        self.targets.push(SnapTarget::GridHorizontal(gx));
                        Some(gx)
                    }
                }
            }
            _ => None,
        };
        let snap_y = match (control_point_snap.1, grid_snap.1) {
            (Some((y, index_set)), None) => {
                self.targets.extend(index_set.iter().filter_map(|index| {
                    memory
                        .shape_control_points
                        .pos_by_index(*index)
                        .map(|pos| SnapTarget::ShapeControlPoint(pos))
                }));
                Some(y)
            }
            (None, Some(y)) => {
                self.targets.push(SnapTarget::GridVertical(y));
                Some(y)
            }
            (Some((py, index_set)), Some(gy)) => {
                let py_distance = py.sub(pos.y).abs();
                let gy_distance = gy.sub(pos.y).abs();
                match py_distance.total_cmp(&gy_distance) {
                    Ordering::Less => {
                        self.targets.extend(index_set.iter().filter_map(|index| {
                            memory
                                .shape_control_points
                                .pos_by_index(*index)
                                .map(|pos| SnapTarget::ShapeControlPoint(pos))
                        }));
                        Some(py)
                    }
                    Ordering::Equal => {
                        self.targets.extend(index_set.iter().filter_map(|index| {
                            memory
                                .shape_control_points
                                .pos_by_index(*index)
                                .map(|pos| SnapTarget::ShapeControlPoint(pos))
                        }));
                        self.targets.push(SnapTarget::GridVertical(gy));
                        Some(py)
                    }
                    Ordering::Greater => {
                        self.targets.push(SnapTarget::GridVertical(gy));
                        Some(gy)
                    }
                }
            }
            _ => None,
        };
        if snap_x.is_some() || snap_y.is_some() {
            self.snap_point
                .replace(Pos2::new(snap_x.unwrap_or(pos.x), snap_y.unwrap_or(pos.y)));
        }
    }

    pub fn clear(&mut self) {
        self.targets.clear();
        self.snap_point.take();
    }
}

pub fn paint_snap_point_highlight(
    ctx: &CanvasContext,
    snap_info: &SnapInfo,
    style: &dyn style::Style,
) {
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
