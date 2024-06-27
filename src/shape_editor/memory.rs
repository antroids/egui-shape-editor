use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::interaction::Interaction;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::snap::SnapInfo;
use crate::shape_editor::transform::Transform;
use crate::shape_editor::Selection;
use egui::{Context, Id, Pos2, Shape};

#[derive(Clone)]
pub struct ShapeEditorMemory {
    transform: Transform,
    interaction: Vec<Box<dyn Interaction>>,
    action_history: Vec<(Box<dyn ShapeAction>, String)>,
    last_mouse_hover_pos: Pos2,
    last_canvas_mouse_hover_pos: Pos2,
    selection: Selection,
    pub(crate) snap: SnapInfo,
    pub(crate) constraints: Constraints,
}

impl Default for ShapeEditorMemory {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            interaction: Vec::new(),
            action_history: Vec::new(),
            last_mouse_hover_pos: Pos2::ZERO,
            last_canvas_mouse_hover_pos: Pos2::ZERO,
            selection: Default::default(),
            snap: Default::default(),
            constraints: Constraints::default(),
        }
    }
}

impl ShapeEditorMemory {
    pub(crate) fn load(ctx: &Context, id: Id) -> Self {
        ctx.data(|data| data.get_temp(id)).unwrap_or_default()
    }

    pub(crate) fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| data.insert_temp(id, self))
    }

    pub(crate) fn apply_boxed_action(&mut self, action: Box<dyn ShapeAction>, shape: &mut Shape) {
        let short_name = action.short_name();
        let undo_action =
            action.apply_with_selection(shape, &mut self.constraints, &mut self.selection);
        self.push_action_history(undo_action, short_name)
    }

    pub(crate) fn push_action_history(&mut self, action: Box<dyn ShapeAction>, short_name: String) {
        self.action_history.push((action, short_name))
    }

    pub(crate) fn undo(&mut self, shape: &mut Shape) {
        if let Some((action, _)) = self.action_history.pop() {
            action.apply_with_selection(shape, &mut self.constraints, &mut self.selection);
        }
    }

    pub(crate) fn transform(&self) -> &Transform {
        &self.transform
    }
    pub(crate) fn interaction(&self) -> &Vec<Box<dyn Interaction>> {
        &self.interaction
    }

    pub(crate) fn interaction_mut(&mut self) -> &mut Vec<Box<dyn Interaction>> {
        &mut self.interaction
    }
    pub(crate) fn action_history(&self) -> &Vec<(Box<dyn ShapeAction>, String)> {
        &self.action_history
    }
    pub(crate) fn last_mouse_hover_pos(&self) -> Pos2 {
        self.last_mouse_hover_pos
    }
    pub(crate) fn selection(&self) -> &Selection {
        &self.selection
    }

    pub(crate) fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }
    pub(crate) fn snap(&self) -> &SnapInfo {
        &self.snap
    }

    pub(crate) fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub(crate) fn set_last_mouse_hover_pos(&mut self, last_mouse_hover_pos: Pos2) {
        self.last_mouse_hover_pos = last_mouse_hover_pos;
    }
    pub(crate) fn set_last_canvas_mouse_hover_pos(&mut self, last_canvas_mouse_hover_pos: Pos2) {
        self.last_canvas_mouse_hover_pos = last_canvas_mouse_hover_pos;
    }
}
