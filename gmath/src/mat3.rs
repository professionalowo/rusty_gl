use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

const RANK: usize = 3;
use crate::vec3::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat3<T: Copy> {
    cols: [Vec3<T>; RANK],
}

impl<T: Copy> Mat3<T> {
    pub const fn new(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Self {
        Self { cols: [c0, c1, c2] }
    }

    pub const fn cols(&self) -> [T; RANK * RANK] {
        [
            self.cols[0].x,
            self.cols[0].y,
            self.cols[0].z,
            self.cols[1].x,
            self.cols[1].y,
            self.cols[1].z,
            self.cols[2].x,
            self.cols[2].y,
            self.cols[2].z,
        ]
    }

    pub fn transpose(&self) -> Self {
        Self::new(
            Vec3::new(self.cols[0].x, self.cols[1].x, self.cols[2].x),
            Vec3::new(self.cols[0].y, self.cols[1].y, self.cols[2].y),
            Vec3::new(self.cols[0].z, self.cols[1].z, self.cols[2].z),
        )
    }
}

impl<T: Copy + Add<Output = T>> Add for Mat3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cols: [
                self.cols[0] + other.cols[0],
                self.cols[1] + other.cols[1],
                self.cols[2] + other.cols[2],
            ],
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Mat3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            cols: [
                self.cols[0] - other.cols[0],
                self.cols[1] - other.cols[1],
                self.cols[2] - other.cols[2],
            ],
        }
    }
}

impl<T: Copy + Mul<Output = T> + Default + Add<Output = T> + Sum> Mul for Mat3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let result = Mat3::new(
            Vec3::new(T::default(), T::default(), T::default()),
            Vec3::new(T::default(), T::default(), T::default()),
            Vec3::new(T::default(), T::default(), T::default()),
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
