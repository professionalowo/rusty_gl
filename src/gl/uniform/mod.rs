use std::{ffi::CString, fmt::Debug};

use crate::{
    framework::textures::Texture2D,
    gl::{
        GLfloat, glGetUniformLocation, glUniform1f, glUniform1i, glUniform3f, glUniformMatrix3fv,
        glUniformMatrix4fv, program::Program,
    },
    math::{mat3::Mat3, mat4::Mat4, vec3::Vec3},
};

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

    #[deprecated]
    pub fn mat3f(&self, transpose: bool, matrix: Mat3<f32>) {
        let cols = matrix.cols();
        let value = cols.as_ptr() as *const f32;
        unsafe {
            glUniformMatrix3fv(self.0, 1, u8::from(transpose), value);
        }
    }

    #[deprecated]
    pub fn mat4f(&self, transpose: bool, matrix: Mat4<f32>) {
        let cols = matrix.cols();
        let value = cols.as_ptr() as *const f32;
        unsafe {
            glUniformMatrix4fv(self.0, 1, u8::from(transpose), value);
        }
    }

    #[deprecated]
    pub fn tex2d(&self, texture: &Texture2D, unit: u32) {
        texture.bind(unit);
        unsafe {
            glUniform1i(self.0, unit as i32);
        }
    }

    #[deprecated]
    pub fn vec3f(&self, vector: &Vec3<f32>) {
        let Vec3 { x, y, z } = *vector;
        unsafe {
            glUniform3f(self.0, x, y, z);
        }
    }

    #[deprecated]
    pub fn float32(&self, value: GLfloat) {
        unsafe {
            glUniform1f(self.0, value);
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

#[derive(Debug)]
pub enum UniformLocationError {
    UnusedUniform { program: u32, name: String },
    FFIError,
}
