use egui::{Rangef, Rect};

pub fn grid_step(scale: f32) -> f32 {
    50f32 * 5f32.powi(-scale.log(5.0).round() as i32)
}

pub fn step_by(range: Rangef, step: f32) -> impl Iterator<Item = f32> {
    let min = (range.min / step).floor() as i32;
    let max = (range.max / step).ceil() as i32;
    (min..max).map(move |i| i as f32 * step)
}

pub fn normalize_rect(rect: &Rect) -> Rect {
    let mut rect = *rect;
    if rect.left() > rect.right() {
        let temp = rect.left();
        *rect.left_mut() = rect.right();
        *rect.right_mut() = temp;
    }
    if rect.top() > rect.bottom() {
        let temp = rect.top();
        *rect.top_mut() = rect.bottom();
        *rect.bottom_mut() = temp;
    }
    rect
}
