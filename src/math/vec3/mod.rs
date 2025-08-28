pub mod f32;

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
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

impl<T> AddAssign for Vec3<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
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
        let Self::Output { x, y, z } = other;
        Self::Output::new(scalar * x, scalar * y, scalar * z)
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self {
        let [x, y, z] = arr;
        Self::new(x, y, z)
    }
}

impl<T: Copy> From<(T, T, T)> for Vec3<T> {
    fn from(tup: (T, T, T)) -> Self {
        let (x, y, z) = tup;
        Self::new(x, y, z)
    }
}
