use std::{
    collections::HashMap,
    ffi::CString,
    fmt,
    path::{Path, PathBuf},
};

use assimp_sys::AiTextureType;

use crate::{
    framework::{
        assimp::{AMaterial, AiError},
        texture::TextureError,
    },
    gl::{self, program::Program, uniform::UniformLocationError},
    math::{vec3::Vec3, vec4::Vec4},
};

use super::texture::Texture2D;

#[derive(Debug)]
pub enum MaterialConversionError {
    AiError(AiError),
    NulError(std::ffi::NulError),
    TextureError(TextureError),
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

impl From<TextureError> for MaterialConversionError {
    fn from(value: TextureError) -> Self {
        Self::TextureError(value)
    }
}

impl fmt::Display for MaterialConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AiError(a) => fmt::Display::fmt(a, f),
            Self::NulError(n) => fmt::Display::fmt(n, f),
            Self::TextureError(e) => fmt::Display::fmt(e, f),
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
        // program.uniform("k_diff", &self.k_diff)?;
        // program.uniform("k_spec", &self.k_spec)?;
        let mut unit = 0;
        for (name, texture) in &self.textures {
            texture.bind(unit)?;
            program.uniform_opt(name, texture, unit)?;
            unit += 1;
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
            let texture = get_texture(base_path, mat, AiTextureType::Diffuse)?;
            textures.insert("diffuse".to_string(), texture);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.diffuse")?, 0, 0) {
            let texture = Texture2D::from_data(
                1,
                1,
                gl::GL_RGB32F as i32,
                gl::GL_RGB,
                gl::GL_FLOAT,
                &[col.r],
                false,
            )?;
            textures.insert("specular".to_string(), texture);
        }

        if mat.get_texture_count(AiTextureType::Specular) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Specular)?;
            textures.insert("specular".to_string(), texture);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.specular")?, 0, 0) {
            let texture = Texture2D::from_data(
                1,
                1,
                gl::GL_RGB32F as i32,
                gl::GL_RGB,
                gl::GL_FLOAT,
                &[col.r],
                false,
            )?;
            textures.insert("specular".to_string(), texture);
        }

        /*
        if mat.get_texture_count(AiTextureType::Height) > 0 {
            let tex = mat.get_texture(AiTextureType::Height, 0)?;
            let buf = PathBuf::from(base_path).join(tex);
            let texture = Texture2D::try_from_file(buf, false)?;
            textures.insert("normalmap".to_string(), texture);
        }
        */
        if mat.get_texture_count(AiTextureType::Opacity) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Opacity)?;
            textures.insert("alphamap".to_string(), texture);
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

fn get_texture(
    base_path: &Path,
    mat: &AMaterial<'_>,
    texture_type: AiTextureType,
) -> Result<Texture2D, MaterialConversionError> {
    let tex = mat.get_texture(texture_type, 0)?;
    let buf = PathBuf::from(base_path).join(tex);
    let t = Texture2D::try_from_file(buf, false)?;
    Ok(t)
}
