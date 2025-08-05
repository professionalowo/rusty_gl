use std::ops::{Add, Mul, Neg, Sub};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub const fn z(&self) -> T {
        self.z
    }

    pub const fn data(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub const fn size() -> usize {
        3
    }
}

impl<T: Copy> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        let Scalar(scalar) = other;
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec3<T>> for Scalar<T> {
    type Output = Vec3<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        let Self(scalar) = self;
        Self::Output {
            x: scalar * other.x,
            y: scalar * other.y,
            z: scalar * other.z,
        }
    }
}
