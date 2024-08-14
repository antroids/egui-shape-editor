use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::shape_visitor::get_points_positions::GetPointsPositions;
use crate::shape_editor::shape_visitor::indexed_shape_control_points_visitor::{
    IndexedShapeControlPointsVisitor, IndexedShapeControlPointsVisitorAdapter,
};
use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType, ShapeVisitor};
use egui::ahash::{HashMap, HashSet};
use egui::emath::One;
use egui::{Pos2, Shape, Vec2};
use num_traits::Zero;
use std::ops::{AddAssign, DerefMut, MulAssign, Neg};

#[derive(Clone)]
pub struct MoveShapePoints(HashMap<ShapePointIndex, Vec2>);

impl IndexedShapeControlPointsVisitor for MoveShapePoints {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
            point.add_assign(translation);
            if shape_type.is_center_and_radius_topography() {
                let mut index = index;
                for _ in 0..ShapeType::MAX_RADIUS_POINTS {
                    index.assign_next_point();
                    self.0.remove(&index);
                }
            }
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
        shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
            if !shape_type.is_center_and_radius_topography()
                || !self.0.contains_key(&index.first_point())
            {
                control_point.add_assign(translation);
            }
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }
}

impl MoveShapePoints {
    pub fn from_index_and_translation<'a>(
        indexes: impl IntoIterator<Item = &'a ShapePointIndex>,
        translation: &Vec2,
    ) -> Self {
        Self(
            indexes
                .into_iter()
                .map(|index| (*index, *translation))
                .collect(),
        )
    }

    pub fn invert(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|(index, translate)| (*index, translate.neg()))
                .collect(),
        )
    }

    fn apply_constraints(&mut self, constraints: &Constraints, shape: &mut Shape) {
        let mut connected_translations: HashMap<ShapePointIndex, Vec2> = HashMap::default();
        for (from, transform) in &self.0 {
            if let Some(mut connected_set) = constraints.translation_propagation.get(from).cloned()
            {
                while !connected_set.is_empty() {
                    for to in std::mem::replace(&mut connected_set, HashSet::default()) {
                        if !connected_translations.contains_key(&to) {
                            connected_translations.insert(to, *transform);
                            if let Some(set) = constraints.translation_propagation.get(&to) {
                                connected_set.extend(set);
                            }
                        }
                    }
                }
            }
        }
        self.0.extend(connected_translations);
        let mut positions_visitor = GetPointsPositions::new(
            self.0
                .keys()
                .filter(|index| constraints.point_position_range.contains_key(index))
                .copied()
                .collect(),
        );
        IndexedShapeControlPointsVisitorAdapter(&mut positions_visitor).visit(shape);
        let positions = positions_visitor.into_not_found_and_positions().1;
        let mut translation_factor = f32::ONE;
        for (index, position) in positions {
            if let Some(constraint) = constraints.point_position_range.get(&index) {
                if let Some(translation) = self.0.get(&index) {
                    let clamped_translation = constraint.clamp_translation(*translation, position);
                    if !translation.x.is_zero() {
                        translation_factor =
                            translation_factor.min(clamped_translation.x / translation.x);
                    }
                    if !translation.y.is_zero() {
                        translation_factor =
                            translation_factor.min(clamped_translation.y / translation.y);
                    }
                }
            }
        }
        if translation_factor != f32::ONE {
            for translation in self.0.values_mut() {
                translation.x.mul_assign(translation_factor);
                translation.y.mul_assign(translation_factor);
            }
        }
    }
}

impl ShapeAction for MoveShapePoints {
    fn apply(
        mut self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        self.apply_constraints(constraints, shape);
        IndexedShapeControlPointsVisitorAdapter(self.deref_mut()).visit(shape);
        Box::new(self.invert())
    }

    fn short_name(&self) -> String {
        "Move".into()
    }
}
