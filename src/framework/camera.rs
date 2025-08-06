use crate::math::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

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

    pub fn view(&self) -> Mat4<f32> {
        let forward = (self.look_at - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward).normalize();

        Mat4::new(
            Vec4::new(right.x(), up.x(), -forward.x(), 0.0),
            Vec4::new(right.y(), up.y(), -forward.y(), 0.0),
            Vec4::new(right.z(), up.z(), -forward.z(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub const fn position(&self) -> Vec3<f32> {
        self.position
    }

    pub fn translate(&mut self, translation: Vec3<f32>) {
        self.position += translation;
    }
}
