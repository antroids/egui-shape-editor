use crate::shape_editor::visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter, ShapeVisitor,
};
use derivative::Derivative;
use egui::ahash::{HashMap, HashSet};
use egui::epaint::{
    CircleShape, CubicBezierShape, PathShape, QuadraticBezierShape, RectShape, TextShape,
};
use egui::{Color32, Mesh, Pos2, Rounding, Shape, Stroke, TextureId};
use num_traits::Zero;
use ordered_float::NotNan;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Derivative, Debug)]
#[derivative(Hash, Eq)]
pub enum ShapeParam {
    StrokeColor(Color32),
    StrokeWidth(NotNan<f32>),
    Rounding(#[derivative(Hash(hash_with = "rounding_hash"))] Rounding),
    FillColor(Color32),
    ClosedShape(bool),
    Radius(NotNan<f32>),
    Texture(TextureId),
}

#[derive(Clone, Debug)]
pub struct ShapesParams(HashMap<usize, HashSet<ShapeParam>>);

impl ShapesParams {
    pub fn extract(shape: &mut Shape, index: HashSet<usize>) -> Self {
        let mut visitor = ExtractShapeParamsVisitor {
            shapes: index,
            shape_params: Default::default(),
        };
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Self(visitor.shape_params)
    }
}

fn rounding_hash<H>(rounding: &Rounding, state: &mut H)
where
    H: Hasher,
{
    not_nan_f32(rounding.sw).hash(state);
    not_nan_f32(rounding.nw).hash(state);
    not_nan_f32(rounding.se).hash(state);
    not_nan_f32(rounding.ne).hash(state);
}

struct ExtractShapeParamsVisitor {
    shapes: HashSet<usize>,
    shape_params: HashMap<usize, HashSet<ShapeParam>>,
}

impl IndexedShapesVisitor for ExtractShapeParamsVisitor {
    fn indexed_line_segment(
        &mut self,
        index: usize,
        _points: &mut [Pos2; 2],
        stroke: &mut Stroke,
    ) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(stroke.width)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_path(&mut self, index: usize, path: &mut PathShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(path.stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(path.stroke.width)),
                    ShapeParam::ClosedShape(path.closed),
                    ShapeParam::FillColor(path.fill),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_circle(&mut self, index: usize, circle: &mut CircleShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(circle.stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(circle.stroke.width)),
                    ShapeParam::FillColor(circle.fill),
                    ShapeParam::Radius(not_nan_f32(circle.radius)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_rect(&mut self, index: usize, rect: &mut RectShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(rect.stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(rect.stroke.width)),
                    ShapeParam::FillColor(rect.fill),
                    ShapeParam::Rounding(rect.rounding),
                    ShapeParam::Texture(rect.fill_texture_id),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_text(&mut self, index: usize, _text: &mut TextShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    // TODO
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_mesh(&mut self, index: usize, mesh: &mut Mesh) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([ShapeParam::Texture(mesh.texture_id)]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_quadratic_bezier(
        &mut self,
        index: usize,
        bezier: &mut QuadraticBezierShape,
    ) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(bezier.stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(bezier.stroke.width)),
                    ShapeParam::ClosedShape(bezier.closed),
                    ShapeParam::FillColor(bezier.fill),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_cubic_bezier(&mut self, index: usize, bezier: &mut CubicBezierShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                HashSet::from_iter([
                    ShapeParam::StrokeColor(bezier.stroke.color),
                    ShapeParam::StrokeWidth(not_nan_f32(bezier.stroke.width)),
                    ShapeParam::ClosedShape(bezier.closed),
                    ShapeParam::FillColor(bezier.fill),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }
}

struct ApplyShapeParamsVisitor {
    shape_params: HashMap<usize, HashSet<ShapeParam>>,
    changed_params: HashMap<usize, HashSet<ShapeParam>>,
}

impl IndexedShapesVisitor for ApplyShapeParamsVisitor {
    fn indexed_line_segment(
        &mut self,
        index: usize,
        _points: &mut [Pos2; 2],
        stroke: &mut Stroke,
    ) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(stroke.width);
                        std::mem::swap(v, &mut value);
                        stroke.width = value.into_inner();
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_path(&mut self, index: usize, path: &mut PathShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut path.stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(path.stroke.width);
                        std::mem::swap(v, &mut value);
                        path.stroke.width = value.into_inner();
                    }
                    ShapeParam::ClosedShape(v) => {
                        std::mem::swap(v, &mut path.closed);
                    }
                    ShapeParam::FillColor(v) => {
                        std::mem::swap(v, &mut path.fill);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_circle(&mut self, index: usize, circle: &mut CircleShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut circle.stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(circle.stroke.width);
                        std::mem::swap(v, &mut value);
                        circle.stroke.width = value.into_inner();
                    }
                    ShapeParam::Radius(v) => {
                        let mut value = not_nan_f32(circle.radius);
                        std::mem::swap(v, &mut value);
                        circle.radius = value.into_inner();
                    }
                    ShapeParam::FillColor(v) => {
                        std::mem::swap(v, &mut circle.fill);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_rect(&mut self, index: usize, rect: &mut RectShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut rect.stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(rect.stroke.width);
                        std::mem::swap(v, &mut value);
                        rect.stroke.width = value.into_inner();
                    }
                    ShapeParam::Rounding(v) => {
                        std::mem::swap(v, &mut rect.rounding);
                    }
                    ShapeParam::FillColor(v) => {
                        std::mem::swap(v, &mut rect.fill);
                    }
                    ShapeParam::Texture(v) => {
                        std::mem::swap(v, &mut rect.fill_texture_id);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_text(&mut self, index: usize, _text: &mut TextShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    _ => continue,
                }
                // changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_mesh(&mut self, index: usize, mesh: &mut Mesh) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::Texture(v) => {
                        std::mem::swap(v, &mut mesh.texture_id);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_quadratic_bezier(
        &mut self,
        index: usize,
        bezier: &mut QuadraticBezierShape,
    ) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut bezier.stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(bezier.stroke.width);
                        std::mem::swap(v, &mut value);
                        bezier.stroke.width = value.into_inner();
                    }
                    ShapeParam::ClosedShape(v) => {
                        std::mem::swap(v, &mut bezier.closed);
                    }
                    ShapeParam::FillColor(v) => {
                        std::mem::swap(v, &mut bezier.fill);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_cubic_bezier(&mut self, index: usize, bezier: &mut CubicBezierShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = HashSet::default();
            for mut param in params {
                match &mut param {
                    ShapeParam::StrokeColor(v) => {
                        std::mem::swap(v, &mut bezier.stroke.color);
                    }
                    ShapeParam::StrokeWidth(v) => {
                        let mut value = not_nan_f32(bezier.stroke.width);
                        std::mem::swap(v, &mut value);
                        bezier.stroke.width = value.into_inner();
                    }
                    ShapeParam::ClosedShape(v) => {
                        std::mem::swap(v, &mut bezier.closed);
                    }
                    ShapeParam::FillColor(v) => {
                        std::mem::swap(v, &mut bezier.fill);
                    }
                    _ => continue,
                }
                changed.insert(param);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }
}

fn not_nan_f32(v: f32) -> NotNan<f32> {
    NotNan::new(v).unwrap_or(NotNan::zero())
}
