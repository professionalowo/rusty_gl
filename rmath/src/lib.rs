use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Not, Shl, Shr, Sub, SubAssign,
};

pub mod mat3;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;

#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

impl<T> Deref for Scalar<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Scalar<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Copy for Scalar<T> where T: Copy {}

impl<T> Add<T> for Scalar<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self(self.0 + other)
    }
}

impl<T> Add<Scalar<T>> for Scalar<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, Self(o): Self) -> Self {
        Self(self.0 + o)
    }
}

impl<T> AddAssign<T> for Scalar<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, other: T) {
        self.0 += other;
    }
}

impl<T> AddAssign<Scalar<T>> for Scalar<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, Self(o): Self) {
        self.0 += o;
    }
}

impl<T> Sub<T> for Scalar<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self(self.0 - other)
    }
}

impl<T> Sub<Scalar<T>> for Scalar<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, Self(o): Self) -> Self {
        Self(self.0 - o)
    }
}

impl<T> SubAssign<T> for Scalar<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, other: T) {
        self.0 -= other;
    }
}

impl<T> SubAssign<Scalar<T>> for Scalar<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, Self(o): Self) {
        self.0 -= o;
    }
}

impl<T> Mul<T> for Scalar<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self(self.0 * other)
    }
}

impl<T> Mul<Scalar<T>> for Scalar<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, Self(o): Self) -> Self {
        Self(self.0 * o)
    }
}

impl<T> MulAssign<T> for Scalar<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, other: T) {
        self.0 *= other;
    }
}

impl<T> MulAssign<Scalar<T>> for Scalar<T>
where
    T: MulAssign<T>,
{
    fn mul_assign(&mut self, Self(o): Self) {
        self.0 *= o;
    }
}

impl<T> Div<T> for Scalar<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self(self.0 / other)
    }
}

impl<T> Div<Scalar<T>> for Scalar<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, Self(o): Self) -> Self {
        Self(self.0 / o)
    }
}

impl<T> DivAssign<T> for Scalar<T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, other: T) {
        self.0 /= other;
    }
}

impl<T> DivAssign<Scalar<T>> for Scalar<T>
where
    T: DivAssign<T>,
{
    fn div_assign(&mut self, Self(o): Self) {
        self.0 /= o;
    }
}

impl<T> Neg for Scalar<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl<T> Not for Scalar<T>
where
    T: Not<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl<T> Shl<T> for Scalar<T>
where
    T: Shl<Output = T>,
{
    type Output = Self;

    fn shl(self, other: T) -> Self {
        Self(self.0 << other)
    }
}

impl<T> Shl<Scalar<T>> for Scalar<T>
where
    T: Shl<Output = T>,
{
    type Output = Self;

    fn shl(self, Self(o): Self) -> Self {
        Self(self.0 << o)
    }
}

impl<T> Shr<T> for Scalar<T>
where
    T: Shr<Output = T>,
{
    type Output = Self;

    fn shr(self, other: T) -> Self {
        Self(self.0 >> other)
    }
}

impl<T> Shr<Scalar<T>> for Scalar<T>
where
    T: Shr<Output = T>,
{
    type Output = Self;

    fn shr(self, Self(o): Self) -> Self {
        Self(self.0 >> o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_from() {
        let a = Scalar::from(5);
        assert_eq!(a, Scalar(5));
    }

    #[test]
    fn test_scalar_into() {
        let a: Scalar<i32> = 5.into();
        assert_eq!(a, Scalar(5));
    }

    #[test]
    fn test_scalar_ord() {
        let a = Scalar(5);
        let b = Scalar(10);
        assert!(a < b);
    }

    #[test]
    fn test_scalar_eq() {
        let a = Scalar(5);
        let b = Scalar(5);
        assert!(a == b);
    }

    #[test]
    fn test_scalar_t_add() {
        let a = Scalar(5);
        let b = 10;
        assert_eq!(a + b, Scalar(15));
    }

    #[test]
    fn test_scalar_scalar_add() {
        let a = Scalar(5);
        let b = Scalar(10);
        assert_eq!(a + b, Scalar(15));
    }

    #[test]
    fn test_scalar_t_sub() {
        let a = Scalar(5);
        let b = Scalar(10);
        assert_eq!(a - b, Scalar(-5));
    }

    #[test]
    fn test_scalar_t_mul() {
        let a = Scalar(5);
        let b = Scalar(10);
        assert_eq!(a * b, Scalar(50));
    }

    #[test]
    fn test_scalar_t_div() {
        let a = Scalar(10);
        let b = Scalar(5);
        assert_eq!(a / b, Scalar(2));
    }

    #[test]
    fn test_scalar_neg() {
        assert_eq!(-Scalar(5), Scalar(-5));
    }

    #[test]
    fn test_scalar_not() {
        assert_eq!(!Scalar(true), Scalar(false));
    }

    #[test]
    fn test_scalar_shl() {
        let a = Scalar(5);
        let b = Scalar(2);
        assert_eq!(a << b, Scalar(20));
        assert_eq!(a << 2, Scalar(20));
    }

    #[test]
    fn test_scalar_shr() {
        let a = Scalar(20);
        let b = Scalar(2);
        assert_eq!(a >> b, Scalar(5));
        assert_eq!(a >> 2, Scalar(5));
    }
}
