use crate::shape_editor::shape_visitor;
use crate::shape_editor::shape_visitor::indexed_shapes_visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter,
};
use egui::Shape;

#[derive(Clone)]
pub struct CountShapes;

impl IndexedShapesVisitor for CountShapes {}

impl CountShapes {
    pub fn count(shape: &mut Shape) -> usize {
        let mut count = 0usize;
        shape_visitor::visit_shape_with_index(
            &mut IndexedShapesVisitorAdapter(&mut Self),
            shape,
            &mut count,
        );
        count
    }
}
