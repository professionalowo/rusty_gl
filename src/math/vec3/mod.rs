pub mod f32;

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use crate::math::{Scalar, vec2::Vec2, vec4::Vec4};

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Vec3<T>
where
    T: Copy,
{
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
        let Self { x, y, z } = *self;
        Vec3::<U>::new(f(x), f(y), f(z))
    }

    pub fn to<U>(&self) -> Vec3<U>
    where
        T: Into<U>,
        U: Copy,
    {
        self.map(|v| T::into(v))
    }

    pub const fn truncate(&self) -> Vec2<T> {
        let Self { x, y, .. } = *self;
        Vec2::new(x, y)
    }

    pub const fn expand(&self, w: T) -> Vec4<T> {
        let Self { x, y, z } = *self;
        Vec4::new(x, y, z, w)
    }

    pub const fn from_slice(slice: &[T]) -> Option<Self> {
        if let [x, y, z, ..] = *slice {
            Some(Self::new(x, y, z))
        } else {
            None
        }
    }

    pub unsafe fn from_raw(ptr: *const T) -> Self {
        unsafe { [ptr.read(), ptr.add(1).read(), ptr.add(2).read()] }.into()
    }

    pub fn cmin(&self) -> T
    where
        T: Ord,
    {
        self.x.min(self.y).min(self.z)
    }

    pub fn cmax(&self) -> T
    where
        T: Ord,
    {
        self.x.max(self.y).max(self.z)
    }
}

impl<T> Neg for Vec3<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        let Self { x, y, z } = self;
        Self::new(-x, -y, -z)
    }
}

impl<T> Add for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, Self { x: a, y: b, z: c }: Self) -> Self {
        let Self { x, y, z } = self;
        Self {
            x: x + a,
            y: y + b,
            z: z + c,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, Self { x, y, z }: Self) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, Self { x, y, z }: Self) {
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

    fn sub(self, Self { x, y, z }: Self) -> Self {
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

    fn mul(self, Scalar(s): Scalar<T>) -> Self {
        let Self { x, y, z } = self;
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
    fn mul_assign(&mut self, Scalar(s): Scalar<T>) {
        let Self { x, y, z } = *self;
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

    fn mul(self, Self::Output { x, y, z }: Self::Output) -> Self::Output {
        let Self(s) = self;
        Self::Output::new(s * x, s * y, s * z)
    }
}

impl<T> Div<Vec3<T>> for Vec3<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, Self::Output { x, y, z }: Self::Output) -> Self::Output {
        Self::Output::new(self.x / x, self.y / y, self.z / z)
    }
}

impl<T> Not for Vec3<T>
where
    T: Copy + Not<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self::new(!x, !y, !z)
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Copy,
{
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Copy,
{
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_from_array() {
        assert_eq!(Vec3::from([1, 2, 3]), Vec3::new(1, 2, 3));
    }

    #[test]
    fn test_vec3_from_tuple() {
        assert_eq!(Vec3::from((1, 2, 3)), Vec3::new(1, 2, 3));
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

    #[test]
    fn test_vec3_from_raw() {
        let data = [1, 2, 3, 4, 5];
        let ptr = data.as_ptr();
        let vec = unsafe { Vec3::from_raw(ptr.add(1)) };

        assert_eq!(vec, Vec3::new(2, 3, 4));
    }

    #[test]
    fn test_vec3_from_slice() {
        let data = [1, 2, 3, 4, 5];
        let vec = Vec3::from_slice(&data[1..]);

        assert_eq!(vec, Some(Vec3::new(2, 3, 4)));

        assert_eq!(Vec3::from_slice(&[0]), None)
    }

    #[test]
    fn test_vec3_map() {
        assert_eq!(
            Vec3::new(1, 0, 1).map(|x| x == 0),
            Vec3::new(false, true, false)
        )
    }
}
