use crate::gl;

pub enum Keycode {
    W,
    A,
    S,
    D,
    Other(u32),
}

impl From<u32> for Keycode {
    fn from(code: u32) -> Self {
        match code {
            gl::GLFW_KEY_W => Self::W,
            gl::GLFW_KEY_A => Self::A,
            gl::GLFW_KEY_S => Self::S,
            gl::GLFW_KEY_D => Self::D,
            _ => Self::Other(code),
        }
    }
}
