use crate::math::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

#[derive(Debug)]
pub struct Camera {
    position: Vec3<f32>,
    dir: Vec3<f32>,
    up: Vec3<f32>,
    fov_deg: f32,
    near_plane: f32,
    far_plane: f32,
}

impl Camera {
    pub fn new(
        position: Vec3<f32>,
        dir: Vec3<f32>,
        up: Vec3<f32>,
        fov_deg: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        Self {
            position,
            dir: dir.normalize(),
            up: up.normalize(),
            fov_deg,
            near_plane,
            far_plane,
        }
    }

    pub fn view(&self) -> Mat4<f32> {
        let look = self.position + self.dir;
        build_view_matrix(&self.position, &look, &self.up)
    }

    pub fn projection(&self, aspect_ratio: f32) -> Mat4<f32> {
        perspective(self.fov_deg, aspect_ratio, self.near_plane, self.far_plane)
    }

    pub const fn position(&self) -> &Vec3<f32> {
        &self.position
    }

    pub fn translate(&mut self, translation: &Vec3<f32>) {
        self.position += *translation;
    }

    pub fn look_at(&mut self, target: &Vec3<f32>) {
        self.dir = (*target - self.position).normalize();
    }

    pub fn transform_position(&mut self, mut fun: impl FnMut(&mut Vec3<f32>)) {
        fun(&mut self.position);
    }
}

fn build_view_matrix(eye: &Vec3<f32>, center: &Vec3<f32>, up: &Vec3<f32>) -> Mat4<f32> {
    let f = (*center - *eye).normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    Mat4::new(
        Vec4::new(s.x(), u.x(), -f.x(), 0.0),
        Vec4::new(s.y(), u.y(), -f.y(), 0.0),
        Vec4::new(s.z(), u.z(), -f.z(), 0.0),
        Vec4::new(-s.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0),
    )
}

fn perspective(fov_deg: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Mat4<f32> {
    let fov_rad = fov_deg.to_radians();
    let f = 1.0 / (fov_rad / 2.0).tan();
    let range_inv = 1.0 / (far_plane - near_plane);

    Mat4::new(
        Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -(far_plane + near_plane) * range_inv, -1.0),
        Vec4::new(0.0, 0.0, -2.0 * far_plane * near_plane * range_inv, 0.0),
    )
}
