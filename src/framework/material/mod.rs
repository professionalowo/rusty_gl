use std::{
    collections::HashMap,
    ffi::CString,
    fmt,
    path::{Path, PathBuf},
};

use assimp_sys::AiTextureType;

use crate::{
    framework::assimp::{AMaterial, AiError},
    gl::{program::Program, uniform::UniformLocationError},
    math::{vec3::Vec3, vec4::Vec4},
};

use super::texture::Texture2D;

#[derive(Debug)]
pub enum MaterialConversionError {
    AiError(AiError),
    NulError(std::ffi::NulError),
}

impl From<AiError> for MaterialConversionError {
    fn from(value: AiError) -> Self {
        Self::AiError(value)
    }
}

impl From<std::ffi::NulError> for MaterialConversionError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}

impl fmt::Display for MaterialConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AiError(a) => fmt::Display::fmt(a, f),
            Self::NulError(n) => fmt::Display::fmt(n, f),
        }
    }
}

#[derive(Debug, Default)]
pub struct Material {
    pub name: String,
    pub textures: HashMap<String, Texture2D>,
    pub k_amb: Vec4<f32>,
    pub k_diff: Vec4<f32>,
    pub k_spec: Vec4<f32>,
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

    pub fn bind(&self, program: &Program) -> Result<(), UniformLocationError> {
        //program.uniform("k_amb", &self.k_amb)?;
        program.uniform("k_diff", &self.k_diff)?;
        program.uniform("k_spec", &self.k_spec)?;
        let mut unit = 0;
        for (name, texture) in &self.textures {
            texture.bind(unit);
            unit += 1;
            program.uniform_opt(name, texture, unit)?;
        }
        Ok(())
    }
    pub fn unbind(&self) {
        for (_, texture) in &self.textures {
            texture.unbind();
        }
    }

    pub fn from_ai_mesh(
        base_path: &Path,
        name: String,
        mat: &AMaterial,
    ) -> Result<Self, MaterialConversionError> {
        let diff = Vec3::from(mat.get_material_color(CString::new("$clr.diffuse")?, 0, 0)?);
        let k_diff = diff.expand(1.0);

        let spec = Vec3::from(mat.get_material_color(CString::new("$clr.specular")?, 0, 0)?);
        let k_spec = spec.expand(1.0);

        let amb = Vec3::from(mat.get_material_color(CString::new("$clr.ambient")?, 0, 0)?);
        let k_amb = amb.expand(1.0);

        let mut textures = HashMap::new();

        if mat.get_texture_count(AiTextureType::Diffuse) > 0 {
            let tex = mat.get_texture(AiTextureType::Diffuse, 0)?;
            dbg!(&tex);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.diffuse")?, 0, 0) {
            dbg!(col);
        }

        if mat.get_texture_count(AiTextureType::Specular) > 0 {
            let tex = mat.get_texture(AiTextureType::Specular, 0)?;
            dbg!(&tex);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.specular")?, 0, 0) {
            dbg!(col);
        }

        if mat.get_texture_count(AiTextureType::Height) > 0 {
            let tex = mat.get_texture(AiTextureType::Height, 0)?;
            dbg!(&tex);
        }

        if mat.get_texture_count(AiTextureType::Opacity) > 0 {
            let tex = mat.get_texture(AiTextureType::Opacity, 0)?;
            dbg!(&tex);
        }

        Ok(Self {
            name,
            textures,
            k_amb,
            k_diff,
            k_spec,
        })
    }
}
