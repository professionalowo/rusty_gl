use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorRGB(pub Vec3<f32>);

impl ColorRGB {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl From<Vec3<f32>> for ColorRGB {
    fn from(value: Vec3<f32>) -> Self {
        Self(value)
    }
}

impl From<ColorRGB> for Vec3<f32> {
    fn from(ColorRGB(vec): ColorRGB) -> Self {
        vec
    }
}
