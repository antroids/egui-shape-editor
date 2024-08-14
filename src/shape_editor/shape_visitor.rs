use egui::emath::Pos2;
use egui::epaint::{
    CircleShape, CubicBezierShape, EllipseShape, Mesh, PaintCallback, PathShape,
    QuadraticBezierShape, RectShape, Shape, Stroke, TextShape,
};

use std::ops::{AddAssign, SubAssign};

pub(crate) mod count_shapes;
pub mod get_points_positions;
pub(crate) mod get_shape_type_by_point_index;
pub(crate) mod indexed_shape_control_points_visitor;
pub(crate) mod indexed_shapes_visitor;
pub(crate) mod last_shape_point_index;

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

impl ShapeType {
    pub const MAX_RADIUS_POINTS: u8 = 2;
    pub fn is_center_and_radius_topography(&self) -> bool {
        match self {
            ShapeType::Circle | ShapeType::Ellipse => true,
            _ => false,
        }
    }
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

    pub fn first_point(&self) -> Self {
        let mut slf = *self;
        slf.point_index = 0;
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
