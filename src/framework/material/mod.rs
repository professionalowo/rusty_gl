use std::{collections::HashMap, fmt};

use assimp_sys::{AiColor3D, AiColor4D};

use crate::math::vec4::Vec4;

use super::texture::Texture2D;

#[derive(Debug)]
pub struct MaterialConversionError;

impl fmt::Display for MaterialConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not convert material")
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
        let mut diff = AiColor4D {
            r: 0.0,
            b: 0.0,
            g: 0.0,
            a: 0.0,
        };
        let mut spec = diff.clone();
        let mut amb = diff.clone();

        let raw = value.to_raw();
        Err(MaterialConversionError)
    }
}
