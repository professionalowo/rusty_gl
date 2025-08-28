pub mod f32;

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn data(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub const fn size() -> usize {
        3
    }

    pub fn map<U, F>(&self, f: F) -> Vec3<U>
    where
        F: Fn(T) -> U,
        U: Copy,
    {
        let x = f(self.x);
        let y = f(self.y);
        let z = f(self.z);
        Vec3::<U>::new(x, y, z)
    }

    pub fn to<U>(&self) -> Vec3<U>
    where
        T: Into<U>,
        U: Copy,
    {
        self.map(|v| T::into(v))
    }
}

impl<T> Neg for Vec3<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        let Self { x, y, z } = self;

        Self {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Self { x, y, z } = self;
        let Self {
            x: ox,
            y: oy,
            z: oz,
        } = other;
        Self {
            x: x + ox,
            y: y + oy,
            z: z + oz,
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

impl<T> Sub for Vec3<T>
where
    T: Copy + Sub<Output = T>,
{
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

impl<T> Mul<Scalar<T>> for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        let Self { x, y, z } = self;
        let Scalar(s) = other;
        Self {
            x: x * s,
            y: y * s,
            z: z * s,
        }
    }
}

impl<T> MulAssign<Scalar<T>> for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: Scalar<T>) {
        let Self { x, y, z } = *self;
        let Scalar(s) = other;
        self.x = x * s;
        self.y = y * s;
        self.z = z * s;
    }
}

impl<T> Mul<Vec3<T>> for Scalar<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        let Self(s) = self;
        let Self::Output { x, y, z } = other;
        Self::Output::new(s * x, s * y, s * z)
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Copy,
{
    fn from(arr: [T; 3]) -> Self {
        let [x, y, z] = arr;
        Self::new(x, y, z)
    }
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Copy,
{
    fn from(tup: (T, T, T)) -> Self {
        let (x, y, z) = tup;
        Self::new(x, y, z)
    }
}

impl<T> Not for Vec3<T>
where
    T: Copy + Not<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self {
            x: !x,
            y: !y,
            z: !z,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_from_array() {
        let vec: Vec3<u8> = Vec3::from([1, 2, 3]);
        assert_eq!(vec, Vec3::new(1, 2, 3));
    }

    #[test]
    fn test_vec3_from_tuple() {
        let vec: Vec3<u8> = Vec3::from((1, 2, 3));
        assert_eq!(vec, Vec3::new(1, 2, 3));
    }

    #[test]
    fn test_vec3_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a -= b;
        assert_eq!(a, Vec3::new(-3.0, -3.0, -3.0));
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

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Scalar(2.0);
        assert_eq!(b * a, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Scalar(2.0);
        a *= b;
        assert_eq!(a, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_not() {
        let a = Vec3::new(true, false, true);
        assert_eq!(!a, Vec3::new(false, true, false));
    }

    #[test]
    fn test_vec3_default() {
        let a: Vec3<u8> = Vec3::default();
        assert_eq!(a, Vec3::new(u8::default(), u8::default(), u8::default()));
    }

    #[test]
    fn test_vec3_eq() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(1, 2, 3);
        let c = Vec3::new(4, 5, 6);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
