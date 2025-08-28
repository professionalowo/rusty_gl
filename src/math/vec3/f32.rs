use std::ops::Div;

use crate::{
    gl::{
        self,
        uniform::{UniformLocation, uniform_trait::Uniform},
    },
    math::Scalar,
};

use super::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}
impl Vec3<f32> {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b)
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn rotate(&self, angle: f32, axis: &Vec3<f32>) -> Self {
        let theta = angle.to_radians();
        let c = theta.cos();
        let s = theta.sin();

        let axis = axis.normalize();
        let temp = axis * Scalar(1.0 - c);
        Self::new(
            self.x * c + temp.x * self.dot(&axis),
            self.y * c + temp.y * self.dot(&axis),
            self.z * c + temp.z * self.dot(&axis),
        ) + Self::cross(&axis, &self) * Scalar(s)
    }

    pub const fn cross(&self, other: &Self) -> Self {
        let Self { x, y, z } = *self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = *other;
        Self::new(y * oz - z * oy, z * ox - x * oz, x * oy - y * ox)
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length == 0.0 {
            Self::zero()
        } else {
            *self / length
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub const fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Div<f32> for Vec3<f32> {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 {
            self
        } else {
            Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
        }
    }
}

impl From<assimp::Vector3D> for Vec3<f32> {
    fn from(v: assimp::Vector3D) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Uniform for &Vec3<f32> {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, location: &UniformLocation) {
        let Vec3 { x, y, z } = self;
        let UniformLocation(location) = *location;
        unsafe {
            gl::glUniform3f(location, *x, *y, *z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a.cross(&b);
        assert_eq!(result, Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_vec3_normalize() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let length = a.length();
        let result = a.normalize();
        assert_eq!(result, Vec3::new(1.0 / length, 2.0 / length, 3.0 / length));
    }

    #[test]
    fn test_vec3_length() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let result = a.length();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a.dot(&b);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vec3_from_assimp() {
        let assimp_vec = assimp::Vector3D::new(1.0, 2.0, 3.0);
        let vec: Vec3<f32> = Vec3::from(assimp_vec);
        assert_eq!(vec, Vec3::new(1.0, 2.0, 3.0));
    }
}
