use egui::ahash::{HashMap, HashSet};
use egui::{Rangef, Rect};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;

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

pub fn map_grouped_by<'a, T, K, V, F, I>(iter: I, mut key_value_fn: F) -> HashMap<K, HashSet<V>>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
    F: FnMut(&T) -> (K, V),
    K: PartialEq + Eq + Hash,
    V: PartialEq + Eq + Hash,
{
    let mut grouped: HashMap<K, HashSet<V>> = Default::default();
    for item in iter {
        let (key, value) = key_value_fn(item);
        grouped.entry(key).or_default().insert(value);
    }
    grouped
}

pub fn b_tree_map_grouped_by<'a, T, K, V, F, I>(
    iter: I,
    mut key_value_fn: F,
) -> BTreeMap<K, BTreeSet<V>>
where
    T: 'a,
    I: Iterator<Item = &'a T>,
    F: FnMut(&T) -> (K, V),
    K: PartialEq + Eq + Hash + Ord,
    V: PartialEq + Eq + Hash + Ord,
{
    let mut grouped: BTreeMap<K, BTreeSet<V>> = Default::default();
    for item in iter {
        let (key, value) = key_value_fn(item);
        grouped.entry(key).or_default().insert(value);
    }
    grouped
}
