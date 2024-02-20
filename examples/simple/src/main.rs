#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::epaint::CubicBezierShape;
use egui::{Color32, Context, Shape, Stroke};
use egui_shape_editor::shape_editor::style::Light;
use egui_shape_editor::shape_editor::ShapeEditorBuilder;

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
        Box::new(|_| Box::<App>::default()),
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
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let style = Light::default();
                ShapeEditorBuilder::new("Shape Editor".into(), &mut self.shape, &style)
                    .build()
                    .show(ui, ctx)
            });
        });
    }
}
