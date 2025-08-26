use crate::glfw;

#[derive(Debug)]
pub struct Timer<const FPS: u32> {
    now: f64,
    last_frame: f64,
}

impl<const FPS: u32> Timer<FPS> {
    pub fn new() -> Self {
        Self {
            now: 0.0,
            last_frame: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.now = glfw::get_time();
    }

    pub fn should_render(&self) -> bool {
        self.now - self.last_frame >= 1.0 / FPS as f64
    }

    pub fn rendered(&mut self) {
        self.last_frame = self.now;
    }
}
