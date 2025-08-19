use crate::glfw;

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
            glfw::GLFW_PRESS => Self::Press,
            glfw::GLFW_RELEASE => Self::Release,
            glfw::GLFW_REPEAT => Self::Repeat,
            _ => Self::Unknown(code),
        }
    }
}
