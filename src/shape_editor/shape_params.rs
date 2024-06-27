use crate::shape_editor::constraints::Constraints;
use crate::shape_editor::shape_action::ShapeAction;
use crate::shape_editor::visitor::{
    IndexedShapesVisitor, IndexedShapesVisitorAdapter, ShapeVisitor,
};
use derivative::Derivative;
use egui::ahash::HashSet;
use egui::epaint::{
    CircleShape, Color32, CubicBezierShape, EllipseShape, PathShape, QuadraticBezierShape,
    RectShape, Shape, Stroke, TextShape,
};
use egui::{Mesh, Pos2, Rounding, TextureId};
use num_traits::Zero;
use ordered_float::NotNan;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Derivative, Debug)]
#[derivative(Hash, Eq)]
pub enum ParamValue {
    Color(Color32),
    Float(NotNan<f32>),
    Rounding(#[derivative(Hash(hash_with = "rounding_hash"))] Rounding),
    Boolean(bool),
    Texture(TextureId),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum ParamType {
    StrokeColor,
    StrokeWidth,
    Rounding,
    FillColor,
    ClosedShape,
    Radius,
    Texture,
}

#[derive(Clone, Debug)]
pub struct ShapesParams(pub BTreeMap<usize, BTreeMap<ParamType, ParamValue>>);

impl ShapesParams {
    pub fn extract(shape: &mut Shape, index: HashSet<usize>) -> Self {
        let mut visitor = ExtractShapeParamsVisitor {
            shapes: index,
            shape_params: Default::default(),
        };
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Self(visitor.shape_params)
    }

    pub fn common(&self) -> BTreeMap<ParamType, Option<ParamValue>> {
        let mut params: BTreeMap<ParamType, HashSet<ParamValue>> = BTreeMap::default();
        for (_, param) in &self.0 {
            for (ty, val) in param {
                if let Some(values) = params.get_mut(ty) {
                    if values.len() < 2 {
                        values.insert(*val);
                    }
                } else {
                    params.insert(*ty, HashSet::from_iter([*val]));
                }
            }
        }
        params
            .into_iter()
            .map(|(k, v)| {
                if v.len() == 1 {
                    (k, Some(v.into_iter().next().expect("Must be at least one")))
                } else {
                    (k, None)
                }
            })
            .collect()
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
    shape_params: BTreeMap<usize, BTreeMap<ParamType, ParamValue>>,
}

struct ApplyShapeParamsVisitor {
    shape_params: BTreeMap<usize, BTreeMap<ParamType, ParamValue>>,
    changed_params: BTreeMap<usize, BTreeMap<ParamType, ParamValue>>,
}

#[derive(Clone)]
pub struct ApplyShapeParams(pub BTreeMap<usize, BTreeMap<ParamType, ParamValue>>);

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
                BTreeMap::from_iter([
                    (ParamType::StrokeColor, ParamValue::Color(stroke.color)),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(stroke.width)),
                    ),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_path(&mut self, index: usize, path: &mut PathShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
                    (ParamType::StrokeColor, ParamValue::Color(path.stroke.color)),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(path.stroke.width)),
                    ),
                    (ParamType::ClosedShape, ParamValue::Boolean(path.closed)),
                    (ParamType::FillColor, ParamValue::Color(path.fill)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_circle(&mut self, index: usize, circle: &mut CircleShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
                    (
                        ParamType::StrokeColor,
                        ParamValue::Color(circle.stroke.color),
                    ),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(circle.stroke.width)),
                    ),
                    (ParamType::FillColor, ParamValue::Color(circle.fill)),
                    (
                        ParamType::Radius,
                        ParamValue::Float(not_nan_f32(circle.radius)),
                    ),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_ellipse(&mut self, index: usize, ellipse: &mut EllipseShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
                    (
                        ParamType::StrokeColor,
                        ParamValue::Color(ellipse.stroke.color),
                    ),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(ellipse.stroke.width)),
                    ),
                    (ParamType::FillColor, ParamValue::Color(ellipse.fill)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_rect(&mut self, index: usize, rect: &mut RectShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
                    (ParamType::StrokeColor, ParamValue::Color(rect.stroke.color)),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(rect.stroke.width)),
                    ),
                    (ParamType::FillColor, ParamValue::Color(rect.fill)),
                    (ParamType::Rounding, ParamValue::Rounding(rect.rounding)),
                    (
                        ParamType::Texture,
                        ParamValue::Texture(rect.fill_texture_id),
                    ),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_text(&mut self, index: usize, _text: &mut TextShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
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
                BTreeMap::from_iter([(ParamType::Texture, ParamValue::Texture(mesh.texture_id))]),
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
                BTreeMap::from_iter([
                    (
                        ParamType::StrokeColor,
                        ParamValue::Color(bezier.stroke.color),
                    ),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(bezier.stroke.width)),
                    ),
                    (ParamType::ClosedShape, ParamValue::Boolean(bezier.closed)),
                    (ParamType::FillColor, ParamValue::Color(bezier.fill)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }

    fn indexed_cubic_bezier(&mut self, index: usize, bezier: &mut CubicBezierShape) -> Option<()> {
        if self.shapes.remove(&index) {
            self.shape_params.insert(
                index,
                BTreeMap::from_iter([
                    (
                        ParamType::StrokeColor,
                        ParamValue::Color(bezier.stroke.color),
                    ),
                    (
                        ParamType::StrokeWidth,
                        ParamValue::Float(not_nan_f32(bezier.stroke.width)),
                    ),
                    (ParamType::ClosedShape, ParamValue::Boolean(bezier.closed)),
                    (ParamType::FillColor, ParamValue::Color(bezier.fill)),
                ]),
            );
        }
        self.shapes.is_empty().then_some(())
    }
}

impl IndexedShapesVisitor for ApplyShapeParamsVisitor {
    fn indexed_line_segment(
        &mut self,
        index: usize,
        _points: &mut [Pos2; 2],
        stroke: &mut Stroke,
    ) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(stroke.width);
                        std::mem::swap(v, &mut value);
                        stroke.width = value.into_inner();
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_path(&mut self, index: usize, path: &mut PathShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut path.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(path.stroke.width);
                        std::mem::swap(v, &mut value);
                        path.stroke.width = value.into_inner();
                    }
                    (ParamType::ClosedShape, ParamValue::Boolean(v)) => {
                        std::mem::swap(v, &mut path.closed);
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut path.fill);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_circle(&mut self, index: usize, circle: &mut CircleShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut circle.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(circle.stroke.width);
                        std::mem::swap(v, &mut value);
                        circle.stroke.width = value.into_inner();
                    }
                    (ParamType::Radius, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(circle.radius);
                        std::mem::swap(v, &mut value);
                        circle.radius = value.into_inner();
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut circle.fill);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_ellipse(&mut self, index: usize, ellipse: &mut EllipseShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut ellipse.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(ellipse.stroke.width);
                        std::mem::swap(v, &mut value);
                        ellipse.stroke.width = value.into_inner();
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut ellipse.fill);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_rect(&mut self, index: usize, rect: &mut RectShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut rect.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(rect.stroke.width);
                        std::mem::swap(v, &mut value);
                        rect.stroke.width = value.into_inner();
                    }
                    (ParamType::Rounding, ParamValue::Rounding(v)) => {
                        std::mem::swap(v, &mut rect.rounding);
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut rect.fill);
                    }
                    (ParamType::Texture, ParamValue::Texture(v)) => {
                        std::mem::swap(v, &mut rect.fill_texture_id);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_text(&mut self, index: usize, _text: &mut TextShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    _ => continue,
                }
                // changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_mesh(&mut self, index: usize, mesh: &mut Mesh) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::Texture, ParamValue::Texture(v)) => {
                        std::mem::swap(v, &mut mesh.texture_id);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
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
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut bezier.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(bezier.stroke.width);
                        std::mem::swap(v, &mut value);
                        bezier.stroke.width = value.into_inner();
                    }
                    (ParamType::ClosedShape, ParamValue::Boolean(v)) => {
                        std::mem::swap(v, &mut bezier.closed);
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut bezier.fill);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }

    fn indexed_cubic_bezier(&mut self, index: usize, bezier: &mut CubicBezierShape) -> Option<()> {
        if let Some(params) = self.shape_params.remove(&index) {
            let mut changed = BTreeMap::default();
            for mut param in params {
                match &mut param {
                    (ParamType::StrokeColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut bezier.stroke.color);
                    }
                    (ParamType::StrokeWidth, ParamValue::Float(v)) => {
                        let mut value = not_nan_f32(bezier.stroke.width);
                        std::mem::swap(v, &mut value);
                        bezier.stroke.width = value.into_inner();
                    }
                    (ParamType::ClosedShape, ParamValue::Boolean(v)) => {
                        std::mem::swap(v, &mut bezier.closed);
                    }
                    (ParamType::FillColor, ParamValue::Color(v)) => {
                        std::mem::swap(v, &mut bezier.fill);
                    }
                    _ => continue,
                }
                changed.insert(param.0, param.1);
            }
            if !changed.is_empty() {
                self.changed_params.insert(index, changed);
            }
        }
        self.shape_params.is_empty().then_some(())
    }
}

impl ApplyShapeParams {
    pub fn from_common(params: BTreeMap<ParamType, ParamValue>, shapes: HashSet<usize>) -> Self {
        Self(
            shapes
                .into_iter()
                .map(|shape| (shape, params.clone()))
                .collect(),
        )
    }
}

impl ShapeAction for ApplyShapeParams {
    fn apply(
        self: Box<Self>,
        shape: &mut Shape,
        _constraints: &mut Constraints,
    ) -> Box<dyn ShapeAction> {
        let mut visitor = ApplyShapeParamsVisitor {
            shape_params: (*self).0,
            changed_params: Default::default(),
        };
        IndexedShapesVisitorAdapter(&mut visitor).visit(shape);
        Box::new(Self(visitor.changed_params))
    }

    fn short_name(&self) -> String {
        "Update Parameters".into()
    }
}

fn not_nan_f32(v: f32) -> NotNan<f32> {
    NotNan::new(v).unwrap_or(NotNan::zero())
}
