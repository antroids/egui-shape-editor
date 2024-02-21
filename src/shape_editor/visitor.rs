use egui::ahash::HashMap;
use egui::emath::Pos2;
use egui::epaint::{
    CircleShape, CubicBezierShape, Mesh, PaintCallback, PathShape, QuadraticBezierShape, RectShape,
    Shape, Stroke, TextShape,
};
use egui::Vec2;
use std::ops::{Add, AddAssign};

pub trait ShapeVisitor<R = (), I: Default = usize> {
    fn line_segment(
        &mut self,
        _index: &mut I,
        _points: &mut [Pos2; 2],
        _stroke: &mut Stroke,
    ) -> Option<R> {
        None
    }
    fn path(&mut self, _index: &mut I, _path: &mut PathShape) -> Option<R> {
        None
    }
    fn circle(&mut self, _index: &mut I, _circle: &mut CircleShape) -> Option<R> {
        None
    }
    fn rect(&mut self, _index: &mut I, _rect: &mut RectShape) -> Option<R> {
        None
    }
    fn text(&mut self, _index: &mut I, _text: &mut TextShape) -> Option<R> {
        None
    }
    fn mesh(&mut self, _index: &mut I, _mesh: &mut Mesh) -> Option<R> {
        None
    }
    fn none(&mut self, _index: &mut I) -> Option<R> {
        None
    }
    fn quadratic_bezier(
        &mut self,
        _index: &mut I,
        _bezier: &mut QuadraticBezierShape,
    ) -> Option<R> {
        None
    }
    fn cubic_bezier(&mut self, _index: &mut I, _cubic_bezier: &mut CubicBezierShape) -> Option<R> {
        None
    }
    fn paint_callback(&mut self, _index: &mut I, _paint_callback: &mut PaintCallback) -> Option<R> {
        None
    }

    fn single_shape(&mut self, shape: &mut Shape, index: &mut I) -> Option<R> {
        match shape {
            Shape::Noop => None,
            Shape::Vec(_) => None,
            Shape::Circle(circle) => self.circle(index, circle),
            Shape::LineSegment { points, stroke } => self.line_segment(index, points, stroke),
            Shape::Path(path) => self.path(index, path),
            Shape::Rect(rect) => self.rect(index, rect),
            Shape::Text(text) => self.text(index, text),
            Shape::Mesh(mesh) => self.mesh(index, mesh),
            Shape::QuadraticBezier(qb) => self.quadratic_bezier(index, qb),
            Shape::CubicBezier(cb) => self.cubic_bezier(index, cb),
            Shape::Callback(callback) => self.paint_callback(index, callback),
        }
    }

    fn visit(&mut self, shape: &mut Shape) -> Option<R>
    where
        Self: Sized,
    {
        visit_shape(self, shape)
    }
}

pub fn visit_shape<R, I: Default>(
    visitor: &mut impl ShapeVisitor<R, I>,
    shape: &mut Shape,
) -> Option<R> {
    let mut index = I::default();
    visit_shape_with_index(visitor, shape, &mut index)
}

pub(crate) fn visit_shape_with_index<R, I: Default>(
    visitor: &mut impl ShapeVisitor<R, I>,
    shape: &mut Shape,
    index: &mut I,
) -> Option<R> {
    match shape {
        Shape::Vec(vec) => vec
            .iter_mut()
            .find_map(|shape| visit_shape_with_index(visitor, shape, index)),
        single_shape => visitor.single_shape(single_shape, index),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShapeType {
    Circle,
    LineSegment,
    Path,
    Rect,
    Text,
    Mesh,
    QuadraticBezier,
    CubicBezier,
    Callback,
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct ShapeControlPointIndex {
    pub shape_index: usize,
    pub point_index: usize,
}

pub trait IndexedShapeControlPointsVisitor<R = ()> {
    fn indexed_path_point(
        &mut self,
        _index: ShapeControlPointIndex,
        _point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<R> {
        None
    }
    fn indexed_control_point(
        &mut self,
        _index: ShapeControlPointIndex,
        _control_point: &mut Pos2,
        _connected_points: HashMap<usize, Pos2>,
        _shape_type: ShapeType,
    ) -> Option<R> {
        None
    }
}

pub trait IndexedShapesVisitor<R = ()> {
    fn indexed_line_segment(
        &mut self,
        _index: usize,
        _points: &mut [Pos2; 2],
        _stroke: &mut Stroke,
    ) -> Option<R> {
        None
    }
    fn indexed_path(&mut self, _index: usize, _path: &mut PathShape) -> Option<R> {
        None
    }
    fn indexed_circle(&mut self, _index: usize, _circle: &mut CircleShape) -> Option<R> {
        None
    }
    fn indexed_rect(&mut self, _index: usize, _rect: &mut RectShape) -> Option<R> {
        None
    }
    fn indexed_text(&mut self, _index: usize, _text: &mut TextShape) -> Option<R> {
        None
    }
    fn indexed_mesh(&mut self, _index: usize, _mesh: &mut Mesh) -> Option<R> {
        None
    }
    fn indexed_none(&mut self, _index: usize) -> Option<R> {
        None
    }
    fn indexed_quadratic_bezier(
        &mut self,
        _index: usize,
        _bezier: &mut QuadraticBezierShape,
    ) -> Option<R> {
        None
    }
    fn indexed_cubic_bezier(
        &mut self,
        _index: usize,
        _cubic_bezier: &mut CubicBezierShape,
    ) -> Option<R> {
        None
    }
    fn indexed_paint_callback(
        &mut self,
        _index: usize,
        _paint_callback: &mut PaintCallback,
    ) -> Option<R> {
        None
    }

    fn indexed_single_shape(&mut self, index: usize, shape: &mut Shape) -> Option<R> {
        match shape {
            Shape::Noop => self.indexed_none(index),
            Shape::Vec(_) => None,
            Shape::Circle(circle) => self.indexed_circle(index, circle),
            Shape::LineSegment { points, stroke } => {
                self.indexed_line_segment(index, points, stroke)
            }
            Shape::Path(path) => self.indexed_path(index, path),
            Shape::Rect(rect) => self.indexed_rect(index, rect),
            Shape::Text(text) => self.indexed_text(index, text),
            Shape::Mesh(mesh) => self.indexed_mesh(index, mesh),
            Shape::QuadraticBezier(qb) => self.indexed_quadratic_bezier(index, qb),
            Shape::CubicBezier(cb) => self.indexed_cubic_bezier(index, cb),
            Shape::Callback(callback) => self.indexed_paint_callback(index, callback),
        }
    }
}

#[derive(Clone)]
pub struct CountShapeControlPoints;

impl IndexedShapeControlPointsVisitor for CountShapeControlPoints {}

impl CountShapeControlPoints {
    pub fn count(shape: &mut Shape) -> usize {
        let mut count = ShapeControlPointIndex::default();
        visit_shape_with_index(
            &mut IndexedShapeControlPointsVisitorAdapter(&mut Self),
            shape,
            &mut count,
        );
        count.point_index
    }

    pub fn last_index(shape: &mut Shape) -> Option<usize> {
        let count = Self::count(shape);
        if count > 0 {
            Some(count - 1)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct CountShapes;

impl IndexedShapesVisitor for CountShapes {}

impl CountShapes {
    pub fn count(shape: &mut Shape) -> usize {
        let mut count = 0usize;
        visit_shape_with_index(
            &mut IndexedShapesVisitorAdapter(&mut Self),
            shape,
            &mut count,
        );
        count
    }
}

pub struct IndexedShapeControlPointsVisitorAdapter<'a, T>(pub &'a mut T);

impl<'a, R, T: IndexedShapeControlPointsVisitor<R>> ShapeVisitor<R, ShapeControlPointIndex>
    for IndexedShapeControlPointsVisitorAdapter<'a, T>
{
    fn line_segment(
        &mut self,
        index: &mut ShapeControlPointIndex,
        points: &mut [Pos2; 2],
        _stroke: &mut Stroke,
    ) -> Option<R> {
        let result = points.iter_mut().find_map(|point| {
            let result = self
                .0
                .indexed_path_point(*index, point, ShapeType::LineSegment);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn path(&mut self, index: &mut ShapeControlPointIndex, path: &mut PathShape) -> Option<R> {
        let result = path.points.iter_mut().find_map(|point| {
            let result = self.0.indexed_path_point(*index, point, ShapeType::Path);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn circle(
        &mut self,
        index: &mut ShapeControlPointIndex,
        circle: &mut CircleShape,
    ) -> Option<R> {
        let result = {
            let result = self
                .0
                .indexed_path_point(*index, &mut circle.center, ShapeType::Circle);
            index.point_index.add_assign(1);
            result
        }
        .or_else(|| {
            let mut radius_point = circle
                .center
                .add(Vec2::angled(std::f32::consts::TAU / 8.0) * circle.radius);
            let connected = HashMap::from_iter([(index.point_index - 1, circle.center)]);
            let result = self.0.indexed_control_point(
                *index,
                &mut radius_point,
                connected,
                ShapeType::Circle,
            );
            circle.radius = radius_point.distance(circle.center);
            index.point_index.add_assign(1);
            result
        });

        index.shape_index.add_assign(1);
        result
    }

    fn rect(&mut self, index: &mut ShapeControlPointIndex, rect: &mut RectShape) -> Option<R> {
        let result = {
            let result = self
                .0
                .indexed_path_point(*index, &mut rect.rect.min, ShapeType::Rect);
            index.point_index.add_assign(1);
            result
        }
        .or_else(|| {
            let result = self
                .0
                .indexed_path_point(*index, &mut rect.rect.max, ShapeType::Rect);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn text(&mut self, index: &mut ShapeControlPointIndex, text: &mut TextShape) -> Option<R> {
        let result = self
            .0
            .indexed_path_point(*index, &mut text.pos, ShapeType::Text);
        index.point_index.add_assign(1);
        index.shape_index.add_assign(1);
        result
    }

    fn mesh(&mut self, index: &mut ShapeControlPointIndex, mesh: &mut Mesh) -> Option<R> {
        let result = mesh.vertices.iter_mut().find_map(|v| {
            let result = self
                .0
                .indexed_path_point(*index, &mut v.pos, ShapeType::Mesh);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn none(&mut self, index: &mut ShapeControlPointIndex) -> Option<R> {
        index.shape_index.add_assign(1);
        None
    }

    fn quadratic_bezier(
        &mut self,
        index: &mut ShapeControlPointIndex,
        b: &mut QuadraticBezierShape,
    ) -> Option<R> {
        let result = {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[0], ShapeType::QuadraticBezier);
            index.point_index.add_assign(1);
            result
        }
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.point_index - 1, b.points[0]),
                (index.point_index + 1, b.points[2]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[1],
                connected,
                ShapeType::QuadraticBezier,
            );
            index.point_index.add_assign(1);
            result
        })
        .or_else(|| {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[2], ShapeType::QuadraticBezier);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn cubic_bezier(
        &mut self,
        index: &mut ShapeControlPointIndex,
        b: &mut CubicBezierShape,
    ) -> Option<R> {
        let result = {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[0], ShapeType::CubicBezier);
            index.point_index.add_assign(1);
            result
        }
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.point_index - 1, b.points[0]),
                (index.point_index + 1, b.points[2]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[1],
                connected,
                ShapeType::CubicBezier,
            );
            index.point_index.add_assign(1);
            result
        })
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.point_index - 1, b.points[1]),
                (index.point_index + 1, b.points[3]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[2],
                connected,
                ShapeType::CubicBezier,
            );
            index.point_index.add_assign(1);
            result
        })
        .or_else(|| {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[3], ShapeType::CubicBezier);
            index.point_index.add_assign(1);
            result
        });
        index.shape_index.add_assign(1);
        result
    }

    fn paint_callback(
        &mut self,
        index: &mut ShapeControlPointIndex,
        _paint_callback: &mut PaintCallback,
    ) -> Option<R> {
        index.shape_index.add_assign(1);
        None
    }
}

pub struct IndexedShapesVisitorAdapter<'a, T>(pub &'a mut T);
impl<'a, R, T: IndexedShapesVisitor<R>> ShapeVisitor<R> for IndexedShapesVisitorAdapter<'a, T> {
    fn single_shape(&mut self, shape: &mut Shape, index: &mut usize) -> Option<R> {
        let result = self.0.indexed_single_shape(*index, shape);
        index.add_assign(1);
        result
    }
}

pub struct GetShapeTypeByPointIndex(usize);

impl GetShapeTypeByPointIndex {
    pub fn shape_type(shape: &mut Shape, point_index: usize) -> Option<ShapeType> {
        IndexedShapeControlPointsVisitorAdapter(&mut Self(point_index)).visit(shape)
    }
}

impl IndexedShapeControlPointsVisitor<ShapeType> for GetShapeTypeByPointIndex {
    fn indexed_path_point(
        &mut self,
        index: ShapeControlPointIndex,
        _point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<ShapeType> {
        (index.point_index == self.0).then_some(shape_type)
    }

    fn indexed_control_point(
        &mut self,
        index: ShapeControlPointIndex,
        _control_point: &mut Pos2,
        _connected_points: HashMap<usize, Pos2>,
        shape_type: ShapeType,
    ) -> Option<ShapeType> {
        (index.point_index == self.0).then_some(shape_type)
    }
}
