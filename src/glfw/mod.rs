use std::ffi::c_int;

pub mod window;

const GLFW_TRUE: c_int = 1;

#[link(name = "glfw")]
#[allow(missing_abi)]
unsafe extern "C" {}

unsafe extern "C" {
    unsafe fn glfwInit() -> c_int;
    unsafe fn glfwTerminate();
}

pub fn init() -> Result<(), GLFWError> {
    unsafe {
        let code = glfwInit();
        if code != GLFW_TRUE {
            return Err(GLFWError::from(code));
        }
    }
    Ok(())
}

pub fn terminate() {
    unsafe {
        glfwTerminate();
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
