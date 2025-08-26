use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2<T: Copy> {
    x: T,
    y: T,
}

impl<T: Copy> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub const fn data(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub const fn size() -> usize {
        2
    }
}

impl<T: Copy> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        Self {
            x: self.x * other.0,
            y: self.y * other.0,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec2<T>> for Scalar<T> {
    type Output = Vec2<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        Self::Output {
            x: self.0 * other.x,
            y: self.0 * other.y,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<Scalar<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Scalar<T>) -> Self::Output {
        let Scalar(div) = rhs;

        Self::Output {
            x: self.x / div,
            y: self.y / div,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let result = a + b;
        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let result = a - b;
        assert_eq!(result, Vec2::new(-2.0, -2.0));
    }

    #[test]
    fn test_vec2_neg() {
        let a = Vec2::new(1.0, 2.0);
        let result = -a;
        assert_eq!(result, Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn test_vec2_mul() {
        let a = Vec2::new(1.0, 2.0);
        let b = Scalar(3.0);
        let result = a * b;
        assert_eq!(result, Vec2::new(3.0, 6.0));
    }

    #[test]
    fn test_vec2_div() {
        let a = Vec2::new(1.0, 2.0);
        let b = Scalar(2.0);
        let result = a / b;
        assert_eq!(result, Vec2::new(0.5, 1.0));
    }

    #[test]
    fn test_vec2_eq() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(1.0, 2.0);
        assert!(a == b);
    }
}
