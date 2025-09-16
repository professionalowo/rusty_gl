use std::ops::Div;

use super::Vec3;
use crate::{
    gl::{
        self,
        uniform::{UniformLocation, uniform_trait::Uniform},
    },
    math::Scalar,
};

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

    pub const fn scalar(s: f32) -> Self {
        Self::new(s, s, s)
    }

    pub fn rotate(&self, angle: f32, axis: &Vec3<f32>) -> Self {
        let theta = angle.to_radians();
        let c = theta.cos();
        let s = Scalar(theta.sin());

        let axis = axis.normalize();
        let Self {
            x: tx,
            y: ty,
            z: tz,
        } = axis * Scalar(1.0 - c);
        let Self { x, y, z } = *self;
        Self::new(
            x * c + tx * self.dot(&axis),
            y * c + ty * self.dot(&axis),
            z * c + tz * self.dot(&axis),
        ) + axis.cross(&self) * s
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
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub const fn dot(&self, other: &Self) -> f32 {
        let Self { x, y, z } = *self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = *other;
        x * ox + y * oy + z * oz
    }

    pub const fn min(
        Self { x, y, z }: Self,
        Self {
            x: ox,
            y: oy,
            z: oz,
        }: Self,
    ) -> Self {
        Self::new(x.min(ox), y.min(oy), z.min(oz))
    }

    pub const fn max(
        Self { x, y, z }: Self,
        Self {
            x: ox,
            y: oy,
            z: oz,
        }: Self,
    ) -> Self {
        Self::new(x.max(ox), y.max(oy), z.max(oz))
    }

    pub fn pow<B, E>(base: B, exp: E) -> Self
    where
        B: Into<Self>,
        E: Into<Self>,
    {
        let Self { x, y, z }: Self = base.into();
        let b: Self = exp.into();
        Self::new(x.powf(b.x), y.powf(b.y), z.powf(b.z))
    }

    pub const fn cmin(Self { x, y, z }: Self) -> f32 {
        x.min(y).min(z)
    }

    pub const fn cmax(Self { x, y, z }: Self) -> f32 {
        x.max(y).max(z)
    }
}

impl Div<f32> for Vec3<f32> {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        if s == 0.0 {
            self
        } else {
            let Self { x, y, z } = self;
            Self::new(x / s, y / s, z / s)
        }
    }
}

impl From<assimp::Vector3D> for Vec3<f32> {
    fn from(v: assimp::Vector3D) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec3<f32>> for assimp::Vector3D {
    fn from(Vec3 { x, y, z }: Vec3<f32>) -> Self {
        Self::new(x, y, z)
    }
}

impl From<assimp_sys::AiVector3D> for Vec3<f32> {
    fn from(assimp_sys::AiVector3D { x, y, z }: assimp_sys::AiVector3D) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vec3<f32>> for assimp_sys::AiVector3D {
    fn from(Vec3 { x, y, z }: Vec3<f32>) -> Self {
        Self { x, y, z }
    }
}

impl From<assimp::Color3D> for Vec3<f32> {
    fn from(c: assimp::Color3D) -> Self {
        Self::rgb(c.r, c.g, c.b)
    }
}

impl Uniform for &Vec3<f32> {
    type Options = ();

    fn set(&self, _options: Option<Self::Options>, UniformLocation(location): &UniformLocation) {
        let Vec3 { x, y, z } = self;
        unsafe {
            gl::glUniform3f(*location, *x, *y, *z);
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
        assert_eq!(a.length(), 1.0);
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn test_vec3_from_assimp() {
        let assimp_vec = assimp::Vector3D::new(1.0, 2.0, 3.0);
        let vec: Vec3<f32> = Vec3::from(assimp_vec);
        assert_eq!(vec, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_max() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::max(a, b), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_vec3_min() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::min(a, b), Vec3::new(1.0, 2.0, 3.0));
    }
}
