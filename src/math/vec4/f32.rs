use super::Vec4;
use crate::gl::{
    self,
    uniform::{UniformLocation, uniform_trait::Uniform},
};

impl From<assimp::Color4D> for Vec4<f32> {
    fn from(color: assimp::Color4D) -> Self {
        Self {
            x: color.r,
            y: color.g,
            z: color.b,
            w: color.a,
        }
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
