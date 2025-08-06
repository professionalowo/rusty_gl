use std::ops::Neg;

use crate::math::Scalar;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T>> std::ops::Mul<Scalar<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: Scalar<T>) -> Self {
        Self {
            x: self.x * other.0,
            y: self.y * other.0,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T>> std::ops::Mul<Vec2<T>> for Scalar<T> {
    type Output = Vec2<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        Self::Output {
            x: self.0 * other.x,
            y: self.0 * other.y,
        }
    }
}
