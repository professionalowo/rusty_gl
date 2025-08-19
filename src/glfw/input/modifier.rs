use crate::glfw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
    None,
    Shift,
    Control,
    Alt,
    Super,
    CapsLock,
    NumLock,
    Unknown(u32),
}

impl From<u32> for Modifier {
    fn from(code: u32) -> Self {
        match code {
            0 => Self::None,
            glfw::GLFW_MOD_SHIFT => Self::Shift,
            glfw::GLFW_MOD_CONTROL => Self::Control,
            glfw::GLFW_MOD_ALT => Self::Alt,
            glfw::GLFW_MOD_SUPER => Self::Super,
            glfw::GLFW_MOD_CAPS_LOCK => Self::CapsLock,
            glfw::GLFW_MOD_NUM_LOCK => Self::NumLock,
            _ => Self::Unknown(code),
        }
    }
}
