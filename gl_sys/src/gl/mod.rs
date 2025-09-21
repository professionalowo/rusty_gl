#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fmt;

include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

pub mod program;
pub mod shader;
pub mod uniform;
pub mod vao;
pub mod vbo;

pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe { glDrawArrays(mode, first, count) };
}

pub fn draw_elements(mode: u32, count: i32, element_type: u32) {
    unsafe { glDrawElements(mode, count, element_type, std::ptr::null()) };
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha) };
}

pub fn clear(mask: u32) {
    unsafe { glClear(mask) };
}

pub fn enable(cap: GLenum) {
    unsafe { glEnable(cap) };
}

pub fn get_error() -> Result<(), GLError> {
    let err = unsafe { glGetError() };
    if err == GL_NO_ERROR {
        return Ok(());
    }
    Err(GLError::from(err))
}

#[derive(Debug)]
pub enum GLError {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    OutOfMemory,
    Other(u32),
}

impl From<u32> for GLError {
    fn from(code: u32) -> Self {
        match code {
            GL_NO_ERROR => GLError::NoError,
            GL_INVALID_ENUM => GLError::InvalidEnum,
            GL_INVALID_VALUE => GLError::InvalidValue,
            GL_INVALID_OPERATION => GLError::InvalidOperation,
            GL_OUT_OF_MEMORY => GLError::OutOfMemory,
            _ => GLError::Other(code),
        }
    }
}

impl fmt::Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoError => write!(f, "No Error"),
            Self::InvalidEnum => write!(f, "Invalid Enum"),
            Self::InvalidOperation => write!(f, "Invalid Operation"),
            Self::InvalidValue => write!(f, "Invalid Value"),
            Self::OutOfMemory => write!(f, "Out of memory"),
            Self::Other(c) => write!(f, "Other error: {}", c),
        }
    }
}
