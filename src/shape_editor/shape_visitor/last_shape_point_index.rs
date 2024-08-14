use crate::shape_editor::shape_visitor::indexed_shape_control_points_visitor::{
    IndexedShapeControlPointsVisitor, IndexedShapeControlPointsVisitorAdapter,
};
use crate::shape_editor::shape_visitor::{ShapePointIndex, ShapeType, ShapeVisitor};
use egui::ahash::HashMap;
use egui::{Pos2, Shape};

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
