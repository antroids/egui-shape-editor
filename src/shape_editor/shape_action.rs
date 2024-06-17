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
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction>;
    fn apply_with_selection(
        self: Box<Self>,
        shape: &mut Shape,
        _selection: &mut Selection,
    ) -> Box<dyn ShapeAction> {
        self.apply(shape)
    }
    fn short_name(&self) -> String;
}

dyn_clone::clone_trait_object!(ShapeAction);

#[derive(Clone)]
pub struct Noop;

impl ShapeAction for Noop {
    fn apply(self: Box<Self>, _shape: &mut Shape) -> Box<dyn ShapeAction> {
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
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        let owned = *self;
        let inverted: Vec<Box<dyn ShapeAction>> = owned
            .actions
            .into_iter()
            .map(|action| action.apply(shape))
            .rev()
            .collect();
        Box::new(Self::new(format!("Undo {}", owned.short_name), inverted))
    }

    fn apply_with_selection(
        self: Box<Self>,
        shape: &mut Shape,
        selection: &mut Selection,
    ) -> Box<dyn ShapeAction> {
        let owned = *self;
        let inverted: Vec<Box<dyn ShapeAction>> = owned
            .actions
            .into_iter()
            .map(|action| action.apply_with_selection(shape, selection))
            .rev()
            .collect();
        Box::new(Self::new(format!("Undo {}", owned.short_name), inverted))
    }

    fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShapePoint {
    Pos(Pos2),
    Vertex(Vertex, u32),
}
