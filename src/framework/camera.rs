use crate::math::{mat3::Mat3, vec3::Vec3};

#[derive(Debug)]
pub struct Camera {
    position: Vec3<f32>,
    look_at: Vec3<f32>,
    up: Vec3<f32>,
}

impl Camera {
    pub const fn new(position: Vec3<f32>, look_at: Vec3<f32>, up: Vec3<f32>) -> Self {
        Self {
            position,
            look_at,
            up,
        }
    }

    pub fn view(&self) -> Mat3<f32> {
        let forward = (self.look_at - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward).normalize();

        Mat3::new(
            Vec3::new(right.x(), up.x(), -forward.x()),
            Vec3::new(right.y(), up.y(), -forward.y()),
            Vec3::new(right.z(), up.z(), -forward.z()),
        )
    }

    pub const fn position(&self) -> Vec3<f32> {
        self.position
    }

    pub fn translate(&mut self, translation: Vec3<f32>) {
        self.position += translation;
    }
}
