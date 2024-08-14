use crate::shape_editor::shape_visitor::indexed_shape_control_points_visitor::IndexedShapeControlPointsVisitor;
use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType};
use egui::ahash::{HashMap, HashMapExt, HashSet};
use egui::Pos2;

#[derive(Clone)]
pub struct GetPointsPositions(HashSet<ShapePointIndex>, HashMap<ShapePointIndex, Pos2>);

impl GetPointsPositions {
    pub fn new(points: HashSet<ShapePointIndex>) -> Self {
        let map = HashMap::with_capacity(points.len());
        Self(points, map)
    }

    pub fn into_not_found_and_positions(
        self,
    ) -> (HashSet<ShapePointIndex>, HashMap<ShapePointIndex, Pos2>) {
        (self.0, self.1)
    }

    fn handle_point(&mut self, index: ShapePointIndex, point: &Pos2) -> Option<()> {
        if self.0.remove(&index) {
            self.1.insert(index, *point);
            if self.0.is_empty() {
                return Some(());
            }
        }
        None
    }
}

impl IndexedShapeControlPointsVisitor for GetPointsPositions {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        point: &mut Pos2,
        _shape_type: ShapeType,
    ) -> Option<()> {
        self.handle_point(index, point)
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
        _shape_type: ShapeType,
    ) -> Option<()> {
        self.handle_point(index, control_point)
    }
}
