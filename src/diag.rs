use std::time::{Duration, Instant};

pub struct FPS {
    last_update: Instant,
    frame_count: u32,
    last_fps: f64,
}

impl  FPS {

    // default values
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            frame_count: 0,
            last_fps: 0.0,
        }
    }

    pub fn calculate_fps(&mut self) -> f64 {
        self.frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(self.last_update);
        if duration > Duration::from_secs(1) {
            self.last_update = now;
            self.frame_count = 0;
        }

        self.last_fps = self.frame_count as f64 / duration.as_secs_f64();
        self.last_fps
    }

    pub fn to_string(&self) -> String {
        format!("FPS: {:.2}", self.last_fps)
    }
}