use std::{ffi::CString, fmt::Debug};

pub mod uniform_trait;

use crate::gl::{glGetUniformLocation, program::Program, uniform::uniform_trait::Uniform};

#[derive(Clone, Copy, Debug)]
pub struct UniformLocation(pub i32);

impl UniformLocation {
    pub fn try_for_program<S>(program: &Program, name: S) -> Result<Self, UniformLocationError>
    where
        S: AsRef<str>,
    {
        Self::try_get(program.id(), name)
    }

    fn try_get<S>(program: u32, name: S) -> Result<Self, UniformLocationError>
    where
        S: AsRef<str>,
    {
        get_location(program, name).map(UniformLocation)
    }

    pub fn provide<U: Uniform>(location: &Self, value: U) {
        value.set(None, location);
    }

    pub fn provide_opt<U: Uniform>(location: &Self, value: U, options: U::Options) {
        value.set(Some(options), location);
    }
}

fn get_location(program: u32, name: impl AsRef<str>) -> Result<i32, UniformLocationError> {
    let name = name.as_ref();
    let name_cstr = CString::new(name).map_err(|_| UniformLocationError::FFIError)?;
    let loc = unsafe { glGetUniformLocation(program, name_cstr.as_ptr() as *const i8) };
    if loc == -1 {
        Err(UniformLocationError::UnusedUniform {
            program,
            name: name.to_string(),
        })
    } else {
        Ok(loc)
    }
}

#[derive(Debug)]
pub enum UniformLocationError {
    UnusedUniform { program: u32, name: String },
    FFIError,
}
