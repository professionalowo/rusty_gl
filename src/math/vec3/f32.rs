use std::ops::Div;

use crate::math::Scalar;

use super::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}
impl Vec3<f32> {
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
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
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
