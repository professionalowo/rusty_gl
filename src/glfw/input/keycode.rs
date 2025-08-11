use crate::gl;

#[derive(Debug)]
pub enum Keycode {
    W,
    A,
    S,
    D,
    SHIFT,
    Other(u32),
}

impl From<u32> for Keycode {
    fn from(code: u32) -> Self {
        match code {
            gl::GLFW_KEY_W => Self::W,
            gl::GLFW_KEY_A => Self::A,
            gl::GLFW_KEY_S => Self::S,
            gl::GLFW_KEY_D => Self::D,
            gl::GLFW_KEY_LEFT_SHIFT | gl::GLFW_KEY_RIGHT_SHIFT => Self::SHIFT,
            _ => Self::Other(code),
        }
    }
}
