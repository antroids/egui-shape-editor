use crate::shape_editor::shape_visitor::ShapePointIndex;
use egui::ahash::{HashMap, HashSet};
use egui::{Pos2, Vec2};
use num_traits::Bounded;
use ordered_float::NotNan;
use std::ops::{Bound, RangeBounds};

#[derive(Default, Clone)]
pub struct Constraints {
    constraints: HashSet<Constraint>,

    pub(crate) translation_propagation: HashMap<ShapePointIndex, HashSet<ShapePointIndex>>,
    pub(crate) point_position_range: HashMap<ShapePointIndex, PositionRange>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PositionRange {
    x_min: Bound<NotNan<f32>>,
    x_max: Bound<NotNan<f32>>,
    y_min: Bound<NotNan<f32>>,
    y_max: Bound<NotNan<f32>>,
}

impl Constraints {
    pub fn add_constraint(&mut self, constraint: Constraint) -> bool {
        self.constraints
            .insert(constraint)
            .then(|| self.rebuild_index())
            .is_some()
    }

    pub fn remove_constraint(&mut self, constraint: &Constraint) -> bool {
        self.constraints
            .remove(constraint)
            .then(|| self.rebuild_index())
            .is_some()
    }

    pub fn constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
    }

    fn rebuild_index(&mut self) {
        self.clear_index();
        for &constraint in &self.constraints {
            match constraint {
                Constraint::LinkTranslationBidirectional(index1, index2) => {
                    insert_translation_propagation(
                        &mut self.translation_propagation,
                        index1,
                        index2,
                    );
                    insert_translation_propagation(
                        &mut self.translation_propagation,
                        index2,
                        index1,
                    );
                }
                Constraint::LinkTranslationFromTo(from, to) => {
                    insert_translation_propagation(&mut self.translation_propagation, from, to);
                }
                Constraint::PointPositionRange(index, position_range) => {
                    self.point_position_range.insert(index, position_range);
                }
            }
        }
    }

    fn clear_index(&mut self) {
        self.translation_propagation.clear();
    }
}

fn insert_translation_propagation(
    translation_propagation: &mut HashMap<ShapePointIndex, HashSet<ShapePointIndex>>,
    from: ShapePointIndex,
    to: ShapePointIndex,
) {
    translation_propagation
        .entry(from)
        .and_modify(|set| {
            set.insert(to);
        })
        .or_insert_with(|| HashSet::from_iter([to]));
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Constraint {
    LinkTranslationBidirectional(ShapePointIndex, ShapePointIndex),
    LinkTranslationFromTo(ShapePointIndex, ShapePointIndex),
    PointPositionRange(ShapePointIndex, PositionRange),
}

impl PositionRange {
    pub fn clamp_translation(&self, mut translation: Vec2, position: Pos2) -> Vec2 {
        match self.x_max {
            Bound::Included(x_max) => {
                translation.x = translation.x.min(*(x_max - position.x));
            }
            Bound::Excluded(x_max) => {
                translation.x = translation.x.min(*(x_max - position.x)) - f32::EPSILON
            }
            Bound::Unbounded => {}
        }

        match self.x_min {
            Bound::Included(x_min) => {
                translation.x = translation.x.max(*(x_min - position.x));
            }
            Bound::Excluded(x_min) => {
                translation.x = translation.x.max(*(x_min - position.x)) - f32::EPSILON
            }
            Bound::Unbounded => {}
        }

        match self.y_max {
            Bound::Included(y_max) => {
                translation.y = translation.y.min(*(y_max - position.y));
            }
            Bound::Excluded(y_max) => {
                translation.y = translation.y.min(*(y_max - position.y)) - f32::EPSILON
            }
            Bound::Unbounded => {}
        }

        match self.y_min {
            Bound::Included(y_min) => {
                translation.y = translation.y.max(*(y_min - position.y));
            }
            Bound::Excluded(y_min) => {
                translation.y = translation.y.max(*(y_min - position.y)) - f32::EPSILON
            }
            Bound::Unbounded => {}
        }

        translation
    }
}

impl<X: RangeBounds<f32>, Y: RangeBounds<f32>> From<(X, Y)> for PositionRange {
    fn from(value: (X, Y)) -> Self {
        let x_min = value
            .0
            .start_bound()
            .map(|&v| NotNan::new(v).unwrap_or(NotNan::min_value()));
        let y_min = value
            .1
            .start_bound()
            .map(|&v| NotNan::new(v).unwrap_or(NotNan::min_value()));
        let x_max = value
            .0
            .end_bound()
            .map(|&v| NotNan::new(v).unwrap_or(NotNan::max_value()));
        let y_max = value
            .1
            .end_bound()
            .map(|&v| NotNan::new(v).unwrap_or(NotNan::max_value()));

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
