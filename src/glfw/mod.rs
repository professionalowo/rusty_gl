pub mod window;
pub mod input;

use crate::gl;

#[link(name = "glfw")]
#[allow(missing_abi)]
unsafe extern "C" {}

pub fn init() -> Result<(), GLFWError> {
    unsafe {
        let code = gl::glfwInit();
        if code != gl::GLFW_TRUE as i32 {
            return Err(GLFWError::from(code));
        }
    }
    Ok(())
}

pub fn terminate() {
    unsafe {
        gl::glfwTerminate();
    }
}

pub fn window_hint(target: u32, hint: u32) {
    unsafe {
        gl::glfwWindowHint(target.try_into().unwrap(), hint.try_into().unwrap());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLFWError {
    NoError,
    NotInitialized,
    NoCurrentContext,
    InvalidEnum,
    InvalidValue,
    OutOfMemory,
    ApiNotAvailable,
    VersionUnavailable,
    PlatformError,
    FormatUnavailable,
    NoWindowContext,
    UnknownError(i32),
}

impl GLFWError {
    pub fn new(code: i32) -> Self {
        match code {
            0 => Self::NoError,
            0x00010001 => Self::NotInitialized,
            0x00010002 => Self::NoCurrentContext,
            0x00010003 => Self::InvalidEnum,
            0x00010004 => Self::InvalidValue,
            0x00010005 => Self::OutOfMemory,
            0x00010006 => Self::ApiNotAvailable,
            0x00010007 => Self::VersionUnavailable,
            0x00010008 => Self::PlatformError,
            0x00010009 => Self::FormatUnavailable,
            0x0001000A => Self::NoWindowContext,
            _ => Self::UnknownError(code),
        }
    }
}

impl From<i32> for GLFWError {
    fn from(code: i32) -> Self {
        Self::new(code)
    }
}
