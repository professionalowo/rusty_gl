use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub mod mat3;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

impl<T> Default for Scalar<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
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

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
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
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
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

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
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
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
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

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
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
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
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

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
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
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_add() {
        let a = Scalar(5);
        let b = 10;
        assert_eq!(a + b, Scalar(15));
    }

    #[test]
    fn test_scalar_sub() {
        let a = Scalar(5);
        let b = 10;
        assert_eq!(a - b, Scalar(-5));
    }

    #[test]
    fn test_scalar_mul() {
        let a = Scalar(5);
        let b = 10;
        assert_eq!(a * b, Scalar(50));
    }

    #[test]
    fn test_scalar_div() {
        let a = Scalar(10);
        let b = 5;
        assert_eq!(a / b, Scalar(2));
    }

    #[test]
    fn test_scalar_neg() {
        assert_eq!(-Scalar(5), Scalar(-5));
    }
}
