use crate::get_time;

#[derive(Debug)]
pub struct Timer<const FPS: u32> {
    now: f64,
    last_frame: f64,
}

impl<const FPS: u32> Timer<FPS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.now = get_time();
    }

    pub fn should_render(&self) -> bool {
        self.now - self.last_frame >= 1.0 / FPS as f64
    }

    pub fn rendered(&mut self) {
        self.last_frame = self.now;
    }
}

impl<const FPS: u32> Default for Timer<FPS> {
    fn default() -> Self {
        Self {
            last_frame: 0.0,
            now: 0.0,
        }
    }
}
