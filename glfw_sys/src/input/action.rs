use crate::bindings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Press,
    Release,
    Repeat,
    Unknown(u32),
}

impl From<u32> for Action {
    fn from(code: u32) -> Self {
        match code {
            bindings::GLFW_PRESS => Self::Press,
            bindings::GLFW_RELEASE => Self::Release,
            bindings::GLFW_REPEAT => Self::Repeat,
            _ => Self::Unknown(code),
        }
    }
}
