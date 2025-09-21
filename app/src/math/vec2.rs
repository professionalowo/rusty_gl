use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2<T: Copy> {
    pub x: T,
    pub y: T,
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

    fn add(self, Self { x, y }: Self) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, Self { x, y }: Self) -> Self {
        Self {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Scalar<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, Scalar(s): Scalar<T>) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec2<T>> for Scalar<T> {
    type Output = Vec2<T>;

    fn mul(self, Self::Output { x, y }: Self::Output) -> Self::Output {
        Self::Output {
            x: self.0 * x,
            y: self.0 * y,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<Scalar<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, Scalar(div): Scalar<T>) -> Self::Output {
        Self::Output {
            x: self.x / div,
            y: self.y / div,
        }
    }
}

impl<T: Copy> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
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
