use crate::shape_editor::visitor::{visit_shape, ShapeVisitor};
use egui::emath::{Pos2, Rect, RectTransform, Vec2};
use egui::epaint::{
    CircleShape, CubicBezierShape, Mesh, PathShape, QuadraticBezierShape, RectShape, Shape, Stroke,
    TextShape,
};
use egui::Rangef;

#[derive(Clone, Debug)]
pub struct Transform(pub(crate) RectTransform);

impl Default for Transform {
    fn default() -> Self {
        Self(RectTransform::identity(Rect::from_min_size(
            Pos2::ZERO,
            VEC_ONE,
        )))
    }
}

const VEC_ONE: Vec2 = Vec2::splat(1.0);

impl Transform {
    pub fn from_to(from: Rect, to: Rect) -> Self {
        Self(RectTransform::from_to(from, to))
    }

    pub fn from_translate(translate: Vec2) -> Self {
        Self::from_min(translate.to_pos2())
    }

    pub fn from_min(min: Pos2) -> Self {
        Self(RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, VEC_ONE),
            Rect::from_min_size(min, VEC_ONE),
        ))
    }

    pub fn combine(outer: &Self, inner: &Self) -> Self {
        puffin_egui::puffin::profile_function!();
        let min = outer.0.to().min + (inner.0.to().min - inner.0.from().min) * outer.scale();

        Self(RectTransform::from_to(
            *outer.0.from(),
            Rect::from_min_size(min, outer.0.to().size() * inner.scale()),
        ))
    }

    pub fn scale(&self) -> Vec2 {
        self.0.scale()
    }

    pub fn transform_shape(&self, shape: &Shape) -> Shape {
        puffin_egui::puffin::profile_function!();
        let mut shape = shape.clone();
        TransformShape::transform(self.clone(), &mut shape);
        shape
    }

    pub fn translate_shape(&self, shape: &Shape) -> Shape {
        puffin_egui::puffin::profile_function!();
        let mut shape = shape.clone();
        TransformShape::transform(self.to_translate_only(), &mut shape);
        shape
    }

    pub fn transform_pos(&self, pos: Pos2) -> Pos2 {
        puffin_egui::puffin::profile_function!();
        self.0.transform_pos(pos)
    }

    pub fn transform_x(&self, x: f32) -> f32 {
        puffin_egui::puffin::profile_function!();
        self.transform_pos(Pos2::new(x, 0.0)).x
    }

    pub fn transform_y(&self, y: f32) -> f32 {
        puffin_egui::puffin::profile_function!();
        self.transform_pos(Pos2::new(0.0, y)).y
    }

    pub fn scale_vec(&self, vec: Vec2) -> Vec2 {
        vec * self.scale()
    }

    pub fn transform_rect(&self, rect: &Rect) -> Rect {
        puffin_egui::puffin::profile_function!();
        self.0.transform_rect(*rect)
    }

    pub fn transform_x_rangef(&self, range: &Rangef) -> Rangef {
        puffin_egui::puffin::profile_function!();
        self.0
            .transform_rect(Rect::from_x_y_ranges(*range, Rangef::NOTHING))
            .x_range()
    }

    pub fn transform_y_rangef(&self, range: &Rangef) -> Rangef {
        puffin_egui::puffin::profile_function!();
        self.0
            .transform_rect(Rect::from_x_y_ranges(Rangef::NOTHING, *range))
            .y_range()
    }

    pub fn resize_at(&self, delta: f32, point: Pos2) -> Self {
        puffin_egui::puffin::profile_function!();
        let from = *self.0.from();
        let to = Rect::from_min_size(self.0.to().min, self.0.from().size() * self.scale() * delta);
        let transformed_point = self.inverse().transform_pos(point);
        let resized = Self(RectTransform::from_to(from, to));
        let resized_point = resized.transform_pos(transformed_point);
        resized.translate(point - resized_point)
    }

    pub fn translate(&self, translate: Vec2) -> Self {
        puffin_egui::puffin::profile_function!();
        Self(RectTransform::from_to(
            *self.0.from(),
            self.0.to().translate(translate),
        ))
    }

    pub fn to_translate_only(&self) -> Self {
        puffin_egui::puffin::profile_function!();
        Self(RectTransform::from_to(
            *self.0.from(),
            Rect::from_min_size(self.0.to().min, self.0.from().size()),
        ))
    }

    pub fn inverse(&self) -> Self {
        puffin_egui::puffin::profile_function!();
        Self(self.0.inverse())
    }
}

pub struct TransformShape(Transform);

impl TransformShape {
    pub fn transform(transform: Transform, shape: &mut Shape) {
        let mut slf = TransformShape(transform);
        visit_shape(&mut slf, shape);
    }
}

impl ShapeVisitor for TransformShape {
    fn line_segment(
        &mut self,
        _index: &mut usize,
        points: &mut [Pos2; 2],
        _stroke: &mut Stroke,
    ) -> Option<()> {
        points
            .iter_mut()
            .for_each(|p| *p = self.0.transform_pos(*p));
        None
    }

    fn path(&mut self, _index: &mut usize, path: &mut PathShape) -> Option<()> {
        path.points
            .iter_mut()
            .for_each(|p| *p = self.0.transform_pos(*p));
        None
    }

    fn circle(&mut self, _index: &mut usize, c: &mut CircleShape) -> Option<()> {
        c.center = self.0.transform_pos(c.center);
        c.radius *= self.0.scale().x;
        None
    }

    fn rect(&mut self, _index: &mut usize, rect: &mut RectShape) -> Option<()> {
        let scale_factor = self.0.scale().length_sq();
        rect.rect.min = self.0.transform_pos(rect.rect.min);
        rect.rect.max = self.0.transform_pos(rect.rect.max);
        rect.rounding.ne *= scale_factor;
        rect.rounding.nw *= scale_factor;
        rect.rounding.se *= scale_factor;
        rect.rounding.sw *= scale_factor;
        None
    }

    fn text(&mut self, _index: &mut usize, text: &mut TextShape) -> Option<()> {
        text.pos = self.0.transform_pos(text.pos);
        None
    }

    fn mesh(&mut self, _index: &mut usize, mesh: &mut Mesh) -> Option<()> {
        mesh.vertices
            .iter_mut()
            .for_each(|v| v.pos = self.0.transform_pos(v.pos));
        None
    }

    fn quadratic_bezier(
        &mut self,
        _index: &mut usize,
        quad_bezier: &mut QuadraticBezierShape,
    ) -> Option<()> {
        quad_bezier
            .points
            .iter_mut()
            .for_each(|p| *p = self.0.transform_pos(*p));
        None
    }

    fn cubic_bezier(
        &mut self,
        _index: &mut usize,
        cubic_bezier: &mut CubicBezierShape,
    ) -> Option<()> {
        cubic_bezier
            .points
            .iter_mut()
            .for_each(|p| *p = self.0.transform_pos(*p));
        None
    }
}
