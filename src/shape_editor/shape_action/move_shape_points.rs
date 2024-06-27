use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitor, IndexedShapeControlPointsVisitorAdapter, ShapePointIndex,
    ShapeType, ShapeVisitor,
};
use egui::ahash::{HashMap, HashSet};
use egui::{Pos2, Shape, Vec2};
use std::ops::{AddAssign, DerefMut, Neg};

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
            if shape_type == ShapeType::Circle {
                self.0.remove(&index.next_point());
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
            if shape_type != ShapeType::Circle || !self.0.contains_key(&index.prev_point()) {
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

    fn apply_translation_propagation(
        &mut self,
        translation_propagation: &HashMap<ShapePointIndex, HashSet<ShapePointIndex>>,
    ) {
        let mut connected_translations: HashMap<ShapePointIndex, Vec2> = HashMap::default();
        for (from, transform) in &self.0 {
            if let Some(mut connected_set) = translation_propagation.get(from).cloned() {
                while !connected_set.is_empty() {
                    for to in std::mem::replace(&mut connected_set, HashSet::default()) {
                        if !connected_translations.contains_key(&to) {
                            connected_translations.insert(to, *transform);
                            if let Some(set) = translation_propagation.get(&to) {
                                connected_set.extend(set);
                            }
                        }
                    }
                }
            }
        }
        self.0.extend(connected_translations);
    }
}

impl ShapeAction for MoveShapePoints {
    fn apply(
        mut self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        self.apply_translation_propagation(&constraints.translation_propagation);
        IndexedShapeControlPointsVisitorAdapter(self.deref_mut()).visit(shape);
        Box::new(self.invert())
    }

    fn short_name(&self) -> String {
        "Move".into()
    }
}
