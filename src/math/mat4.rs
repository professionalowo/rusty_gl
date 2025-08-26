use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

const RANK: usize = 4;
use crate::{
    gl::{
        glUniformMatrix4fv,
        uniform::{UniformLocation, uniform_trait::Uniform},
    },
    math::vec4::Vec4,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
}

impl Uniform for &Mat4<f32> {
    type Options = bool; // transpose

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation) {
        let transpose = options.unwrap_or(false);
        let cols = self.cols();
        let value = cols.as_ptr() as *const f32;
        let UniformLocation(location) = *location;
        unsafe {
            glUniformMatrix4fv(location, 1, u8::from(transpose), value);
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
        let result = Mat4::new(
            Vec4::new(T::default(), T::default(), T::default(), T::default()),
            Vec4::new(T::default(), T::default(), T::default(), T::default()),
            Vec4::new(T::default(), T::default(), T::default(), T::default()),
            Vec4::new(T::default(), T::default(), T::default(), T::default()),
        );

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
