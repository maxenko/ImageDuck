use eframe::egui::{Ui, Pos2};

/// Wrapper around mouse
pub struct Mouse {
    pub pos: Pos2,
    last_left_click: Pos2,
    last_right_click: Pos2,
}

impl Mouse {
    pub fn new(ui: &mut Ui) -> Self {
        let mut mouse = Self {
            pos: Pos2::new(0.0, 0.0),
            last_left_click: Pos2::new(0.0, 0.0),
            last_right_click: Pos2::new(0.0, 0.0),
        };

        // Update the mouse position during initialization
        mouse.update_initial(ui);
        mouse
    }

    // Separate method for initial update
    fn update_initial(&mut self, ui: &mut Ui) {
        if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
            self.pos = pos;
        }
    }
}