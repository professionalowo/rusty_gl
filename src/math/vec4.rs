use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub struct Vec4<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub const fn data(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub const fn size() -> usize {
        4
    }
}

impl<T: Copy> Neg for Vec4<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> AddAssign for Vec4<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T> SubAssign for Vec4<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec4<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        let Scalar(scalar) = other;
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec4<T>> for Scalar<T> {
    type Output = Vec4<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        let Self(scalar) = self;
        Self::Output {
            x: scalar * other.x,
            y: scalar * other.y,
            z: scalar * other.z,
            w: scalar * other.w,
        }
    }
}

impl<T: Copy> From<(T, T, T, T)> for Vec4<T> {
    fn from(tup: (T, T, T, T)) -> Self {
        let (x, y, z, w) = tup;
        Self::new(x, y, z, w)
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self {
        let [x, y, z, w] = arr;
        Self::new(x, y, z, w)
    }
}
