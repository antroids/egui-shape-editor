use crate::shape_editor::utils::normalize_rect;
use egui::ahash::HashMap;
use egui::emath::Pos2;
use egui::epaint::{
    CircleShape, CubicBezierShape, EllipseShape, Mesh, PaintCallback, PathShape,
    QuadraticBezierShape, RectShape, Shape, Stroke, TextShape,
};
use egui::Vec2;
use std::ops::{Add, AddAssign, SubAssign};

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
    fn ellipse(&mut self, _index: &mut I, _ellipse: &mut EllipseShape) -> Option<R> {
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
            Shape::Noop => self.none(index),
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
            Shape::Ellipse(ellipse) => self.ellipse(index, ellipse),
        }
    }

    fn visit(&mut self, shape: &mut Shape) -> Option<R>
    where
        Self: Sized,
    {
        puffin_egui::puffin::profile_function!();
        visit_shape(self, shape)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, strum::Display)]
pub enum ShapeType {
    Circle,
    Ellipse,
    LineSegment,
    Path,
    Rect,
    Text,
    Mesh,
    QuadraticBezier,
    CubicBezier,
    Callback,
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct ShapePointIndex {
    pub shape_index: usize,
    pub point_index: usize,
}

impl ShapePointIndex {
    pub fn assign_prev_point(&mut self) {
        self.point_index.sub_assign(1);
    }

    pub fn assign_next_point(&mut self) {
        self.point_index.add_assign(1);
    }

    pub fn assign_next_shape(&mut self) {
        self.shape_index.add_assign(1);
        self.point_index = 0;
    }

    pub fn prev_point(&self) -> Self {
        let mut slf = *self;
        slf.assign_prev_point();
        slf
    }

    pub fn next_point(&self) -> Self {
        let mut slf = *self;
        slf.assign_next_point();
        slf
    }

    pub fn next_shape(&self) -> Self {
        let mut slf = *self;
        slf.assign_next_shape();
        slf
    }
}

impl From<(usize, usize)> for ShapePointIndex {
    fn from(value: (usize, usize)) -> Self {
        Self {
            shape_index: value.0,
            point_index: value.1,
        }
    }
}

pub trait IndexedShapeControlPointsVisitor<R = ()> {
    fn indexed_path_point(
        &mut self,
        _index: ShapePointIndex,
        _point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<R> {
        None
    }
    fn indexed_control_point(
        &mut self,
        _index: ShapePointIndex,
        _control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
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
    fn indexed_ellipse(&mut self, _index: usize, _ellipse: &mut EllipseShape) -> Option<R> {
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
        puffin_egui::puffin::profile_function!();
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
            Shape::Ellipse(ellipse) => self.indexed_ellipse(index, ellipse),
        }
    }
}

#[derive(Default)]
pub struct LastShapePointIndex(Option<ShapePointIndex>);

impl LastShapePointIndex {
    pub fn last_index(shape: &mut Shape) -> Option<ShapePointIndex> {
        let mut visitor = Self::default();
        IndexedShapeControlPointsVisitorAdapter(&mut visitor).visit(shape);
        visitor.0
    }
}

impl IndexedShapeControlPointsVisitor for LastShapePointIndex {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        _point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<()> {
        self.0.replace(index);
        None
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        _control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
        _shape_type: ShapeType,
    ) -> Option<()> {
        self.0.replace(index);
        None
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

impl<'a, R, T: IndexedShapeControlPointsVisitor<R>> ShapeVisitor<R, ShapePointIndex>
    for IndexedShapeControlPointsVisitorAdapter<'a, T>
{
    fn line_segment(
        &mut self,
        index: &mut ShapePointIndex,
        points: &mut [Pos2; 2],
        _stroke: &mut Stroke,
    ) -> Option<R> {
        let result = points.iter_mut().find_map(|point| {
            let result = self
                .0
                .indexed_path_point(*index, point, ShapeType::LineSegment);
            index.assign_next_point();
            result
        });
        index.assign_next_shape();
        result
    }

    fn path(&mut self, index: &mut ShapePointIndex, path: &mut PathShape) -> Option<R> {
        let result = path.points.iter_mut().find_map(|point| {
            let result = self.0.indexed_path_point(*index, point, ShapeType::Path);
            index.assign_next_point();
            result
        });
        index.assign_next_shape();
        result
    }

    fn circle(&mut self, index: &mut ShapePointIndex, circle: &mut CircleShape) -> Option<R> {
        let result = {
            let result = self
                .0
                .indexed_path_point(*index, &mut circle.center, ShapeType::Circle);
            index.assign_next_point();
            result
        }
        .or_else(|| {
            let mut radius_point = circle
                .center
                .add(Vec2::angled(std::f32::consts::TAU / 8.0) * circle.radius);
            let connected = HashMap::from_iter([(index.prev_point(), circle.center)]);
            let result = self.0.indexed_control_point(
                *index,
                &mut radius_point,
                connected,
                ShapeType::Circle,
            );
            circle.radius = radius_point.distance(circle.center);
            result
        });

        index.assign_next_shape();
        result
    }

    fn ellipse(&mut self, index: &mut ShapePointIndex, ellipse: &mut EllipseShape) -> Option<R> {
        let result = {
            let result = self
                .0
                .indexed_path_point(*index, &mut ellipse.center, ShapeType::Ellipse);
            index.assign_next_point();
            result
        }
        .or_else(|| {
            {
                let mut radius_point = ellipse
                    .center
                    .add(Vec2::angled(std::f32::consts::TAU / 8.0) * ellipse.radius.x);
                let connected = HashMap::from_iter([(index.prev_point(), ellipse.center)]);
                let result = self.0.indexed_control_point(
                    *index,
                    &mut radius_point,
                    connected,
                    ShapeType::Circle,
                );
                ellipse.radius.x = radius_point.distance(ellipse.center);
                index.assign_next_point();
                result
            }
            .or_else(|| {
                let mut radius_point = ellipse
                    .center
                    .add(Vec2::angled(std::f32::consts::TAU / 8.0) * ellipse.radius.y);
                let connected = HashMap::from_iter([(index.prev_point(), ellipse.center)]);
                let result = self.0.indexed_control_point(
                    *index,
                    &mut radius_point,
                    connected,
                    ShapeType::Circle,
                );
                ellipse.radius.y = radius_point.distance(ellipse.center);
                result
            })
        });

        index.assign_next_shape();
        result
    }

    fn rect(&mut self, index: &mut ShapePointIndex, rect: &mut RectShape) -> Option<R> {
        let result = {
            let result = self
                .0
                .indexed_path_point(*index, &mut rect.rect.min, ShapeType::Rect);
            index.assign_next_point();
            result
        }
        .or_else(|| {
            let result = self
                .0
                .indexed_path_point(*index, &mut rect.rect.max, ShapeType::Rect);
            result
        });
        rect.rect = normalize_rect(&rect.rect);
        index.assign_next_shape();
        result
    }

    fn text(&mut self, index: &mut ShapePointIndex, text: &mut TextShape) -> Option<R> {
        let result = self
            .0
            .indexed_path_point(*index, &mut text.pos, ShapeType::Text);
        index.assign_next_shape();
        result
    }

    fn mesh(&mut self, index: &mut ShapePointIndex, mesh: &mut Mesh) -> Option<R> {
        let result = mesh.vertices.iter_mut().find_map(|v| {
            let result = self
                .0
                .indexed_path_point(*index, &mut v.pos, ShapeType::Mesh);
            index.assign_next_point();
            result
        });
        index.assign_next_shape();
        result
    }

    fn none(&mut self, index: &mut ShapePointIndex) -> Option<R> {
        index.assign_next_shape();
        None
    }

    fn quadratic_bezier(
        &mut self,
        index: &mut ShapePointIndex,
        b: &mut QuadraticBezierShape,
    ) -> Option<R> {
        let result = {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[0], ShapeType::QuadraticBezier);
            index.assign_next_point();
            result
        }
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.prev_point(), b.points[0]),
                (index.next_point(), b.points[2]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[1],
                connected,
                ShapeType::QuadraticBezier,
            );
            index.assign_next_point();
            result
        })
        .or_else(|| {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[2], ShapeType::QuadraticBezier);
            result
        });
        index.assign_next_shape();
        result
    }

    fn cubic_bezier(&mut self, index: &mut ShapePointIndex, b: &mut CubicBezierShape) -> Option<R> {
        let result = {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[0], ShapeType::CubicBezier);
            index.assign_next_point();
            result
        }
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.prev_point(), b.points[0]),
                (index.next_point(), b.points[2]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[1],
                connected,
                ShapeType::CubicBezier,
            );
            index.assign_next_point();
            result
        })
        .or_else(|| {
            let connected = HashMap::from_iter([
                (index.prev_point(), b.points[1]),
                (index.next_point(), b.points[3]),
            ]);
            let result = self.0.indexed_control_point(
                *index,
                &mut b.points[2],
                connected,
                ShapeType::CubicBezier,
            );
            index.assign_next_point();
            result
        })
        .or_else(|| {
            let result =
                self.0
                    .indexed_path_point(*index, &mut b.points[3], ShapeType::CubicBezier);
            result
        });
        index.assign_next_shape();
        result
    }

    fn paint_callback(
        &mut self,
        index: &mut ShapePointIndex,
        _paint_callback: &mut PaintCallback,
    ) -> Option<R> {
        index.assign_next_shape();
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

impl IndexedShapeControlPointsVisitor<ShapeType> for GetShapeTypeByPointIndex {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        _point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<ShapeType> {
        (index.point_index == self.0).then_some(shape_type)
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        _control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
        shape_type: ShapeType,
    ) -> Option<ShapeType> {
        (index.point_index == self.0).then_some(shape_type)
    }
}

pub fn visit_shape<R, I: Default>(
    visitor: &mut impl ShapeVisitor<R, I>,
    shape: &mut Shape,
) -> Option<R> {
    let mut index = I::default();
    visit_shape_with_index(visitor, shape, &mut index)
}

fn visit_shape_with_index<R, I: Default>(
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
