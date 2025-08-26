use std::{
    ffi,
    fmt::{self, Debug},
};

pub mod uniform_trait;

use crate::gl::{self, program::Program, uniform::uniform_trait::Uniform};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformLocation(pub i32);

impl UniformLocation {
    pub fn try_for_program<S>(program: &Program, name: S) -> Result<Self, UniformLocationError>
    where
        S: AsRef<str>,
    {
        let Program(id) = *program;
        get_location(id, name.as_ref()).map(UniformLocation)
    }

    pub fn provide<U>(location: &Self, value: U)
    where
        U: Uniform,
    {
        value.set(None, location);
    }

    pub fn provide_opt<U>(location: &Self, value: U, options: U::Options)
    where
        U: Uniform,
    {
        value.set(Some(options), location);
    }
}

fn get_location(program: u32, name: &str) -> Result<i32, UniformLocationError> {
    let name_cstr = ffi::CString::new(name)?;
    let name_ptr = name_cstr.as_ptr() as *const i8;
    let res = unsafe { gl::glGetUniformLocation(program, name_ptr) };

    gl::get_error()?;

    match res {
        -1 => Err(UniformLocationError::UnusedUniform {
            program,
            name: String::from(name),
        }),
        loc => Ok(loc),
    }
}

#[derive(Debug)]
pub enum UniformLocationError {
    UnusedUniform { program: u32, name: String },
    FFIError(ffi::NulError),
    GlError(gl::GLError),
}

impl fmt::Display for UniformLocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnusedUniform { name, program } => {
                write!(f, "Uniform {} unused in program {}", name, program)
            }
            Self::FFIError(e) => fmt::Display::fmt(e, f),
            Self::GlError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<gl::GLError> for UniformLocationError {
    fn from(value: gl::GLError) -> Self {
        Self::GlError(value)
    }
}

impl From<ffi::NulError> for UniformLocationError {
    fn from(value: ffi::NulError) -> Self {
        Self::FFIError(value)
    }
}
