use crate::shape_editor::index::ShapeControlPointsIndex;
use crate::shape_editor::style;
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitor, IndexedShapeControlPointsVisitorAdapter,
    ShapeControlPointIndex, ShapeType, ShapeVisitor,
};
use egui::ahash::HashMap;
use egui::{Color32, Pos2, Shape, Stroke};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ShapeControlPoint {
    PathPoint {
        position: Pos2,
        shape_index: usize,
    },
    ControlPoint {
        position: Pos2,
        shape_index: usize,
        connected_points: HashMap<usize, Pos2>,
    },
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
    pub control_points: Vec<ShapeControlPoint>,
    pub shapes: HashMap<usize, ShapeType>,
    pub index: ShapeControlPointsIndex,
}

impl ShapeControlPoints {
    pub fn collect(shape: &mut Shape) -> Self {
        let mut slf = Self::default();
        IndexedShapeControlPointsVisitorAdapter(&mut slf).visit(shape);
        slf
    }

    pub fn points_in_radius(&self, pos: Pos2, radius: f32) -> HashMap<usize, ShapeControlPoint> {
        self.index
            .find_points_in_distance(pos, radius)
            .iter()
            .map(|(_, index)| (*index, self.control_points[*index].clone()))
            .collect()
    }

    pub fn connected_bezier_control_point(&self, path_point_index: usize) -> Option<Pos2> {
        self.control_points.iter().find_map(|point| {
            if let ShapeControlPoint::ControlPoint {
                position,
                connected_points,
                ..
            } = point
            {
                connected_points
                    .contains_key(&path_point_index)
                    .then_some(*position)
            } else {
                None
            }
        })
    }

    pub fn by_index(&self, index: usize) -> Option<&ShapeControlPoint> {
        self.control_points.get(index)
    }

    pub fn pos_by_index(&self, index: usize) -> Option<Pos2> {
        self.by_index(index).map(|p| p.position())
    }

    pub fn shape_type_by_control_point(&self, index: usize) -> Option<ShapeType> {
        self.control_points
            .get(index)
            .and_then(|point| self.shapes.get(&point.shape_index()))
            .cloned()
    }
}

impl PartialEq for ShapeControlPoints {
    fn eq(&self, other: &Self) -> bool {
        self.control_points.eq(&other.control_points)
    }
}

impl IntoIterator for ShapeControlPoints {
    type Item = ShapeControlPoint;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.control_points.into_iter()
    }
}

impl IndexedShapeControlPointsVisitor<()> for ShapeControlPoints {
    fn indexed_path_point(
        &mut self,
        index: ShapeControlPointIndex,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<()> {
        self.control_points.push(ShapeControlPoint::PathPoint {
            position: *point,
            shape_index: index.shape_index,
        });
        self.shapes.insert(index.shape_index, shape_type);
        self.index.insert(*point, self.control_points.len() - 1);
        None
    }

    fn indexed_control_point(
        &mut self,
        index: ShapeControlPointIndex,
        control_point: &mut Pos2,
        connected_points: HashMap<usize, Pos2>,
        shape_type: ShapeType,
    ) -> Option<()> {
        self.control_points.push(ShapeControlPoint::ControlPoint {
            position: *control_point,
            shape_index: index.shape_index,
            connected_points,
        });
        self.shapes.insert(index.shape_index, shape_type);
        self.index
            .insert(*control_point, self.control_points.len() - 1);
        None
    }
}
