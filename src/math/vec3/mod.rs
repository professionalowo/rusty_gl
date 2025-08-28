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
        let Self { x, y, z } = other;
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        let Self { x, y, z } = other;
        self.x -= x;
        self.y -= y;
        self.z -= z;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let Self { x, y, z } = other;
        Self {
            x: self.x - x,
            y: self.y - y,
            z: self.z - z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        let Self { x, y, z } = self;
        let Scalar(scalar) = other;
        Self {
            x: x * scalar,
            y: y * scalar,
            z: z * scalar,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_from_array() {
        let arr = [1, 2, 3];
        let vec: Vec3<u8> = arr.into();
        assert_eq!(vec, Vec3::new(1, 2, 3));
    }

    #[test]
    fn test_vec3_from_tuple() {
        let tup = (1, 2, 3);
        let vec: Vec3<u8> = tup.into();
        assert_eq!(vec, Vec3::new(1, 2, 3));
    }

    #[test]
    fn test_vec3_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_vec3_neg() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vec3_mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Scalar(2.0);
        assert_eq!(a * b, Vec3::new(2.0, 4.0, 6.0));
    }
}
