use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter, ShapeVisitor,
};
use egui::ahash::HashMap;
use egui::Shape;
use std::mem;

#[derive(Clone)]
pub struct ReplaceShapes {
    shapes_to_replace: HashMap<usize, Shape>,
}

impl ReplaceShapes {
    pub fn new(shapes_to_replace: HashMap<usize, Shape>) -> Self {
        Self { shapes_to_replace }
    }
}

impl ShapeAction for ReplaceShapes {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        _constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        let mut visitor = ReplaceShapesVisitor::new(self.shapes_to_replace);
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Box::new(Self::new(visitor.replaced_shapes))
    }

    fn short_name(&self) -> String {
        "Replace Shapes".into()
    }
}

pub struct ReplaceShapesVisitor {
    shapes_to_replace: HashMap<usize, Shape>,
    pub(crate) replaced_shapes: HashMap<usize, Shape>,
}

impl ReplaceShapesVisitor {
    fn new(shapes_to_replace: HashMap<usize, Shape>) -> Self {
        Self {
            shapes_to_replace,
            replaced_shapes: Default::default(),
        }
    }

    pub(crate) fn single(index: usize, shape: Shape) -> Self {
        Self {
            shapes_to_replace: HashMap::from_iter([(index, shape)]),
            replaced_shapes: Default::default(),
        }
    }

    pub(crate) fn replace_by_noop<'a>(values: impl Iterator<Item = &'a usize>) -> Self {
        Self {
            shapes_to_replace: values.map(|index| (*index, Shape::Noop)).collect(),
            replaced_shapes: Default::default(),
        }
    }
}

impl IndexedShapesVisitor for ReplaceShapesVisitor {
    fn indexed_single_shape(&mut self, index: usize, shape: &mut Shape) -> Option<()> {
        if let Some(shape_replacement) = self.shapes_to_replace.remove(&index) {
            let replaced = mem::replace(shape, shape_replacement);
            self.replaced_shapes.insert(index, replaced);
        }
        self.shapes_to_replace.is_empty().then_some(())
    }
}
