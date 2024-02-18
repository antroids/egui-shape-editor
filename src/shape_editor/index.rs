use crate::shape_editor::canvas::CanvasTransform;
use egui::ahash::{HashMap, HashSet};
use egui::{Pos2, Rect};
use num_traits::Bounded;
use ordered_float::{FloatCore, NotNan};
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::{RangeBounds, Sub};

#[derive(Clone, Default, Debug)]
pub struct FloatIndex<K: FloatCore, V>(pub BTreeMap<NotNan<K>, HashSet<V>>);

impl<K: FloatCore, V: Eq + Hash + Copy> FloatIndex<K, V> {
    pub fn find_in_range<R: RangeBounds<NotNan<K>>>(
        &self,
        range: R,
    ) -> btree_map::Range<NotNan<K>, HashSet<V>> {
        self.0.range(range)
    }

    pub fn find_in_distance(
        &self,
        key: NotNan<K>,
        max_distance: NotNan<K>,
    ) -> btree_map::Range<NotNan<K>, HashSet<V>> {
        let d = max_distance.abs();
        self.find_in_range(key - d..=key + d)
    }

    pub fn find_closest_in_distance_and_ignore(
        &self,
        key: NotNan<K>,
        max_distance: NotNan<K>,
        ignore_values: &HashSet<V>,
    ) -> Option<(&NotNan<K>, HashSet<V>)> {
        let d = max_distance.abs();
        [
            self.0
                .range(key - d..=key)
                .filter(|(_, value)| {
                    ignore_values.is_empty()
                        || value.iter().any(|value| !ignore_values.contains(value))
                })
                .last()
                .map(|(key, value)| (key, value.sub(ignore_values))),
            self.0
                .range(key..=key + d)
                .filter(|(_, value)| {
                    ignore_values.is_empty()
                        || value.iter().any(|value| !ignore_values.contains(value))
                })
                .next()
                .map(|(key, value)| (key, value.sub(ignore_values))),
        ]
        .into_iter()
        .filter_map(|v| v)
        .min_by_key(|(k, _)| NotNan::new(k.sub(key).abs()).unwrap_or(NotNan::max_value()))
    }

    pub fn find_closest_in_distance(
        &self,
        key: NotNan<K>,
        max_distance: NotNan<K>,
    ) -> Option<(&NotNan<K>, &HashSet<V>)> {
        let d = max_distance.abs();
        [
            self.0.range(key - d..=key).last(),
            self.0.range(key..=key + d).next(),
        ]
        .iter()
        .filter_map(|v| *v)
        .min_by_key(|(k, _)| NotNan::new(k.sub(key).abs()).unwrap_or(NotNan::max_value()))
    }

    pub fn insert(&mut self, key: NotNan<K>, value: V) {
        if let Some(set) = self.0.get_mut(&key) {
            set.insert(value);
        } else {
            self.0.insert(key, HashSet::from_iter([value]));
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Clone, Default, Debug)]
pub struct ShapeControlPointsIndex {
    pub x_index: FloatIndex<f32, usize>,
    pub y_index: FloatIndex<f32, usize>,
}

impl ShapeControlPointsIndex {
    pub fn insert(&mut self, pos: Pos2, index: usize) {
        let not_nan_pos = not_nan_pos2(pos);
        self.x_index.insert(not_nan_pos.0, index);
        self.y_index.insert(not_nan_pos.1, index);
    }

    pub fn find_points_in_distance(&self, pos: Pos2, max_distance: f32) -> Vec<(Pos2, usize)> {
        let not_nan_pos = not_nan_pos2(pos);
        let x_points = self
            .x_index
            .find_in_distance(not_nan_pos.0, not_nan_f32(max_distance));
        let y_points = self
            .y_index
            .find_in_distance(not_nan_pos.1, not_nan_f32(max_distance));
        let y_points_index: HashMap<usize, NotNan<f32>> = y_points
            .flat_map(|(y, y_index_set)| y_index_set.iter().map(|index| (*index, *y)))
            .collect();
        x_points
            .flat_map(|(x, index_set)| {
                index_set.iter().filter_map(|index| {
                    y_points_index.get(index).and_then(|y| {
                        let point_pos = Pos2::new(x.into_inner(), y.into_inner());
                        (point_pos.distance(pos) <= max_distance).then_some((point_pos, *index))
                    })
                })
            })
            .collect()
    }

    pub fn find_closest_point_in_distance(
        &self,
        pos: Pos2,
        max_distance: f32,
    ) -> Vec<(Pos2, usize)> {
        let not_nan_pos = not_nan_pos2(pos);
        let x_points = self
            .x_index
            .find_in_distance(not_nan_pos.0, not_nan_f32(max_distance));
        let y_points = self
            .y_index
            .find_in_distance(not_nan_pos.1, not_nan_f32(max_distance));
        let y_points_index: HashMap<usize, NotNan<f32>> = y_points
            .flat_map(|(y, y_index_set)| y_index_set.iter().map(|index| (*index, *y)))
            .collect();
        x_points
            .flat_map(|(x, index_set)| {
                index_set.iter().filter_map(|index| {
                    y_points_index.get(index).and_then(|y| {
                        let point_pos = Pos2::new(x.into_inner(), y.into_inner());
                        (point_pos.distance(pos) <= max_distance).then_some((point_pos, *index))
                    })
                })
            })
            .collect()
    }

    pub fn find_points_in_rect(&self, rect: &Rect) -> Vec<(Pos2, usize)> {
        let x_points = self
            .x_index
            .find_in_range(not_nan_f32(rect.left())..=not_nan_f32(rect.right()));
        let y_points = self
            .y_index
            .find_in_range(not_nan_f32(rect.top())..=not_nan_f32(rect.bottom()));
        let y_points_index: HashMap<usize, NotNan<f32>> = y_points
            .flat_map(|(y, y_index_set)| y_index_set.iter().map(|index| (*index, *y)))
            .collect();
        x_points
            .flat_map(|(x, index_set)| {
                index_set.iter().filter_map(|index| {
                    y_points_index
                        .get(index)
                        .map(|y| (Pos2::new(x.into_inner(), y.into_inner()), *index))
                })
            })
            .collect()
    }

    pub fn find_position_by_index(&self, index: usize) -> Option<Pos2> {
        self.x_index
            .0
            .iter()
            .find_map(|(x, i)| i.contains(&index).then_some(x))
            .and_then(|x| {
                self.y_index.0.iter().find_map(|(y, i)| {
                    i.contains(&index)
                        .then_some(Pos2::new(x.into_inner(), y.into_inner()))
                })
            })
    }

    pub fn snap_point(
        &self,
        pos: Pos2,
        max_distance: f32,
        ignore: &HashSet<usize>,
    ) -> (Option<(f32, HashSet<usize>)>, Option<(f32, HashSet<usize>)>) {
        let x = self
            .x_index
            .find_closest_in_distance_and_ignore(
                not_nan_f32(pos.x),
                not_nan_f32(max_distance),
                ignore,
            )
            .map(|(x, index)| (x.into_inner(), index));
        let y = self
            .y_index
            .find_closest_in_distance_and_ignore(
                not_nan_f32(pos.y),
                not_nan_f32(max_distance),
                ignore,
            )
            .map(|(y, index)| (y.into_inner(), index));
        (x, y)
    }

    pub fn clear(&mut self) {
        self.x_index.clear();
        self.y_index.clear();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum GridLineType {
    #[default]
    Zero,
    Primary,
    Secondary,
    Sub,
}

#[derive(Clone, Default, Debug)]
pub(crate) struct GridIndex {
    pub horizontal: FloatIndex<f32, GridLineType>,
    pub vertical: FloatIndex<f32, GridLineType>,
}

impl GridIndex {
    pub fn from_transform(transform: &CanvasTransform) -> Self {
        let mut slf = Self::default();
        let canvas_viewport = transform.canvas_content_viewport();
        let x_range = canvas_viewport.x_range();
        let y_range = canvas_viewport.y_range();
        let scale = transform.canvas_content_to_ui.scale().x;
        let step = super::grid_step(scale);
        let half_step = step / 2.0;
        let sub_step = half_step / 3.0;
        for x in super::step_by(x_range, step) {
            if x == 0.0 {
                slf.add_horizontal(x, GridLineType::Zero);
            } else {
                slf.add_horizontal(x, GridLineType::Primary);
            }
            slf.add_horizontal(x + half_step, GridLineType::Secondary);
            slf.add_horizontal(x + sub_step, GridLineType::Sub);
            slf.add_horizontal(x + sub_step * 2.0, GridLineType::Sub);
            slf.add_horizontal(x + half_step + sub_step, GridLineType::Sub);
            slf.add_horizontal(x + half_step + sub_step * 2.0, GridLineType::Sub);
        }
        let scale = transform.canvas_content_to_ui.scale().y;
        let step = super::grid_step(scale);
        let half_step = step / 2.0;
        let sub_step = half_step / 3.0;
        for y in super::step_by(y_range, step) {
            if y == 0.0 {
                slf.add_vertical(y, GridLineType::Zero);
            } else {
                slf.add_vertical(y, GridLineType::Primary);
            }
            slf.add_vertical(y + half_step, GridLineType::Secondary);
            slf.add_vertical(y + sub_step, GridLineType::Sub);
            slf.add_vertical(y + sub_step * 2.0, GridLineType::Sub);
            slf.add_vertical(y + half_step + sub_step, GridLineType::Sub);
            slf.add_vertical(y + half_step + sub_step * 2.0, GridLineType::Sub);
        }
        slf
    }

    pub fn snap_point(
        &self,
        pos: Pos2,
        max_distance: f32,
        ignore: HashSet<GridLineType>,
    ) -> (Option<f32>, Option<f32>) {
        let x = self
            .horizontal
            .find_closest_in_distance_and_ignore(
                not_nan_f32(pos.x),
                not_nan_f32(max_distance),
                &ignore,
            )
            .map(|(x, _)| x.into_inner());
        let y = self
            .horizontal
            .find_closest_in_distance_and_ignore(
                not_nan_f32(pos.y),
                not_nan_f32(max_distance),
                &ignore,
            )
            .map(|(y, _)| y.into_inner());
        (x, y)
    }

    pub fn add_horizontal(&mut self, x: f32, line_type: GridLineType) {
        self.horizontal.insert(not_nan_f32(x), line_type);
    }

    pub fn add_vertical(&mut self, y: f32, line_type: GridLineType) {
        self.vertical.insert(not_nan_f32(y), line_type);
    }
}

fn not_nan_pos2(pos: Pos2) -> (NotNan<f32>, NotNan<f32>) {
    (not_nan_f32(pos.x), not_nan_f32(pos.y))
}

pub(crate) fn not_nan_f32(v: f32) -> NotNan<f32> {
    NotNan::new(v).unwrap_or(NotNan::max_value())
}
