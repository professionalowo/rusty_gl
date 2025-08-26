use std::{ffi::CString, fmt::Debug};

pub mod uniform_trait;

use crate::gl::{glGetUniformLocation, program::Program, uniform::uniform_trait::Uniform};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformLocation(pub i32);

impl UniformLocation {
    pub fn try_for_program<S>(program: &Program, name: S) -> Result<Self, UniformLocationError>
    where
        S: AsRef<str>,
    {
        let Program(id) = *program;
        get_location(id, name).map(UniformLocation)
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
