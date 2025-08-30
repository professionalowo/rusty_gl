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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4_rgba() {
        let color = Vec4::rgba(1.0, 0.0, 0.0, 1.0);
        assert_eq!(color.x, 1.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
        assert_eq!(color.w, 1.0);
    }

    #[test]
    fn test_vec4_from_color4d() {
        let color = assimp::Color4D::new(1.0, 0.0, 0.0, 1.0);
        let vec4: Vec4<f32> = color.into();
        assert_eq!(vec4.x, 1.0);
        assert_eq!(vec4.y, 0.0);
        assert_eq!(vec4.z, 0.0);
        assert_eq!(vec4.w, 1.0);
    }

    #[test]
    fn test_vec4_to_color4d() {
        let vec4 = Vec4::rgba(1.0, 0.0, 0.0, 1.0);
        let color: assimp::Color4D = vec4.into();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }
}
