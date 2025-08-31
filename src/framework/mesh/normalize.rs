use assimp::Scene;
use assimp_sys::AiVector3D;

use crate::math::{Scalar, vec3::Vec3};

#[derive(Debug)]
pub enum NormalizeOptions {
    False,
    True(u32),
}

impl NormalizeOptions {
    pub fn normalize_scene(&self, scene: &mut Scene<'_>) {
        match self {
            Self::False => {}
            Self::True(scale) => normalize_scene(scene, *scale),
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
    let s = scale as f32;

    let min = Vec3::scalar(-s);
    let max = Vec3::scalar(s);

    let span = max - min;

    let scale_f = Vec3::cmin(span / span);

    let center = bbox.center();

    for mesh in scene.mesh_iter() {
        for (index, vector) in mesh.vertex_iter().enumerate() {
            let vec = (Vec3::from(vector) - center) * Scalar(scale_f);
            let ai = AiVector3D::from(vec);
            unsafe {
                mesh.vertices.add(index).write(ai);
            }
        }
    }
}
