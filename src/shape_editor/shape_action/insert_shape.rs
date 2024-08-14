use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::replace_shapes::ReplaceShapesVisitor;
use crate::shape_editor::shape_action::{Noop, ShapeAction};
use crate::shape_editor::shape_visitor::count_shapes::CountShapes;
use crate::shape_editor::shape_visitor::indexed_shapes_visitor::IndexedShapesVisitorAdapter;
use crate::shape_editor::shape_visitor::ShapeVisitor;
use egui::epaint::{CubicBezierShape, QuadraticBezierShape};
use egui::{Color32, Pos2, Shape, Stroke};
use std::mem;

#[derive(Clone)]
pub struct InsertShape {
    shape: Option<Shape>,
    replace: Option<usize>,
}

impl InsertShape {
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            shape: Some(shape),
            replace: None,
        }
    }

    pub fn replace_by_noop(index: usize) -> Self {
        Self {
            shape: None,
            replace: Some(index),
        }
    }

    pub fn cubic_bezier_from_two_points(
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

    pub fn quadratic_bezier_from_two_points(
        start_point: Pos2,
        start_point_control: Option<Pos2>,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        let distance = start_point.distance(end_point);
        let start_control_point = start_point_control
            .map(|pos| start_point + (start_point - pos).normalized() * distance / 3.0)
            .unwrap_or(start_point);
        InsertShape::from_shape(
            QuadraticBezierShape::from_points_stroke(
                [start_point, start_control_point, end_point],
                false,
                Color32::TRANSPARENT,
                stroke,
            )
            .into(),
        )
    }
}

impl From<Shape> for InsertShape {
    fn from(value: Shape) -> Self {
        Self::from_shape(value)
    }
}

impl ShapeAction for InsertShape {
    fn apply(
        mut self: Box<Self>,
        shape: &mut Shape,
        _constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        if let Some(replace) = self.replace {
            let mut visitor =
                ReplaceShapesVisitor::single(replace, self.shape.unwrap_or(Shape::Noop));
            IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
            visitor
                .replaced_shapes
                .remove(&replace)
                .map_or(Box::new(Noop), |replaced| {
                    Box::new(Self {
                        shape: Some(replaced),
                        replace: self.replace,
                    })
                })
        } else {
            if !matches!(shape, Shape::Vec(_)) {
                let original = mem::replace(shape, Shape::Noop);
                *shape = Shape::Vec(vec![original]);
            }
            if let Shape::Vec(vec) = shape {
                vec.push(self.shape.take().unwrap_or(Shape::Noop));
            }
            let index = CountShapes::count(shape) - 1;
            Box::new(InsertShape::replace_by_noop(index))
        }
    }

    fn short_name(&self) -> String {
        "Insert Shape".into()
    }
}
