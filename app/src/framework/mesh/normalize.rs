use assimp::Scene;
use assimp_sys::AiVector3D;

use rmath::{Scalar, vec3::Vec3};

#[derive(Debug)]
pub enum NormalizeOptions {
    Scale(u32),
    One,
    None,
}

impl NormalizeOptions {
    pub fn normalize_scene(&self, scene: &mut Scene<'_>) {
        match self {
            Self::One => normalize_scene(scene, 1),
            Self::Scale(scale) => normalize_scene(scene, *scale),
            Self::None => {}
        }
    }
}

#[derive(Debug)]
struct BoundingBox {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
}

impl BoundingBox {
    const fn new(min: f32, max: f32) -> Self {
        Self {
            min: Vec3::scalar(min),
            max: Vec3::scalar(max),
        }
    }

    const fn bb(&mut self, v: Vec3<f32>) {
        (self.min, self.max) = (Vec3::min(self.min, v), Vec3::max(self.max, v))
    }

    fn distance(&self) -> Vec3<f32> {
        self.max - self.min
    }

    fn center(&self) -> Vec3<f32> {
        Scalar(0.5) * (self.min + self.max)
    }
}

fn normalize_scene(scene: &mut Scene<'_>, scale: u32) {
    let mut bbox = BoundingBox::new(f32::MAX, f32::MIN);

    for mesh in scene.mesh_iter() {
        for v in mesh.vertex_iter().map(Vec3::from) {
            bbox.bb(v);
        }
    }
    let s = (2 * scale) as f32;

    let scale_v = Vec3::scalar(s) / bbox.distance();

    let scale_f = Scalar(Vec3::cminf32(scale_v));

    let center = bbox.center();

    for mesh in scene.mesh_iter() {
        for (index, vector) in mesh.vertex_iter().map(Vec3::from).enumerate() {
            let vec = scale_f * (vector - center);
            let ai = AiVector3D::from(vec);
            unsafe {
                mesh.vertices.add(index).write(ai);
            }
        }
    }
}
