use std::{ffi::CString, fmt::Debug};

use crate::{
    gl::{glGetUniformLocation, glUniformMatrix3fv, program::Program},
    math::mat3::Mat3,
};

#[derive(Clone, Copy, Debug)]
pub struct UniformLocation(pub i32);

impl UniformLocation {
    pub fn try_for_program(
        program: &Program,
        name: impl AsRef<str>,
    ) -> Result<Self, UniformLocationError> {
        Self::try_get(program.id(), name)
    }

    fn try_get(program: u32, name: impl AsRef<str>) -> Result<Self, UniformLocationError> {
        get_location(program, name).map(UniformLocation)
    }

    pub fn mat3f(&self, transpose: bool, matrix: Mat3<f32>) {
        let cols = matrix.cols();
        let value = cols.as_ptr() as *const f32;
        unsafe {
            glUniformMatrix3fv(self.0, 1, u8::from(transpose), value);
        }
    }
}

//TODO: always seems to return -1, even if the uniform exists
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

pub enum UniformLocationError {
    UnusedUniform { program: u32, name: String },
    FFIError,
}

impl Debug for UniformLocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniformLocationError::UnusedUniform { program, name } => {
                write!(
                    f,
                    "UniformLocationError: program: {}, uniform: '{}'",
                    program, name
                )
            }
            UniformLocationError::FFIError => {
                write!(f, "UniformLocationError: FFIError")
            }
        }
    }
}
