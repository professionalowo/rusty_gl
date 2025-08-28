use crate::{
    framework::{camera::Camera, material::Material, mesh::Mesh},
    gl::program::Program,
    math::mat4::Mat4,
};

pub struct Drawelement {
    program: Option<Program>,
    material: Option<Material>,
    meshes: Box<[Mesh]>,
}

impl Drawelement {
    pub fn bind(&self, camera: &Camera) {
        if let Some(p) = self.program.as_ref() {
            p.bind();
            if let Some(m) = self.material.as_ref() {
                m.bind();
            }
        }
    }

    pub fn unbind(&self) {
        if let Some(m) = self.material.as_ref() {
            m.unbind();
        }
        if let Some(p) = self.program.as_ref() {
            p.unbind();
        }
    }

    pub fn draw(&self, model: &Mat4<f32>) {
        if let Some(p) = self.program.as_ref() {
            let model_normal = model
                .invert()
                .map(|mat| mat.transpose())
                .unwrap_or(Mat4::identity());
        }
        for mesh in self.meshes.iter() {
            mesh.bind();
            mesh.draw();
            mesh.unbind();
        }
    }
}
