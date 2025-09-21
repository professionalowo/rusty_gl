use crate::bindings;

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
            bindings::GLFW_KEY_W => Self::W,
            bindings::GLFW_KEY_A => Self::A,
            bindings::GLFW_KEY_S => Self::S,
            bindings::GLFW_KEY_D => Self::D,
            bindings::GLFW_KEY_ESCAPE => Self::Escape,
            bindings::GLFW_KEY_LEFT_SHIFT | bindings::GLFW_KEY_RIGHT_SHIFT => Self::Shift,
            _ => Self::Other(code),
        }
    }
}
