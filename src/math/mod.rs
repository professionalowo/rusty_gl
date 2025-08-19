use std::ops::Mul;

pub mod mat3;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T)
where
    T: Copy + Mul<Output = T>;
