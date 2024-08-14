use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType, ShapeVisitor};
use crate::shape_editor::utils::normalize_rect;
use egui::ahash::HashMap;
use egui::epaint::{
    CircleShape, CubicBezierShape, EllipseShape, PathShape, QuadraticBezierShape, RectShape,
    TextShape,
};
use egui::{Mesh, PaintCallback, Pos2, Stroke, Vec2};
use std::ops::Add;

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

pub struct IndexedShapeControlPointsVisitorAdapter<'a, T>(pub &'a mut T);

impl<'a, T> IndexedShapeControlPointsVisitorAdapter<'a, T> {
    fn handle_indexed_path_point_and_advance<R>(
        &mut self,
        index: &mut ShapePointIndex,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<R>
    where
        T: IndexedShapeControlPointsVisitor<R>,
    {
        let result = self.0.indexed_path_point(*index, point, shape_type);
        index.assign_next_point();
        result
    }

    fn handle_indexed_control_point_and_advance<R>(
        &mut self,
        index: &mut ShapePointIndex,
        connected: impl IntoIterator<Item = (ShapePointIndex, Pos2)>,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<R>
    where
        T: IndexedShapeControlPointsVisitor<R>,
    {
        let result = self.0.indexed_control_point(
            *index,
            point,
            connected.into_iter().collect(),
            shape_type,
        );
        index.assign_next_point();
        result
    }

    fn advance_shape<R>(index: &mut ShapePointIndex, result: Option<R>) -> Option<R> {
        index.assign_next_shape();
        result
    }
}

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
            self.handle_indexed_path_point_and_advance(index, point, ShapeType::LineSegment)
        });
        Self::advance_shape(index, result)
    }

    fn path(&mut self, index: &mut ShapePointIndex, path: &mut PathShape) -> Option<R> {
        let result = path.points.iter_mut().find_map(|point| {
            self.handle_indexed_path_point_and_advance(index, point, ShapeType::Path)
        });
        Self::advance_shape(index, result)
    }

    fn circle(&mut self, index: &mut ShapePointIndex, circle: &mut CircleShape) -> Option<R> {
        let result = self
            .handle_indexed_path_point_and_advance(index, &mut circle.center, ShapeType::Circle)
            .or_else(|| {
                let mut radius_point = circle.center.add(Vec2::RIGHT * circle.radius);
                let result = self.handle_indexed_control_point_and_advance(
                    index,
                    [(index.first_point(), circle.center)],
                    &mut radius_point,
                    ShapeType::Circle,
                );
                circle.radius = radius_point.distance(circle.center);
                result
            });

        Self::advance_shape(index, result)
    }

    fn ellipse(&mut self, index: &mut ShapePointIndex, ellipse: &mut EllipseShape) -> Option<R> {
        let result = self
            .handle_indexed_path_point_and_advance(index, &mut ellipse.center, ShapeType::Ellipse)
            .or_else(|| {
                let mut radius_point = ellipse.center.add(Vec2::RIGHT * ellipse.radius.x);
                let result = self.handle_indexed_control_point_and_advance(
                    index,
                    [(index.first_point(), ellipse.center)],
                    &mut radius_point,
                    ShapeType::Ellipse,
                );
                ellipse.radius.x = radius_point.distance(ellipse.center);
                result
            })
            .or_else(|| {
                let mut radius_point = ellipse.center.add(Vec2::DOWN * ellipse.radius.y);
                let result = self.handle_indexed_control_point_and_advance(
                    index,
                    [(index.first_point(), ellipse.center)],
                    &mut radius_point,
                    ShapeType::Ellipse,
                );
                ellipse.radius.y = radius_point.distance(ellipse.center);
                result
            });

        Self::advance_shape(index, result)
    }

    fn rect(&mut self, index: &mut ShapePointIndex, rect: &mut RectShape) -> Option<R> {
        let result = self
            .handle_indexed_path_point_and_advance(index, &mut rect.rect.min, ShapeType::Rect)
            .or_else(|| {
                self.handle_indexed_path_point_and_advance(
                    index,
                    &mut rect.rect.max,
                    ShapeType::Rect,
                )
            });
        rect.rect = normalize_rect(&rect.rect);
        Self::advance_shape(index, result)
    }

    fn text(&mut self, index: &mut ShapePointIndex, text: &mut TextShape) -> Option<R> {
        let result =
            self.handle_indexed_path_point_and_advance(index, &mut text.pos, ShapeType::Text);
        Self::advance_shape(index, result)
    }

    fn mesh(&mut self, index: &mut ShapePointIndex, mesh: &mut Mesh) -> Option<R> {
        let result = mesh.vertices.iter_mut().find_map(|v| {
            self.handle_indexed_path_point_and_advance(index, &mut v.pos, ShapeType::Mesh)
        });
        Self::advance_shape(index, result)
    }

    fn none(&mut self, index: &mut ShapePointIndex) -> Option<R> {
        Self::advance_shape(index, None)
    }

    fn quadratic_bezier(
        &mut self,
        index: &mut ShapePointIndex,
        b: &mut QuadraticBezierShape,
    ) -> Option<R> {
        let result = self
            .handle_indexed_path_point_and_advance(
                index,
                &mut b.points[0],
                ShapeType::QuadraticBezier,
            )
            .or_else(|| {
                self.handle_indexed_control_point_and_advance(
                    index,
                    [
                        (index.prev_point(), b.points[0]),
                        (index.next_point(), b.points[2]),
                    ],
                    &mut b.points[1],
                    ShapeType::QuadraticBezier,
                )
            })
            .or_else(|| {
                self.handle_indexed_path_point_and_advance(
                    index,
                    &mut b.points[2],
                    ShapeType::QuadraticBezier,
                )
            });
        Self::advance_shape(index, result)
    }

    fn cubic_bezier(&mut self, index: &mut ShapePointIndex, b: &mut CubicBezierShape) -> Option<R> {
        let result = self
            .handle_indexed_path_point_and_advance(index, &mut b.points[0], ShapeType::CubicBezier)
            .or_else(|| {
                self.handle_indexed_control_point_and_advance(
                    index,
                    [
                        (index.prev_point(), b.points[0]),
                        (index.next_point(), b.points[2]),
                    ],
                    &mut b.points[1],
                    ShapeType::CubicBezier,
                )
            })
            .or_else(|| {
                self.handle_indexed_control_point_and_advance(
                    index,
                    [
                        (index.prev_point(), b.points[1]),
                        (index.next_point(), b.points[3]),
                    ],
                    &mut b.points[2],
                    ShapeType::CubicBezier,
                )
            })
            .or_else(|| {
                self.handle_indexed_path_point_and_advance(
                    index,
                    &mut b.points[3],
                    ShapeType::CubicBezier,
                )
            });
        Self::advance_shape(index, result)
    }

    fn paint_callback(
        &mut self,
        index: &mut ShapePointIndex,
        _paint_callback: &mut PaintCallback,
    ) -> Option<R> {
        Self::advance_shape(index, None)
    }
}
