use crate::shape_editor::shape_action::add_shape_points::AddShapePoints;
use crate::shape_editor::shape_action::replace_shapes::{ReplaceShapes, ReplaceShapesVisitor};
use crate::shape_editor::shape_action::{Combined, ShapeAction, ShapePoint};
use crate::shape_editor::utils::b_tree_map_grouped_by;
use crate::shape_editor::visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter, ShapePointIndex, ShapeVisitor,
};
use crate::shape_editor::Selection;
use egui::Shape;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct RemoveShapePoints(pub BTreeSet<ShapePointIndex>);

impl ShapeAction for RemoveShapePoints {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        self.apply_with_selection(shape, &mut Selection::default())
    }

    fn apply_with_selection(
        self: Box<Self>,
        shape: &mut Shape,
        selection: &mut Selection,
    ) -> Box<dyn ShapeAction> {
        let owned = *self;
        let mut points_visitor = RemoveShapePointsVisitor::from_iter(owned.0.iter());
        IndexedShapesVisitorAdapter(&mut points_visitor).visit(shape);
        points_visitor.update_selection(selection);
        if points_visitor.shapes_to_remove.is_empty() {
            Box::new(AddShapePoints(points_visitor.removed_points))
        } else {
            let mut shapes_visitor =
                ReplaceShapesVisitor::replace_by_noop(points_visitor.shapes_to_remove.iter());
            IndexedShapesVisitorAdapter(&mut shapes_visitor).visit(shape);
            Box::new(Combined::new(
                "Add Shapes and Points".into(),
                vec![
                    Box::new(ReplaceShapes::new(shapes_visitor.replaced_shapes)),
                    Box::new(AddShapePoints(points_visitor.removed_points)),
                ],
            ))
        }
    }

    fn short_name(&self) -> String {
        "Remove points".into()
    }
}

struct RemoveShapePointsVisitor {
    points_to_remove: BTreeMap<usize, BTreeSet<usize>>,

    shapes_to_remove: BTreeSet<usize>,
    removed_points: BTreeMap<usize, BTreeMap<usize, ShapePoint>>,
}

impl RemoveShapePointsVisitor {
    fn from_iter<'a>(value: impl Iterator<Item = &'a ShapePointIndex>) -> Self {
        let points_to_remove = b_tree_map_grouped_by(value, |v| (v.shape_index, v.point_index));
        Self {
            points_to_remove,
            shapes_to_remove: Default::default(),
            removed_points: Default::default(),
        }
    }

    fn update_selection(&self, selection: &mut Selection) {
        for shape_index in &self.shapes_to_remove {
            let range = selection
                .control_points_mut()
                .range(&(*shape_index, 0).into()..&(*shape_index + 1, 0).into());
            let Some(right_shape_point_index) = range.last().map(|p| p.point_index) else {
                continue;
            };
            for point_index in 0..=right_shape_point_index {
                selection
                    .control_points_mut()
                    .remove(&(*shape_index, point_index).into());
            }
        }
        for (shape_index, removed_shape_points) in &self.removed_points {
            let Some(left_point_index) = removed_shape_points.first_key_value().map(|p| *p.0)
            else {
                continue;
            };
            let range = selection
                .control_points_mut()
                .range(&(*shape_index, 0).into()..&(*shape_index + 1, 0).into());
            let Some(right_shape_point_index) = range.last().map(|p| p.point_index) else {
                continue;
            };
            let mut shift = 0;
            for point_index in left_point_index..=right_shape_point_index {
                if removed_shape_points.contains_key(&point_index) {
                    selection
                        .control_points_mut()
                        .remove(&(*shape_index, point_index).into());
                    shift += 1;
                } else if shift > 0 {
                    selection
                        .control_points_mut()
                        .remove(&(*shape_index, point_index).into());
                    selection
                        .control_points_mut()
                        .insert((*shape_index, point_index - shift).into());
                }
            }
        }
    }
}

impl IndexedShapesVisitor for RemoveShapePointsVisitor {
    fn indexed_single_shape(&mut self, shape_index: usize, shape: &mut Shape) -> Option<()> {
        if let Some(shape_points_to_remove) = self.points_to_remove.remove(&shape_index) {
            match shape {
                Shape::Path(path) => {
                    if shape_points_to_remove.len() > path.points.len() - 2 {
                        self.shapes_to_remove.insert(shape_index);
                    } else {
                        for i in shape_points_to_remove.into_iter().sorted().rev() {
                            self.removed_points
                                .entry(shape_index)
                                .or_default()
                                .insert(i, ShapePoint::Pos(path.points.remove(i)));
                        }
                    }
                }
                Shape::Mesh(mesh) => {
                    if shape_points_to_remove.len() > mesh.vertices.len() - 3 {
                        self.shapes_to_remove.insert(shape_index);
                    } else {
                        for i in shape_points_to_remove.into_iter().sorted().rev() {
                            self.removed_points.entry(shape_index).or_default().insert(
                                i,
                                ShapePoint::Vertex(mesh.vertices.remove(i), mesh.indices.remove(i)),
                            );
                        }
                    }
                }
                _ => {
                    self.shapes_to_remove.insert(shape_index);
                }
            }
        }
        self.points_to_remove.is_empty().then_some(())
    }
}
