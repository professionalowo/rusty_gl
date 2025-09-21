use std::{
    ffi,
    fmt::{self, Debug},
};

pub mod uniform_trait;

use crate::{
    GLError,
    bindings::{GLint, glGetUniformLocation},
    get_error,
    program::Program,
    uniform::uniform_trait::Uniform,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformLocation(pub GLint);

impl UniformLocation {
    pub fn try_for_program<S>(Program(id): &Program, name: S) -> Result<Self, UniformLocationError>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();
        let name_cstr = ffi::CString::new(name)?;
        let name_ptr = name_cstr.as_ptr() as *const i8;

        let res = unsafe { glGetUniformLocation(*id, name_ptr) };

        get_error()?;

        match res {
            -1 => Err(UniformLocationError::UnusedUniform {
                id: *id,
                name: String::from(name),
            }),
            loc => Ok(Self(loc)),
        }
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

#[derive(Debug)]
pub enum UniformLocationError {
    UnusedUniform { id: u32, name: String },
    FFIError(ffi::NulError),
    GlError(GLError),
}

impl fmt::Display for UniformLocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnusedUniform { name, id } => {
                write!(f, "Uniform {} unused in program with id {}", name, id)
            }
            Self::FFIError(e) => fmt::Display::fmt(e, f),
            Self::GlError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<GLError> for UniformLocationError {
    fn from(value: GLError) -> Self {
        Self::GlError(value)
    }
}

impl From<ffi::NulError> for UniformLocationError {
    fn from(value: ffi::NulError) -> Self {
        Self::FFIError(value)
    }
}
