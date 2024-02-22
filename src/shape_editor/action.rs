use crate::shape_editor::visitor::{
    CountShapes, IndexedShapeControlPointsVisitor, ShapePointIndex, ShapeType, ShapeVisitor,
};
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitorAdapter, IndexedShapesVisitor, IndexedShapesVisitorAdapter,
};
use dyn_clone::DynClone;
use egui::ahash::{HashMap, HashSet};
use egui::emath::Pos2;
use egui::epaint::{CircleShape, CubicBezierShape, PathShape, QuadraticBezierShape, Vertex};
use egui::{Color32, Rect, Shape, Stroke, Vec2};
use std::ops::{AddAssign, DerefMut, Neg};

pub trait ShapeAction: DynClone + Send + Sync {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction>;
    fn short_name(&self) -> String;
}

dyn_clone::clone_trait_object!(ShapeAction);

#[derive(Clone)]
pub struct Noop;

impl ShapeAction for Noop {
    fn apply(self: Box<Self>, _shape: &mut Shape) -> Box<dyn ShapeAction> {
        self
    }

    fn short_name(&self) -> String {
        "None".into()
    }
}

#[derive(Clone)]
pub struct MoveShapeControlPoints(HashMap<ShapePointIndex, Vec2>);

impl IndexedShapeControlPointsVisitor for MoveShapeControlPoints {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
            point.add_assign(translation);
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
        _shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
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
}

impl ShapeAction for MoveShapeControlPoints {
    fn apply(mut self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        IndexedShapeControlPointsVisitorAdapter(self.deref_mut()).visit(shape);
        Box::new(self.invert())
    }

    fn short_name(&self) -> String {
        "Move".into()
    }
}

#[derive(Clone)]
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

    pub fn circle_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::Circle(CircleShape::stroke(
            end_point,
            start_point.distance(end_point),
            stroke,
        ))
        .into()
    }

    pub fn line_segment_from_two_points(
        start_point: Pos2,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        Shape::line_segment([start_point, end_point], stroke).into()
    }

    pub fn path_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::Path(PathShape::line(vec![start_point, end_point], stroke)).into()
    }

    pub fn rect_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::rect_stroke(Rect::from_two_pos(start_point, end_point), 0.0, stroke).into()
    }
}

impl From<Shape> for InsertShape {
    fn from(value: Shape) -> Self {
        Self::from_shape(value)
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
    fn apply(mut self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        if self.replace.is_some() {
            let replaced = IndexedShapesVisitorAdapter(self.deref_mut()).visit(shape);
            replaced.map_or(Box::new(Noop), |replaced| {
                Box::new(Self {
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
            Box::new(Self {
                shape: None,
                replace: Some(index),
            })
        }
    }

    fn short_name(&self) -> String {
        "Insert Shape".into()
    }
}

#[derive(Clone)]
pub struct Combined {
    short_name: String,
    actions: Vec<Box<dyn ShapeAction>>,
}

impl Combined {
    pub fn new(short_name: String, actions: Vec<Box<dyn ShapeAction>>) -> Self {
        Self {
            short_name,
            actions,
        }
    }

    pub fn from_actions(actions: Vec<Box<dyn ShapeAction>>) -> Self {
        Self {
            short_name: "Combined action".to_string(),
            actions,
        }
    }
}

impl ShapeAction for Combined {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        let owned = *self;
        let inverted: Vec<Box<dyn ShapeAction>> = owned
            .actions
            .into_iter()
            .map(|action| action.apply(shape))
            .rev()
            .collect();
        Box::new(Self::new(format!("Undo {}", owned.short_name), inverted))
    }

    fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

#[derive(Clone, Copy)]
pub enum ShapePoint {
    Pos(Pos2),
    Vertex(Vertex, u32),
}

#[derive(Clone)]
pub struct RemoveShapePoints(HashSet<usize>);

impl ShapeAction for RemoveShapePoints {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        todo!()
    }

    fn short_name(&self) -> String {
        "Remove points".into()
    }
}

#[derive(Clone)]
struct RemoveShapePointsVisitor {
    index_to_remove: HashSet<usize>,
    removed: HashMap<usize, HashMap<usize, ShapePoint>>,
}

impl ShapeVisitor<(), ShapePointIndex> for RemoveShapePointsVisitor {
    fn single_shape(&mut self, shape: &mut Shape, index: &mut ShapePointIndex) -> Option<()> {
        match shape {
            Shape::Circle(_) => index.point_index.add_assign(2),
            Shape::LineSegment { .. } => index.point_index.add_assign(2),
            Shape::Path(path) => {
                let count = path.points.len();
                for i in (index.point_index..index.point_index + count).rev() {
                    if let Some(i) = self.index_to_remove.take(&i) {
                        self.removed
                            .entry(index.shape_index)
                            .or_default()
                            .insert(i, ShapePoint::Pos(path.points.remove(i)));
                    }
                    if self.index_to_remove.is_empty() {
                        break;
                    }
                }
                index.point_index.add_assign(count);
            }
            Shape::Rect(_) => index.point_index.add_assign(2),
            Shape::Text(_) => index.point_index.add_assign(1),
            Shape::Mesh(mesh) => {
                let count = mesh.vertices.len();
                for i in (index.point_index..index.point_index + count).rev() {
                    if let Some(i) = self.index_to_remove.take(&i) {
                        self.removed.entry(index.shape_index).or_default().insert(
                            i,
                            ShapePoint::Vertex(mesh.vertices.remove(i), mesh.indices.remove(i)),
                        );
                    }
                    if self.index_to_remove.is_empty() {
                        break;
                    }
                }
                index.point_index.add_assign(count);
            }
            Shape::QuadraticBezier(_) => index.point_index.add_assign(3),
            Shape::CubicBezier(_) => index.point_index.add_assign(4),
            _ => {}
        }
        index.shape_index.add_assign(1);
        self.index_to_remove.is_empty().then_some(())
    }
}

struct AddShapePoints {
    index_to_add: HashMap<usize, HashMap<usize, ShapePoint>>,
    added: HashSet<usize>,
}
