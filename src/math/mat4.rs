use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

const RANK: usize = 4;

use crate::{
    gl::{
        self,
        uniform::{UniformLocation, uniform_trait::Uniform},
    },
    math::vec4::Vec4,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Mat4<T: Copy> {
    cols: [Vec4<T>; RANK],
}

impl<T: Copy> Mat4<T> {
    pub const fn new(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Self {
        Self {
            cols: [c0, c1, c2, c3],
        }
    }

    pub const fn cols(&self) -> [T; RANK * RANK] {
        [
            self.cols[0].x,
            self.cols[0].y,
            self.cols[0].z,
            self.cols[0].w,
            self.cols[1].x,
            self.cols[1].y,
            self.cols[1].z,
            self.cols[1].w,
            self.cols[2].x,
            self.cols[2].y,
            self.cols[2].z,
            self.cols[2].w,
            self.cols[3].x,
            self.cols[3].y,
            self.cols[3].z,
            self.cols[3].w,
        ]
    }

    pub fn transpose(&self) -> Self {
        Self::new(
            Vec4::new(
                self.cols[0].x,
                self.cols[1].x,
                self.cols[2].x,
                self.cols[3].x,
            ),
            Vec4::new(
                self.cols[0].y,
                self.cols[1].y,
                self.cols[2].y,
                self.cols[3].y,
            ),
            Vec4::new(
                self.cols[0].z,
                self.cols[1].z,
                self.cols[2].z,
                self.cols[3].z,
            ),
            Vec4::new(
                self.cols[0].w,
                self.cols[1].w,
                self.cols[2].w,
                self.cols[3].w,
            ),
        )
    }
}

impl Mat4<f32> {
    pub const fn identity() -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn invert(&self) -> Option<Self> {
        let mut inv = [0.0; 16];
        let m: [f32; 16] = self.cols();

        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];

        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];

        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];

        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];

        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
            - m[9] * m[3] * m[14]
            - m[13] * m[2] * m[11]
            + m[13] * m[3] * m[10];

        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
            + m[8] * m[3] * m[14]
            + m[12] * m[2] * m[11]
            - m[12] * m[3] * m[10];

        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
            - m[8] * m[3] * m[13]
            - m[12] * m[1] * m[11]
            + m[12] * m[3] * m[9];

        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
            + m[8] * m[2] * m[13]
            + m[12] * m[1] * m[10]
            - m[12] * m[2] * m[9];

        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
            + m[5] * m[3] * m[14]
            + m[13] * m[2] * m[7]
            - m[13] * m[3] * m[6];

        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
            - m[4] * m[3] * m[14]
            - m[12] * m[2] * m[7]
            + m[12] * m[3] * m[6];

        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
            + m[4] * m[3] * m[13]
            + m[12] * m[1] * m[7]
            - m[12] * m[3] * m[5];

        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
            - m[4] * m[2] * m[13]
            - m[12] * m[1] * m[6]
            + m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
            - m[5] * m[3] * m[10]
            - m[9] * m[2] * m[7]
            + m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
            + m[4] * m[3] * m[10]
            + m[8] * m[2] * m[7]
            - m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
            - m[4] * m[3] * m[9]
            - m[8] * m[1] * m[7]
            + m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
            + m[4] * m[2] * m[9]
            + m[8] * m[1] * m[6]
            - m[8] * m[2] * m[5];

        let mut det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == 0.0 {
            return None;
        }

        det = 1.0 / det;

        return Some(Self::from(inv.map(|x| x * det)));
    }
}

impl From<[f32; RANK * RANK]> for Mat4<f32> {
    fn from(data: [f32; RANK * RANK]) -> Self {
        Self::new(
            Vec4::new(data[0], data[1], data[2], data[3]),
            Vec4::new(data[4], data[5], data[6], data[7]),
            Vec4::new(data[8], data[9], data[10], data[11]),
            Vec4::new(data[12], data[13], data[14], data[15]),
        )
    }
}

impl Uniform for &Mat4<f32> {
    type Options = bool; // transpose

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation) {
        let transpose = options.unwrap_or(false);
        let cols = self.cols();
        let value = cols.as_ptr() as *const f32;
        let UniformLocation(location) = *location;
        unsafe {
            gl::glUniformMatrix4fv(location, 1, u8::from(transpose), value);
        }
    }
}

impl<T: Copy + Add<Output = T>> Add for Mat4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cols: [
                self.cols[0] + other.cols[0],
                self.cols[1] + other.cols[1],
                self.cols[2] + other.cols[2],
                self.cols[3] + other.cols[3],
            ],
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Mat4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            cols: [
                self.cols[0] - other.cols[0],
                self.cols[1] - other.cols[1],
                self.cols[2] - other.cols[2],
                self.cols[3] - other.cols[3],
            ],
        }
    }
}

impl<T: Copy + Mul<Output = T> + Default + Add<Output = T> + Sum> Mul for Mat4<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let result = Mat4::default();

        for i in 0..RANK {
            for j in 0..RANK {
                result.cols[i].data()[j] = self.cols[i]
                    .data()
                    .iter()
                    .zip(other.cols[j].data())
                    .map(|(a, b)| a.clone() * b)
                    .sum();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat4_transpose() {
        let a = Mat4::identity();
        assert_eq!(a.transpose(), Mat4::identity());
    }

    #[test]
    fn test_mat4_inverse() {
        let a = Mat4::identity();
        assert_eq!(a.invert(), Some(a));
    }
}
