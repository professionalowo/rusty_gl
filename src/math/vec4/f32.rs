use super::Vec4;
use crate::gl::{
    self,
    uniform::{UniformLocation, uniform_trait::Uniform},
};

impl Vec4<f32> {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<assimp::Color4D> for Vec4<f32> {
    fn from(c: assimp::Color4D) -> Self {
        Self::rgba(c.r, c.g, c.b, c.a)
    }
}

impl From<Vec4<f32>> for assimp::Color4D {
    fn from(Vec4 { x, y, z, w }: Vec4<f32>) -> Self {
        Self::new(x, y, z, w)
    }
}

impl Uniform for &Vec4<f32> {
    type Options = ();
    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        unsafe {
            gl::glUniform4f(*location, self.x, self.y, self.z, self.w);
        }
    }
}
