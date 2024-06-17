#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::epaint::{Color32, CubicBezierShape, Shape, Stroke};
use egui::panel::TopBottomSide;
use egui::{Context, DragValue, Response, Rounding, Style, Ui, Visuals, Widget, WidgetText};
use egui_shape_editor::shape_editor::style::Light;
use egui_shape_editor::shape_editor::{
    ParamType, ParamValue, ShapeEditorBuilder, ShapeEditorOptions,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{BitOrAssign, RangeInclusive};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Shape Editor Simple Example",
        options,
        Box::new(|creation_context| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            creation_context.egui_ctx.set_style(style);

            Box::<App>::default()
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                {
                    use base64::prelude::*;
                    let mut app = App::default();
                    if let Some(hash_string) =
                        web_sys::window().and_then(|window| window.location().hash().ok())
                    {
                        if hash_string.starts_with("#") {
                            match BASE64_STANDARD.decode(&hash_string[1..]) {
                                Ok(decoded) => {
                                    match rmp_serde::from_slice::<ShapeWrapper>(&decoded) {
                                        Ok(ShapeWrapper(shape)) => app.shape = shape,
                                        Err(err) => {
                                            log::warn!("Cannot deserialize Shape: {:}", err)
                                        }
                                    }
                                }
                                Err(err) => log::warn!("Cannot deserialize Shape: {}", err),
                            }
                        }
                    }
                    Box::new(|_| Box::new(app))
                },
            )
            .await
            .expect("failed to start eframe");
    });
}

pub struct App {
    shape: Shape,
    options: ShapeEditorOptions,
}

impl Default for App {
    fn default() -> Self {
        Self {
            shape: Shape::Noop,
            options: ShapeEditorOptions::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            puffin_egui::puffin::profile_function!();

            egui::TopBottomPanel::new(TopBottomSide::Bottom, "Bottom panel").show(ctx, |ui| {
                let mut profile = puffin_egui::puffin::are_scopes_on();
                ui.checkbox(&mut profile, "Show profiler window");
                puffin_egui::puffin::set_scopes_on(profile); // controls both the profile capturing, and the displaying of it
                if profile {
                    puffin_egui::profiler_window(ctx);
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            use base64::prelude::*;
            use web_time::Instant;
            static mut LAST_SERIALIZED: Option<Instant> = None;

            unsafe {
                if LAST_SERIALIZED.is_none()
                    || LAST_SERIALIZED
                        .is_some_and(|time| time.elapsed() > std::time::Duration::from_secs(1))
                {
                    if let Ok(serialized) = rmp_serde::to_vec(&ShapeWrapper(self.shape.clone())) {
                        if let Some(window) = web_sys::window() {
                            let _ = window
                                .location()
                                .set_hash(&BASE64_STANDARD.encode(serialized));
                        } else {
                            log::warn!("Cannot serialize Shape, browser window is unreachable");
                        }
                    }
                    LAST_SERIALIZED = Some(Instant::now());
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let style = Light::default();
            let mut editor =
                ShapeEditorBuilder::new("Shape Editor".into(), &mut self.shape, &style)
                    .options(self.options.clone())
                    .build();

            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    let options = &mut self.options;
                    ui.set_width(100.0);
                    egui::stroke_ui(ui, &mut options.stroke, "Stroke");
                    ui.separator();
                    ui.checkbox(&mut options.snap_enabled_by_default, "Snap enabled");
                    ui.add_enabled_ui(options.snap_enabled_by_default, |ui| {
                        ui.add(DragValue::new(&mut options.snap_distance).clamp_range(0..=100));
                    });
                    ui.separator();
                    ui.label("Parameters:");
                    let params = editor.selection_shapes_params(ctx);
                    let mut common_params = params.common();
                    let mut changed = false;
                    for (ty, val) in &mut common_params {
                        ui.horizontal(|ui| match ty {
                            ParamType::StrokeColor => changed.bitor_assign(
                                ui.add(color_param_widget(val, options.stroke.color, "Color: "))
                                    .changed(),
                            ),
                            ParamType::StrokeWidth => changed.bitor_assign(
                                ui.add(float_param_widget(
                                    val,
                                    options.stroke.width,
                                    0.0..=50.0,
                                    "Width: ",
                                ))
                                .changed(),
                            ),
                            ParamType::Rounding => changed.bitor_assign(
                                ui.add(rounding_param_widget(
                                    val,
                                    Rounding::ZERO,
                                    0.0..=50.0,
                                    "Rounding: ",
                                ))
                                .changed(),
                            ),
                            ParamType::FillColor => changed.bitor_assign(
                                ui.add(color_param_widget(
                                    val,
                                    options.stroke.color,
                                    "Fill Color: ",
                                ))
                                .changed(),
                            ),
                            ParamType::ClosedShape => changed.bitor_assign(
                                ui.add(boolean_param_widget(val, false, "Closed: "))
                                    .changed(),
                            ),
                            ParamType::Radius => changed.bitor_assign(
                                ui.add(float_param_widget(val, 50.0, 0.0..=10000.0, "Radius: "))
                                    .changed(),
                            ),
                            ParamType::Texture => {}
                        });
                    }
                    if changed {
                        editor.apply_common_shapes_params(
                            ctx,
                            common_params
                                .into_iter()
                                .filter_map(|(ty, val)| val.map(|val| (ty, val)))
                                .collect(),
                        );
                    }
                });
                ui.separator();
                ui.vertical(|ui| editor.show(ui, ctx));
            });
        });
    }
}

fn color_param_widget<'a, L: Into<WidgetText> + 'a>(
    value: &'a mut Option<ParamValue>,
    default: Color32,
    label: L,
) -> impl Widget + 'a {
    move |ui: &mut Ui| -> Response {
        let mut enabled = value.is_some();
        let mut color = if let Some(ParamValue::Color(color)) = value {
            *color
        } else {
            default
        };
        let mut response = ui.checkbox(&mut enabled, "");
        ui.label(label);
        ui.add_enabled_ui(enabled, |ui| {
            response.bitor_assign(ui.color_edit_button_srgba(&mut color));
        });
        *value = enabled.then(|| ParamValue::Color(color));
        response
    }
}

fn float_param_widget<'a, L: Into<WidgetText> + 'a>(
    value: &'a mut Option<ParamValue>,
    default: f32,
    range: RangeInclusive<f32>,
    label: L,
) -> impl Widget + 'a {
    move |ui: &mut Ui| -> Response {
        let mut enabled = value.is_some();
        let mut float = if let Some(ParamValue::Float(float)) = value {
            float.into_inner()
        } else {
            default
        };
        let mut response = ui.checkbox(&mut enabled, "");
        ui.label(label);
        ui.add_enabled_ui(enabled, |ui| {
            response.bitor_assign(ui.add(DragValue::new(&mut float).clamp_range(range)));
        });
        *value = enabled.then(|| ParamValue::Float(float.try_into().unwrap_or_default()));
        response
    }
}

fn boolean_param_widget<'a, L: Into<WidgetText> + 'a>(
    value: &'a mut Option<ParamValue>,
    default: bool,
    label: L,
) -> impl Widget + 'a {
    move |ui: &mut Ui| -> Response {
        let mut enabled = value.is_some();
        let mut boolean = if let Some(ParamValue::Boolean(boolean)) = value {
            *boolean
        } else {
            default
        };
        let mut response = ui.checkbox(&mut enabled, "");
        ui.label(label);
        ui.add_enabled_ui(enabled, |ui| {
            response.bitor_assign(ui.checkbox(&mut boolean, ""));
        });
        *value = enabled.then(|| ParamValue::Boolean(boolean));
        response
    }
}

fn rounding_param_widget<'a, L: Into<WidgetText> + 'a>(
    value: &'a mut Option<ParamValue>,
    default: Rounding,
    range: RangeInclusive<f32>,
    label: L,
) -> impl Widget + 'a {
    move |ui: &mut Ui| -> Response {
        let mut enabled = value.is_some();
        let mut rounding = if let Some(ParamValue::Rounding(rounding)) = value {
            *rounding
        } else {
            default
        };
        let mut response = ui.checkbox(&mut enabled, "");
        ui.label(label);
        ui.add_enabled_ui(enabled, |ui| {
            ui.vertical(|ui| {
                response.bitor_assign(
                    ui.add(
                        DragValue::new(&mut rounding.nw)
                            .prefix("NW: ")
                            .clamp_range(range.clone()),
                    ),
                );
                response.bitor_assign(
                    ui.add(
                        DragValue::new(&mut rounding.ne)
                            .prefix("NE: ")
                            .clamp_range(range.clone()),
                    ),
                );
                response.bitor_assign(
                    ui.add(
                        DragValue::new(&mut rounding.se)
                            .prefix("SE: ")
                            .clamp_range(range.clone()),
                    ),
                );
                response.bitor_assign(
                    ui.add(
                        DragValue::new(&mut rounding.sw)
                            .prefix("SW: ")
                            .clamp_range(range),
                    ),
                );
            });
        });
        *value = enabled.then(|| ParamValue::Rounding(rounding));
        response
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "egui::epaint::Shape")]
enum ShapeAdapter {
    Noop,
    Vec(
        #[serde(
            serialize_with = "serialize_vec_shape",
            deserialize_with = "deserialize_vec_shape"
        )]
        Vec<Shape>,
    ),
    Circle(egui::epaint::CircleShape),
    Ellipse(egui::epaint::EllipseShape),
    LineSegment {
        points: [egui::Pos2; 2],
        stroke: Stroke,
    },
    Path(egui::epaint::PathShape),
    Rect(egui::epaint::RectShape),
    Text(egui::epaint::TextShape),
    Mesh(egui::epaint::Mesh),
    QuadraticBezier(egui::epaint::QuadraticBezierShape),
    CubicBezier(CubicBezierShape),
    #[serde(skip)]
    Callback(egui::epaint::PaintCallback),
}

#[derive(Serialize, Deserialize)]
struct ShapeWrapper(#[serde(with = "ShapeAdapter")] Shape);

fn serialize_vec_shape<S>(val: &Vec<Shape>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<ShapeWrapper> = val.iter().map(|s| ShapeWrapper(s.clone())).collect();
    vec.serialize(serializer)
}

fn deserialize_vec_shape<'de, D>(deserializer: D) -> Result<Vec<Shape>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|ShapeWrapper(a)| a).collect())
}
