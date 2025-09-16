use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Default, PartialEq, Add, AddAssign, Sub, SubAssign, Neg)]
pub struct ColorRGB(Vec3<f32>);

impl ColorRGB {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3 { x: r, y: g, z: b })
    }

    pub const fn r(&self) -> f32 {
        self.0.x
    }
    pub const fn g(&self) -> f32 {
        self.0.y
    }
    pub const fn b(&self) -> f32 {
        self.0.z
    }
}

impl From<Vec3<f32>> for ColorRGB {
    fn from(value: Vec3<f32>) -> Self {
        Self(value)
    }
}

impl From<ColorRGB> for Vec3<f32> {
    fn from(ColorRGB(v): ColorRGB) -> Self {
        v
    }
}

impl From<(f32, f32, f32)> for ColorRGB {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<ColorRGB> for (f32, f32, f32) {
    fn from(v: ColorRGB) -> Self {
        (v.r(), v.g(), v.g())
    }
}

impl From<[f32; 3]> for ColorRGB {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::new(r, g, b)
    }
}

impl From<ColorRGB> for [f32; 3] {
    fn from(v: ColorRGB) -> Self {
        [v.r(), v.g(), v.g()]
    }
}
