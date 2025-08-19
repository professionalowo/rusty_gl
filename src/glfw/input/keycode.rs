use crate::glfw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keycode {
    W,
    A,
    S,
    D,
    Shift,
    Escape,
    Other(u32),
}

impl From<u32> for Keycode {
    fn from(code: u32) -> Self {
        match code {
            glfw::GLFW_KEY_W => Self::W,
            glfw::GLFW_KEY_A => Self::A,
            glfw::GLFW_KEY_S => Self::S,
            glfw::GLFW_KEY_D => Self::D,
            glfw::GLFW_KEY_ESCAPE => Self::Escape,
            glfw::GLFW_KEY_LEFT_SHIFT | glfw::GLFW_KEY_RIGHT_SHIFT => Self::Shift,
            _ => Self::Other(code),
        }
    }
}
