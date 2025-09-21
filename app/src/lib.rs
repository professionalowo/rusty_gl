use std::ops::{Deref, DerefMut};

use gl_sys::{
    self,
    uniform::{UniformLocation, uniform_trait::Uniform},
};
use rmath::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

pub mod assimp;
pub mod framework;

#[derive(Debug, Clone)]
pub struct UniformWrapper<T>(pub T);

impl<T> From<T> for UniformWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for UniformWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for UniformWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Uniform for UniformWrapper<&Mat4<f32>> {
    type Options = bool; // transpose

    fn set(&self, options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        let transpose = options.unwrap_or(false);
        let cols = self.cols();
        let value = cols.as_ptr() as *const f32;
        unsafe {
            gl_sys::bindings::glUniformMatrix4fv(*location, 1, u8::from(transpose), value);
        }
    }
}

impl Uniform for UniformWrapper<&Vec4<f32>> {
    type Options = ();
    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        unsafe {
            gl_sys::bindings::glUniform4f(*location, self.x, self.y, self.z, self.w);
        }
    }
}

impl Uniform for UniformWrapper<&Vec3<f32>> {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        let Vec3 { x, y, z } = self.0;
        unsafe {
            gl_sys::bindings::glUniform3f(*location, *x, *y, *z);
        }
    }
}
