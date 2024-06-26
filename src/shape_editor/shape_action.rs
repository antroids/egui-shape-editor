use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::Selection;
use dyn_clone::DynClone;
use egui::emath::Pos2;
use egui::epaint::Vertex;
use egui::Shape;

pub mod add_shape_points;
pub mod insert_shape;
pub mod move_shape_points;
pub mod remove_shape_points;
pub mod replace_shapes;

pub trait ShapeAction: DynClone + Send + Sync {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction>;
    fn apply_with_selection(
        self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
        selection: &mut Selection,
    ) -> Box<dyn ShapeAction> {
        Box::new(RestoreSelectionActionWrapper::new(
            self.apply(shape, constraints),
            selection.clone(),
        ))
    }
    fn short_name(&self) -> String;
}

dyn_clone::clone_trait_object!(ShapeAction);

#[derive(Clone)]
pub struct Noop;

impl ShapeAction for Noop {
    fn apply(
        self: Box<Self>,
        _shape: &mut Shape,
        _constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        self
    }

    fn short_name(&self) -> String {
        "None".into()
    }
}

#[derive(Clone)]
pub struct Combined {
    short_name: String,
    actions: Vec<Box<dyn ShapeAction>>,
}

impl Combined {
    pub fn new(short_name: String, actions: Vec<Box<dyn ShapeAction>>) -> Self {
        Self {
            short_name,
            actions,
        }
    }
}

impl ShapeAction for Combined {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        let owned = *self;
        let inverted: Vec<Box<dyn ShapeAction>> = owned
            .actions
            .into_iter()
            .map(|action| action.apply(shape, constraints))
            .rev()
            .collect();
        Box::new(Self::new(format!("Undo {}", owned.short_name), inverted))
    }

    fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

#[derive(Clone)]
pub struct RestoreSelectionActionWrapper {
    action: Box<dyn ShapeAction>,
    selection: Selection,
}

impl RestoreSelectionActionWrapper {
    pub fn new(action: Box<dyn ShapeAction>, selection: Selection) -> Self {
        Self { action, selection }
    }
}

impl ShapeAction for RestoreSelectionActionWrapper {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        self.action.apply(shape, constraints)
    }

    fn apply_with_selection(
        self: Box<Self>,
        shape: &mut Shape,
        constraints: &mut Constraints,
        selection: &mut Selection,
    ) -> Box<dyn ShapeAction> {
        let result = self
            .action
            .apply_with_selection(shape, constraints, selection);
        *selection = self.selection;
        result
    }

    fn short_name(&self) -> String {
        self.action.short_name()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShapePoint {
    Pos(Pos2),
    Vertex(Vertex, u32),
}
