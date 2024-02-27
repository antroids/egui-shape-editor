#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::epaint::CubicBezierShape;
use egui::panel::TopBottomSide;
use egui::{Color32, Context, DragValue, Shape, Stroke, Style, Visuals};
use egui_shape_editor::shape_editor::style::Light;
use egui_shape_editor::shape_editor::{ShapeEditorBuilder, ShapeEditorOptions};
use std::convert::Into;

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

// When compiling to web using trunk:
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
                Box::new(|_| Box::<App>::default()),
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
            shape: Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                [
                    [5.0, 5.0].into(),
                    [50.0, 200.0].into(),
                    [150.0, 200.0].into(),
                    [300.0, 300.0].into(),
                ],
                false,
                Color32::TRANSPARENT,
                Stroke::new(5.0, Color32::GREEN),
            )),
            options: ShapeEditorOptions::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "Bottom panel").show(ctx, |ui| {
            let mut profile = puffin_egui::puffin::are_scopes_on();
            ui.checkbox(&mut profile, "Show profiler window");
            puffin_egui::puffin::set_scopes_on(profile); // controls both the profile capturing, and the displaying of it
            if profile {
                puffin_egui::profiler_window(ctx);
            }
        });
        puffin_egui::puffin::profile_function!();
        egui::CentralPanel::default().show(ctx, |ui| {
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
                });
                ui.separator();
                ui.vertical(|ui| {
                    let style = Light::default();
                    let editor =
                        ShapeEditorBuilder::new("Shape Editor".into(), &mut self.shape, &style)
                            .options(self.options.clone())
                            .build();
                    editor.show(ui, ctx)
                });
            });
        });
    }
}
