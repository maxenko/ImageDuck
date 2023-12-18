mod node;
mod diag;
mod mouse;

use eframe::egui;
use eframe::egui::{Pos2,};

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Image Duck",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    diag: diag::FPS,
    zoom: f32,
    scroll: egui::Vec2,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            diag: diag::FPS::new(),
            zoom: 1.0,
            scroll: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame<>) {

        let fps = self.diag.calculate_fps();
        let background_color = egui::Color32::BLACK;

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(background_color)
            )
            .show(ctx, |ui| {

                ctx.set_pixels_per_point(2.0 * 2.0);

                // add ui elements here
                //ui.heading("Image Duck");
                ui.label(self.diag.to_string());
                let mut mouse = mouse::Mouse::new(ui);
                let pos = mouse.pos;

                ui.label(format!("Pointer position: ({:.1}, {:.1})", pos.x, pos.y));

                let mut scroll_area = egui::ScrollArea::new([false; 2]);

                scroll_area.show(ui, |ui| {

                    let scroll_delta = ui.input(|i| i.scroll_delta);

                    self.zoom *= (1.0 + scroll_delta.y * 0.001).max(0.01);
                    self.scroll += scroll_delta;

                    // Set a large virtual space to simulate an infinite canvas
                    let size = egui::vec2(10000.0, 10000.0) * self.zoom;
                    let (_id, rect) = ui.allocate_space(size);

                    // Example content: Draw a circle at the center
                    let center = rect.center();
                    ui.painter().circle(Pos2::new(0.0,0.0), 50.0 * self.zoom, egui::Color32::WHITE, (1.0, egui::Color32::RED));

                });
            });
    }
}