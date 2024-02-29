use crate::shape_editor::utils::map_grouped_by;
use crate::shape_editor::visitor::{
    CountShapes, IndexedShapeControlPointsVisitor, ShapePointIndex, ShapeType, ShapeVisitor,
};
use crate::shape_editor::visitor::{
    IndexedShapeControlPointsVisitorAdapter, IndexedShapesVisitor, IndexedShapesVisitorAdapter,
};
use dyn_clone::DynClone;
use egui::ahash::{HashMap, HashSet};
use egui::emath::Pos2;
use egui::epaint::{CircleShape, CubicBezierShape, PathShape, QuadraticBezierShape, Vertex};
use egui::{Color32, Mesh, Rect, Shape, Stroke, Vec2};
use itertools::Itertools;
use std::mem;
use std::ops::{AddAssign, DerefMut, Neg};

pub trait ShapeAction: DynClone + Send + Sync {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction>;
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
pub struct MoveShapeControlPoints(HashMap<ShapePointIndex, Vec2>);

impl IndexedShapeControlPointsVisitor for MoveShapeControlPoints {
    fn indexed_path_point(
        &mut self,
        index: ShapePointIndex,
        point: &mut Pos2,
        shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
            point.add_assign(translation);
            if shape_type == ShapeType::Circle {
                self.0.remove(&index.next_point());
            }
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }

    fn indexed_control_point(
        &mut self,
        index: ShapePointIndex,
        control_point: &mut Pos2,
        _connected_points: HashMap<ShapePointIndex, Pos2>,
        shape_type: ShapeType,
    ) -> Option<()> {
        if let Some(translation) = self.0.remove(&index) {
            if shape_type != ShapeType::Circle || !self.0.contains_key(&index.prev_point()) {
                control_point.add_assign(translation);
            }
        }
        if self.0.is_empty() {
            Some(())
        } else {
            None
        }
    }
}
impl MoveShapeControlPoints {
    pub fn from_index_and_translation<'a>(
        indexes: impl IntoIterator<Item = &'a ShapePointIndex>,
        translation: &Vec2,
    ) -> Self {
        Self(
            indexes
                .into_iter()
                .map(|index| (*index, *translation))
                .collect(),
        )
    }

    pub fn invert(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|(index, translate)| (*index, translate.neg()))
                .collect(),
        )
    }
}

impl ShapeAction for MoveShapeControlPoints {
    fn apply(mut self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        IndexedShapeControlPointsVisitorAdapter(self.deref_mut()).visit(shape);
        Box::new(self.invert())
    }

    fn short_name(&self) -> String {
        "Move".into()
    }
}

#[derive(Clone)]
pub struct InsertShape {
    shape: Option<Shape>,
    replace: Option<usize>,
}

impl InsertShape {
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            shape: Some(shape),
            replace: None,
        }
    }

    pub fn replace_by_noop(index: usize) -> Self {
        Self {
            shape: None,
            replace: Some(index),
        }
    }

    pub fn replace_by_shape(index: usize, shape: Shape) -> Self {
        Self {
            shape: Some(shape),
            replace: Some(index),
        }
    }

    pub fn cubic_bezier_from_two_points(
        start_point: Pos2,
        start_point_control: Option<Pos2>,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        let distance = start_point.distance(end_point);
        let start_control_point = start_point_control
            .map(|pos| start_point + (start_point - pos).normalized() * distance / 3.0)
            .unwrap_or(start_point);
        let end_control_point =
            end_point - (end_point - start_control_point).normalized() * distance / 3.0;
        InsertShape::from_shape(
            CubicBezierShape::from_points_stroke(
                [
                    start_point,
                    start_control_point,
                    end_control_point,
                    end_point,
                ],
                false,
                Color32::TRANSPARENT,
                stroke,
            )
            .into(),
        )
    }

    pub fn quadratic_bezier_from_two_points(
        start_point: Pos2,
        start_point_control: Option<Pos2>,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        let distance = start_point.distance(end_point);
        let start_control_point = start_point_control
            .map(|pos| start_point + (start_point - pos).normalized() * distance / 3.0)
            .unwrap_or(start_point);
        InsertShape::from_shape(
            QuadraticBezierShape::from_points_stroke(
                [start_point, start_control_point, end_point],
                false,
                Color32::TRANSPARENT,
                stroke,
            )
            .into(),
        )
    }

    pub fn circle_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::Circle(CircleShape::stroke(
            end_point,
            start_point.distance(end_point),
            stroke,
        ))
        .into()
    }

    pub fn line_segment_from_two_points(
        start_point: Pos2,
        end_point: Pos2,
        stroke: Stroke,
    ) -> Self {
        Shape::line_segment([start_point, end_point], stroke).into()
    }

    pub fn path_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::Path(PathShape::line(vec![start_point, end_point], stroke)).into()
    }

    pub fn rect_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        Shape::rect_stroke(Rect::from_two_pos(start_point, end_point), 0.0, stroke).into()
    }

    pub fn mesh_from_two_points(start_point: Pos2, end_point: Pos2, stroke: Stroke) -> Self {
        let mut mesh = Mesh::default();
        let third_point = start_point + (start_point - end_point).rot90();
        let first_vertex_index = mesh.vertices.len() as u32;
        mesh.vertices.push(Vertex {
            pos: start_point,
            uv: Pos2::ZERO,
            color: stroke.color,
        });
        mesh.vertices.push(Vertex {
            pos: end_point,
            uv: Pos2::ZERO,
            color: stroke.color,
        });
        mesh.vertices.push(Vertex {
            pos: third_point,
            uv: Pos2::ZERO,
            color: stroke.color,
        });
        mesh.add_triangle(
            first_vertex_index,
            first_vertex_index + 1,
            first_vertex_index + 2,
        );
        Shape::mesh(mesh).into()
    }
}

impl From<Shape> for InsertShape {
    fn from(value: Shape) -> Self {
        Self::from_shape(value)
    }
}

impl ShapeAction for InsertShape {
    fn apply(mut self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        if let Some(replace) = self.replace {
            let mut visitor =
                ReplaceShapesVisitor::single(replace, self.shape.unwrap_or(Shape::Noop));
            IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
            visitor
                .replaced_shapes
                .remove(&replace)
                .map_or(Box::new(Noop), |replaced| {
                    Box::new(Self {
                        shape: Some(replaced),
                        replace: self.replace,
                    })
                })
        } else {
            if !matches!(shape, Shape::Vec(_)) {
                let original = mem::replace(shape, Shape::Noop);
                *shape = Shape::Vec(vec![original]);
            }
            if let Shape::Vec(vec) = shape {
                vec.push(self.shape.take().unwrap_or(Shape::Noop));
            }
            let index = CountShapes::count(shape) - 1;
            Box::new(InsertShape::replace_by_noop(index))
        }
    }

    fn short_name(&self) -> String {
        "Insert Shape".into()
    }
}

#[derive(Clone)]
pub struct ReplaceShapes {
    shapes_to_replace: HashMap<usize, Shape>,
}

impl ReplaceShapes {
    pub fn new(shapes_to_replace: HashMap<usize, Shape>) -> Self {
        Self { shapes_to_replace }
    }

    pub fn single(index: usize, shape: Shape) -> Self {
        Self {
            shapes_to_replace: HashMap::from_iter([(index, shape)]),
        }
    }

    pub fn replace_by_noop<'a>(values: impl Iterator<Item = &'a usize>) -> Self {
        Self {
            shapes_to_replace: values.map(|index| (*index, Shape::Noop)).collect(),
        }
    }
}

impl ShapeAction for ReplaceShapes {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        let mut visitor = ReplaceShapesVisitor::new(self.shapes_to_replace);
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Box::new(Self::new(visitor.replaced_shapes))
    }

    fn short_name(&self) -> String {
        "Replace Shapes".into()
    }
}

struct ReplaceShapesVisitor {
    shapes_to_replace: HashMap<usize, Shape>,
    replaced_shapes: HashMap<usize, Shape>,
}

impl ReplaceShapesVisitor {
    fn new(shapes_to_replace: HashMap<usize, Shape>) -> Self {
        Self {
            shapes_to_replace,
            replaced_shapes: Default::default(),
        }
    }

    fn single(index: usize, shape: Shape) -> Self {
        Self {
            shapes_to_replace: HashMap::from_iter([(index, shape)]),
            replaced_shapes: Default::default(),
        }
    }

    fn replace_by_noop<'a>(values: impl Iterator<Item = &'a usize>) -> Self {
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

    pub fn from_actions(actions: Vec<Box<dyn ShapeAction>>) -> Self {
        Self {
            short_name: "Combined action".to_string(),
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

    fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShapePoint {
    Pos(Pos2),
    Vertex(Vertex, u32),
}

#[derive(Clone)]
pub struct RemoveShapePoints(pub HashSet<ShapePointIndex>);

impl ShapeAction for RemoveShapePoints {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        let owned = *self;
        let mut points_visitor = RemoveShapePointsVisitor::from_iter(owned.0.iter());
        IndexedShapesVisitorAdapter(&mut points_visitor).visit(shape);
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

#[derive(Clone)]
pub struct AddShapePoints(pub HashMap<usize, HashMap<usize, ShapePoint>>);

impl AddShapePoints {
    pub fn single_point(index: ShapePointIndex, point: ShapePoint) -> Self {
        Self(HashMap::from_iter([(
            index.shape_index,
            HashMap::from_iter([(index.point_index, point)]),
        )]))
    }
}

impl ShapeAction for AddShapePoints {
    fn apply(self: Box<Self>, shape: &mut Shape) -> Box<dyn ShapeAction> {
        let owned = *self;
        let mut visitor = AddShapePointsVisitor {
            index_to_add: owned.0,
            added: Default::default(),
        };
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Box::new(RemoveShapePoints(visitor.added))
    }

    fn short_name(&self) -> String {
        "Add points".into()
    }
}

#[derive(Clone)]
struct RemoveShapePointsVisitor {
    points_to_remove: HashMap<usize, HashSet<usize>>,

    shapes_to_remove: HashSet<usize>,
    removed_points: HashMap<usize, HashMap<usize, ShapePoint>>,
}

impl RemoveShapePointsVisitor {
    fn from_iter<'a>(value: impl Iterator<Item = &'a ShapePointIndex>) -> Self {
        let points_to_remove = map_grouped_by(value, |v| (v.shape_index, v.point_index));
        Self {
            points_to_remove,
            shapes_to_remove: Default::default(),
            removed_points: Default::default(),
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

struct AddShapePointsVisitor {
    index_to_add: HashMap<usize, HashMap<usize, ShapePoint>>,
    added: HashSet<ShapePointIndex>,
}

impl IndexedShapesVisitor for AddShapePointsVisitor {
    fn indexed_single_shape(&mut self, shape_index: usize, shape: &mut Shape) -> Option<()> {
        if let Some(points_to_add) = self.index_to_add.remove(&shape_index) {
            for (point_index, shape_point) in points_to_add.into_iter().sorted_by_key(|(k, _)| *k) {
                match shape {
                    Shape::Path(path) => {
                        if let ShapePoint::Pos(pos) = shape_point {
                            path.points.insert(point_index, pos);
                            self.added.insert((shape_index, point_index).into());
                        }
                    }
                    Shape::Mesh(mesh) => {
                        if let ShapePoint::Vertex(vertex, vertex_index) = shape_point {
                            mesh.vertices.insert(point_index, vertex);
                            mesh.indices.insert(point_index, vertex_index);
                            self.added.insert((shape_index, point_index).into());
                        }
                    }
                    _ => {}
                }
            }
        }

        self.index_to_add.is_empty().then_some(())
    }
}
