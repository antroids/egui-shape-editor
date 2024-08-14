use crate::shape_editor::shape_visitor::indexed_shape_control_points_visitor::IndexedShapeControlPointsVisitor;
use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType};
use egui::ahash::HashMap;
use egui::Pos2;

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
