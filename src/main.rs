mod node;

use eframe::egui;
use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Image Duck",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    last_update: Instant,
    frame_count: u32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            last_update: Instant::now(),
            frame_count: 0,
        }
    }

    fn calculate_fps(&mut self) -> f64 {
        self.frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(self.last_update);
        if duration > Duration::from_secs(1) {
            self.last_update = now;
            self.frame_count = 0;
        }

        self.frame_count as f64 / duration.as_secs_f64()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame<>) {

        let fps = self.calculate_fps();

        let fps_text = if fps > 0.0 {
            format!("FPS: {:.2}", fps)
        } else {
            "Calculating FPS...".to_string()
        };

        egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::BLACK)).show(ctx, |ui| {
            ui.label(fps_text);
        });
    }
}
