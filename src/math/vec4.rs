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

    fn add(self, Self { x, y, z, w }: Self) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            w: self.w + w,
        }
    }
}

impl<T> AddAssign for Vec4<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, Self { x, y, z, w }: Self) {
        self.x += x;
        self.y += y;
        self.z += z;
        self.w += w;
    }
}

impl<T> SubAssign for Vec4<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, Self { x, y, z, w }: Self) {
        self.x -= x;
        self.y -= y;
        self.z -= z;
        self.w -= w;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, Self { x, y, z, w }: Self) -> Self {
        Self {
            x: self.x - x,
            y: self.y - y,
            z: self.z - z,
            w: self.w - w,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec4<T> {
    type Output = Self;

    fn mul(self, Scalar(s): Scalar<T>) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec4<T>> for Scalar<T> {
    type Output = Vec4<T>;

    fn mul(self, Self::Output { x, y, z, w }: Self::Output) -> Self::Output {
        let Self(s) = self;
        Self::Output {
            x: s * x,
            y: s * y,
            z: s * z,
            w: s * w,
        }
    }
}

impl<T: Copy> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self::new(x, y, z, w)
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}
