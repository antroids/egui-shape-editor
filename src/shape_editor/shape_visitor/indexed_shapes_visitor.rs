use crate::shape_editor::shape_visitor::ShapeVisitor;
use egui::epaint::{
    CircleShape, CubicBezierShape, EllipseShape, PathShape, QuadraticBezierShape, RectShape,
    TextShape,
};
use egui::{Mesh, PaintCallback, Pos2, Shape, Stroke};
use std::ops::AddAssign;

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

pub struct IndexedShapesVisitorAdapter<'a, T>(pub &'a mut T);

impl<'a, R, T: IndexedShapesVisitor<R>> ShapeVisitor<R> for IndexedShapesVisitorAdapter<'a, T> {
    fn single_shape(&mut self, shape: &mut Shape, index: &mut usize) -> Option<R> {
        let result = self.0.indexed_single_shape(*index, shape);
        index.add_assign(1);
        result
    }
}
