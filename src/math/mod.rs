use std::ops::Mul;

pub mod mat3;
pub mod vec2;
pub mod vec3;

pub struct Scalar<T>(pub T)
where
    T: Copy + Mul<Output = T>;
