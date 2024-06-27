use crate::shape_editor::visitor::ShapePointIndex;
use egui::ahash::{HashMap, HashSet};

#[derive(Default, Clone)]
pub struct Constraints {
    pub(crate) translation_propagation: HashMap<ShapePointIndex, HashSet<ShapePointIndex>>,
}

impl Constraints {
    pub fn connect_translation_bidirectional(
        &mut self,
        index1: ShapePointIndex,
        index2: ShapePointIndex,
    ) {
        self.translation_propagation
            .entry(index1)
            .and_modify(|set| {
                set.insert(index2);
            })
            .or_insert_with(|| HashSet::from_iter([index2]));
        self.translation_propagation
            .entry(index2)
            .and_modify(|set| {
                set.insert(index1);
            })
            .or_insert_with(|| HashSet::from_iter([index1]));
    }
}
