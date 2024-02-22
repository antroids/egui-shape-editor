use crate::shape_editor::index::{ShapeControlPointsIndex, SnapPoint};
use crate::shape_editor::style;
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitor, IndexedShapeControlPointsVisitorAdapter, ShapePointIndex,
    ShapeType, ShapeVisitor,
};
use egui::ahash::{HashMap, HashSet};
use egui::{Color32, Pos2, Rect, Shape, Stroke};
use std::collections::hash_map::Iter;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ShapeControlPoint {
    PathPoint {
        position: Pos2,
        shape_index: usize,
    },
    ControlPoint {
        position: Pos2,
        shape_index: usize,
        connected_points: HashMap<ShapePointIndex, Pos2>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PointRemovingStrategy {
    ControlPoint,
    Shape,
    None,
}

impl ShapeControlPoint {
    pub fn position(&self) -> Pos2 {
        match self {
            ShapeControlPoint::PathPoint { position, .. } => *position,
            ShapeControlPoint::ControlPoint { position, .. } => *position,
        }
    }

    pub fn shape_index(&self) -> usize {
        match self {
            ShapeControlPoint::PathPoint { shape_index, .. } => *shape_index,
            ShapeControlPoint::ControlPoint { shape_index, .. } => *shape_index,
        }
    }

    fn stroke(&self, style: &dyn style::Style) -> Stroke {
        match self {
            ShapeControlPoint::PathPoint { .. } => style.path_point_stroke(),
            ShapeControlPoint::ControlPoint { .. } => style.control_point_stroke(),
        }
    }

    pub fn to_shape(&self, hovered: bool, selected: bool, style: &dyn style::Style) -> Shape {
        let stroke = self.stroke(style);
        let radius = style.control_point_radius();
        let pos = self.position();
        let mut vec_shape = if let Self::ControlPoint {
            position,
            connected_points,
            ..
        } = self
        {
            connected_points
                .values()
                .map(|connected_pos| Shape::LineSegment {
                    points: [*position, *connected_pos],
                    stroke: Stroke::new(1.0, Color32::GRAY),
                })
                .collect()
        } else {
            Vec::default()
        };

        vec_shape.push(Shape::circle_stroke(pos, radius, stroke));
        if hovered {
            vec_shape.push(Shape::circle_filled(
                pos,
                radius,
                stroke.color.linear_multiply(0.5),
            ));
        }
        if selected {
            vec_shape.push(Shape::circle_stroke(pos, radius + 2.0, stroke))
        }

        Shape::Vec(vec_shape)
    }
}

#[derive(Default, Clone, Debug)]
pub struct ShapeControlPoints {
    control_points: HashMap<ShapePointIndex, ShapeControlPoint>,
    shapes: HashMap<usize, ShapeType>,
    index: ShapeControlPointsIndex,
}

impl ShapeControlPoints {
    pub fn collect(shape: &mut Shape) -> Self {
        let mut slf = Self::default();
        IndexedShapeControlPointsVisitorAdapter(&mut slf).visit(shape);
        slf
    }

    pub fn snap_point(
        &self,
        pos: Pos2,
        max_distance: f32,
        ignore: &HashSet<ShapePointIndex>,
    ) -> SnapPoint {
        self.index.snap_point(pos, max_distance, ignore)
    }

    pub fn points_in_radius(
        &self,
        pos: Pos2,
        radius: f32,
    ) -> HashMap<ShapePointIndex, ShapeControlPoint> {
        self.index
            .find_points_in_distance(pos, radius)
            .iter()
            .map(|(_, index)| (*index, self.control_points[index].clone()))
            .collect()
    }

    pub fn find_points_in_rect(&self, rect: &Rect) -> Vec<(Pos2, ShapePointIndex)> {
        self.index.find_points_in_rect(rect)
    }

    pub fn connected_bezier_control_point(
        &self,
        path_point_index: &ShapePointIndex,
    ) -> Option<Pos2> {
        self.control_points.values().find_map(|point| {
            if let ShapeControlPoint::ControlPoint {
                position,
                connected_points,
                ..
            } = point
            {
                connected_points
                    .contains_key(path_point_index)
                    .then_some(*position)
            } else {
                None
            }
        })
    }

    pub fn by_index(&self, index: &ShapePointIndex) -> Option<&ShapeControlPoint> {
        self.control_points.get(index)
    }

    pub fn by_shape_index(&self, shape_index: usize) -> HashSet<usize> {
        self.control_points
            .values()
            .enumerate()
            .filter_map(|(index, point)| (point.shape_index() == shape_index).then_some(index))
            .collect()
    }

    pub fn pos_by_index(&self, index: &ShapePointIndex) -> Option<Pos2> {
        self.by_index(index).map(|p| p.position())
    }

    pub fn shape_type_by_control_point(&self, index: &ShapePointIndex) -> Option<ShapeType> {
        self.control_points
            .get(index)
            .and_then(|point| self.shapes.get(&point.shape_index()))
            .cloned()
    }

    pub fn shape_by_index(&self, shape_index: usize) -> Option<ShapeType> {
        self.shapes.get(&shape_index).cloned()
    }

    pub fn iter(&self) -> Iter<ShapePointIndex, ShapeControlPoint> {
        self.control_points.iter()
    }
}

impl PartialEq for ShapeControlPoints {
    fn eq(&self, other: &Self) -> bool {
        self.control_points.eq(&other.control_points)
    }
}

impl IndexedShapeControlPointsVisitor<()> for ShapeControlPoints {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<()> {
        self.control_points.insert(
            index,
            ShapeControlPoint::PathPoint {
                position: *point,
                shape_index: index.shape_index,
            },
        );
        self.shapes.insert(index.shape_index, shape_type);
        self.index.insert(*point, index);
        None
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        control_point: &mut Pos2,
        connected_points: HashMap<ShapePointIndex, Pos2>,
        shape_type: ShapeType,
    ) -> Option<()> {
        self.control_points.insert(
            index,
            ShapeControlPoint::ControlPoint {
                position: *control_point,
                shape_index: index.shape_index,
                connected_points,
            },
        );
        self.shapes.insert(index.shape_index, shape_type);
        self.index.insert(*control_point, index);
        None
    }
}