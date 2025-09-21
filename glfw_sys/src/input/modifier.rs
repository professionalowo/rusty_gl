use crate::bindings;

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
            bindings::GLFW_MOD_SHIFT => Self::Shift,
            bindings::GLFW_MOD_CONTROL => Self::Control,
            bindings::GLFW_MOD_ALT => Self::Alt,
            bindings::GLFW_MOD_SUPER => Self::Super,
            bindings::GLFW_MOD_CAPS_LOCK => Self::CapsLock,
            bindings::GLFW_MOD_NUM_LOCK => Self::NumLock,
            _ => Self::Unknown(code),
        }
    }
}
