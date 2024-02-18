use crate::shape_editor::visitor::{
    CountShapes, IndexedShapeControlPointsVisitor, ShapeControlPointIndex, ShapeType, ShapeVisitor,
};
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitorAdapter, IndexedShapesVisitor, IndexedShapesVisitorAdapter,
};
use egui::emath::Pos2;
use egui::ahash::HashMap;
use egui::epaint::CubicBezierShape;
use egui::{Color32, Shape, Stroke, Vec2};
use std::ops::{AddAssign, Neg};

#[derive(Clone, Debug)]
pub enum Action {
    MoveShapeControlPoints(MoveShapeControlPoints),
    InsertShape(InsertShape),
    Noop,
}

impl ShapeAction for Action {
    fn apply(self, shape: &mut Shape) -> Self {
        match self {
            Action::MoveShapeControlPoints(a) => a.apply(shape),
            Action::InsertShape(a) => a.apply(shape),
            Action::Noop => Action::Noop,
        }
    }
}

pub trait ShapeAction {
    fn apply(self, shape: &mut Shape) -> Action;
}

#[derive(Clone, Debug)]
pub struct MoveShapeControlPoints(HashMap<usize, Vec2>);

impl IndexedShapeControlPointsVisitor for MoveShapeControlPoints {
    fn indexed_path_point(
        &mut self,
        index: ShapeControlPointIndex,
        point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index.point_index) {
            point.add_assign(translation);
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }

    fn indexed_bezier_control_point(
        &mut self,
        index: ShapeControlPointIndex,
        control_point: &mut Pos2,
        _connected_points: HashMap<usize, Pos2>,
        _shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index.point_index) {
            control_point.add_assign(translation);
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }
}
impl MoveShapeControlPoints {
    pub fn from_index_and_translation<'a>(
        indexes: impl IntoIterator<Item = &'a usize>,
        translation: &Vec2,
    ) -> Self {
        Self(
            indexes
                .into_iter()
                .map(|index| (*index, translation.clone()))
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
}

impl ShapeAction for MoveShapeControlPoints {
    fn apply(mut self, shape: &mut Shape) -> Action {
        IndexedShapeControlPointsVisitorAdapter(&mut self).visit(shape);
        Action::MoveShapeControlPoints(self.invert())
    }
}

#[derive(Clone, Debug)]
pub struct InsertShape {
    pub(crate) shape: Option<Shape>,
    pub(crate) replace: Option<usize>,
}

impl InsertShape {
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            shape: Some(shape),
            replace: None,
        }
    }

    pub fn cubic_bezier_by_two_points(
        start_point: Pos2,
        start_point_control: Option<Pos2>,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        let distance = start_point.distance(end_point);
        let start_control_point = start_point_control
            .map(|pos| start_point + (start_point - pos).normalized() * distance / 3.0)
            .unwrap_or(start_point);
        let end_control_point =
            end_point - (end_point - start_control_point).normalized() * distance / 3.0;
        InsertShape::from_shape(
            CubicBezierShape::from_points_stroke(
                [
                    start_point,
                    start_control_point,
                    end_control_point,
                    end_point,
                ],
                false,
                Color32::TRANSPARENT,
                stroke,
            )
            .into(),
        )
    }
}

impl IndexedShapesVisitor<Shape> for InsertShape {
    fn indexed_single_shape(&mut self, index: usize, shape: &mut Shape) -> Option<Shape> {
        if self.replace.is_some_and(|i| i == index) {
            Some(std::mem::replace(
                shape,
                self.shape.take().unwrap_or(Shape::Noop),
            ))
        } else {
            None
        }
    }
}

impl ShapeAction for InsertShape {
    fn apply(mut self, shape: &mut Shape) -> Action {
        if self.replace.is_some() {
            let replaced = IndexedShapesVisitorAdapter(&mut self).visit(shape);
            replaced.map_or(Action::Noop, |replaced| {
                Action::InsertShape(Self {
                    shape: Some(replaced),
                    replace: self.replace,
                })
            })
        } else {
            if !matches!(shape, Shape::Vec(_)) {
                let original = std::mem::replace(shape, Shape::Noop);
                *shape = Shape::Vec(vec![original]);
            }
            if let Shape::Vec(vec) = shape {
                vec.push(self.shape.take().unwrap_or(Shape::Noop));
            }
            let index = CountShapes::count(shape) - 1;
            Action::InsertShape(InsertShape {
                shape: None,
                replace: Some(index),
            })
        }
    }
}
