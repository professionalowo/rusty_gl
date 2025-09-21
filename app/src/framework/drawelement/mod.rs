use std::rc::Rc;

use crate::{
    framework::{camera::Camera, material::Material, mesh::Mesh},
    math::mat4::Mat4,
};

use gl_sys::gl::{program::Program, uniform::UniformLocationError};

#[derive(Debug)]
pub struct Drawelement {
    pub material: Rc<Material>,
    pub mesh: Mesh,
}

impl Drawelement {
    pub fn bind(
        &self,
        program: &Program,
        camera: &Camera,
        aspect: f32,
    ) -> Result<(), UniformLocationError> {
        program.bind();
        self.material.bind(program)?;
        program.uniform("view", &camera.view())?;
        program.uniform("projection", &camera.projection(aspect))?;
        Ok(())
    }

    pub fn draw(&self, program: &Program, model: &Mat4<f32>) -> Result<(), UniformLocationError> {
        program.uniform("model", model)?;
        program.uniform(
            "model_normal",
            &model.invert().unwrap_or(Mat4::identity()).transpose(),
        )?;
        self.mesh.bind();
        self.mesh.draw();
        self.mesh.unbind();
        Ok(())
    }

    pub fn unbind(&self, program: &Program) {
        self.material.unbind();
        program.unbind();
    }
}
