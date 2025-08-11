use crate::gl;

#[derive(Debug)]
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
            gl::GLFW_MOD_SHIFT => Self::Shift,
            gl::GLFW_MOD_CONTROL => Self::Control,
            gl::GLFW_MOD_ALT => Self::Alt,
            gl::GLFW_MOD_SUPER => Self::Super,
            gl::GLFW_MOD_CAPS_LOCK => Self::CapsLock,
            gl::GLFW_MOD_NUM_LOCK => Self::NumLock,
            _ => Self::Unknown(code),
        }
    }
}
