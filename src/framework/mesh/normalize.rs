use assimp::Scene;
use assimp_sys::AiVector3D;

use crate::math::{Scalar, vec3::Vec3};

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
    const fn infinity() -> Self {
        Self {
            min: Vec3::scalar(f32::MAX),
            max: Vec3::scalar(f32::MIN),
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
    let mut bbox = BoundingBox::infinity();

    for mesh in scene.mesh_iter() {
        for v in mesh.vertex_iter().map(Vec3::from) {
            bbox.bb(v);
        }
    }
    let s = (2 * scale) as f32;

    let scale_v = Vec3::scalar(s) / bbox.distance();

    let scale_f = Scalar(Vec3::cmin(scale_v));

    let center = bbox.center();

    for mesh in scene.mesh_iter() {
        for (index, vector) in mesh.vertex_iter().enumerate() {
            let vec = scale_f * (Vec3::from(vector) - center);
            let ai = AiVector3D::from(vec);
            unsafe {
                mesh.vertices.add(index).write(ai);
            }
        }
    }
}
