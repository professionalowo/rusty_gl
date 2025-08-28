use std::{collections::HashMap, fmt};

use crate::{
    framework::assimp::{AMaterial, AiError},
    math::{vec3::Vec3, vec4::Vec4},
};

use super::texture::Texture2D;

#[derive(Debug)]
pub enum MaterialConversionError {
    AiError(AiError),
}

impl From<AiError> for MaterialConversionError {
    fn from(value: AiError) -> Self {
        Self::AiError(value)
    }
}

impl fmt::Display for MaterialConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AiError(a) => fmt::Display::fmt(a, f),
        }
    }
}

#[derive(Debug, Default)]
pub struct Material {
    textures: HashMap<String, Texture2D>,
    k_amb: Vec4<f32>,
    k_diff: Vec4<f32>,
    k_spec: Vec4<f32>,
}

impl Material {
    pub fn add_texture(&mut self, key: String, value: Texture2D) -> Option<Texture2D> {
        self.textures.insert(key, value)
    }

    pub const fn k_amb(&self) -> &Vec4<f32> {
        &self.k_amb
    }

    pub const fn k_diff(&self) -> &Vec4<f32> {
        &self.k_diff
    }

    pub const fn k_spec(&self) -> &Vec4<f32> {
        &self.k_spec
    }

    pub fn bind(&self) {
        todo!()
    }
    pub fn unbind(&self) {
        todo!()
    }
}

impl TryFrom<assimp::Material<'_>> for Material {
    type Error = MaterialConversionError;
    fn try_from(value: assimp::Material) -> Result<Self, Self::Error> {
        let mat = AMaterial(value);

        let diff =
            Vec3::from(mat.get_material_color("$clr.diffuse".as_ptr().cast::<i8>(), 0, 0)?);
        let k_diff = diff.expand(1.0);

        let spec =
            Vec3::from(mat.get_material_color("$clr.specular".as_ptr().cast::<i8>(), 0, 0)?);
        let k_spec = spec.expand(1.0);

        let amb = Vec3::from(mat.get_material_color("$clr.ambient".as_ptr().cast::<i8>(), 0, 0)?);
        let k_amb = amb.expand(1.0);
        Ok(Self {
            textures: HashMap::new(),
            k_amb,
            k_diff,
            k_spec,
        })
    }
}
