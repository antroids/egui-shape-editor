use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::remove_shape_points::RemoveShapePoints;
use crate::shape_editor::shape_action::{ShapeAction, ShapePoint};
use crate::shape_editor::shape_visitor::indexed_shapes_visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter,
};
use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeVisitor};
use egui::Shape;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct AddShapePoints(pub BTreeMap<usize, BTreeMap<usize, ShapePoint>>);

impl AddShapePoints {
    pub fn single_point(index: ShapePointIndex, point: ShapePoint) -> Self {
        Self(BTreeMap::from_iter([(
            index.shape_index,
            BTreeMap::from_iter([(index.point_index, point)]),
        )]))
    }
}

impl ShapeAction for AddShapePoints {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        _constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        let owned = *self;
        let mut visitor = AddShapePointsVisitor {
            index_to_add: owned.0,
            added: Default::default(),
        };
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Box::new(RemoveShapePoints(visitor.added))
    }

    fn short_name(&self) -> String {
        "Add points".into()
    }
}

struct AddShapePointsVisitor {
    index_to_add: BTreeMap<usize, BTreeMap<usize, ShapePoint>>,
    added: BTreeSet<ShapePointIndex>,
}

impl IndexedShapesVisitor for AddShapePointsVisitor {
    fn indexed_single_shape(&mut self, shape_index: usize, shape: &mut Shape) -> Option<()> {
        if let Some(points_to_add) = self.index_to_add.remove(&shape_index) {
            for (point_index, shape_point) in points_to_add.into_iter().sorted_by_key(|(k, _)| *k) {
                match shape {
                    Shape::Path(path) => {
                        if let ShapePoint::Pos(pos) = shape_point {
                            path.points.insert(point_index, pos);
                            self.added.insert((shape_index, point_index).into());
                        }
                    }
                    Shape::Mesh(mesh) => {
                        if let ShapePoint::Vertex(vertex, vertex_index) = shape_point {
                            mesh.vertices.insert(point_index, vertex);
                            mesh.indices.insert(point_index, vertex_index);
                            self.added.insert((shape_index, point_index).into());
                        }
                    }
                    _ => {}
                }
            }
        }

        self.index_to_add.is_empty().then_some(())
    }
}
