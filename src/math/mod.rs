use std::ops::Mul;

pub mod mat3;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;

#[derive(Clone, Copy, Debug)]
pub struct Scalar<T>(pub T)
where
    T: Copy + Mul<Output = T>;

impl<T> PartialEq for Scalar<T>
where
    T: Copy + PartialEq + Mul<Output = T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Scalar<T> where T: Copy + Eq + Mul<Output = T> {}

impl<T> PartialOrd for Scalar<T>
where
    T: Copy + PartialOrd + Mul<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Scalar<T>
where
    T: Copy + Eq + Ord + Mul<Output = T>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
