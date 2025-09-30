use crate::get_time;

#[derive(Debug)]
pub struct Timer<const FPS: u8> {
    now: f64,
    last_frame: f64,
}

impl<const FPS: u8> Timer<FPS> {

	#[inline]
    pub fn new() -> Self {
        Self::default()
    }

	#[inline]
    pub fn start(&mut self) {
        self.now = get_time();
    }

	#[inline]
    pub fn should_render(&self) -> bool {
        self.now - self.last_frame >= 1.0 / FPS as f64
    }

	#[inline]
    pub fn rendered(&mut self) {
        self.last_frame = self.now;
    }
}

impl<const FPS: u8> Default for Timer<FPS> {
	#[inline]
    fn default() -> Self {
        Self {
            last_frame: 0.0,
            now: 0.0,
        }
    }
}
