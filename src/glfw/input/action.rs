use crate::gl;

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
            gl::GLFW_PRESS => Self::Press,
            gl::GLFW_RELEASE => Self::Release,
            gl::GLFW_REPEAT => Self::Repeat,
            _ => Self::Unknown(code),
        }
    }
}
