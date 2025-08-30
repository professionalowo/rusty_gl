use std::{
    ffi::CString,
    fmt,
    path::{Path, PathBuf},
};

use assimp::Color3D;
use assimp_sys::AiTextureType;

use crate::{
    framework::{
        assimp::{AMaterial, AiError},
        material::material_textures::MaterialTextures,
        texture::TextureError,
    },
    gl::{self, program::Program, uniform::UniformLocationError},
    math::{vec3::Vec3, vec4::Vec4},
};

use super::texture::Texture2D;

pub mod material_textures;

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
    pub textures: MaterialTextures,
    pub k_amb: Vec4<f32>,
    pub k_diff: Vec4<f32>,
    pub k_spec: Vec4<f32>,
}

impl Material {
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
        // program.uniform("k_amb", &self.k_amb)?;
        // program.uniform("k_diff", &self.k_diff)?;
        // program.uniform("k_spec", &self.k_spec)?;
        self.textures.bind(program)?;
        Ok(())
    }
    pub fn unbind(&self) {
        self.textures.unbind();
    }

    pub fn from_ai_mesh(
        mat: &AMaterial,
        name: String,
        base_path: &Path,
    ) -> Result<Self, MaterialConversionError> {
        let diff = Vec3::from(mat.get_material_color(CString::new("$clr.diffuse")?, 0, 0)?);
        let k_diff = diff.expand(1.0);

        let spec = Vec3::from(mat.get_material_color(CString::new("$clr.specular")?, 0, 0)?);
        let k_spec = spec.expand(1.0);

        let amb = Vec3::from(mat.get_material_color(CString::new("$clr.ambient")?, 0, 0)?);
        let k_amb = amb.expand(1.0);

        let mut textures = MaterialTextures::default();

        if mat.get_texture_count(AiTextureType::Diffuse) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Diffuse)?;
            textures.diffuse = Some(texture);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.diffuse")?, 0, 0) {
            textures.specular = Some(get_color(col)?);
        }

        if mat.get_texture_count(AiTextureType::Specular) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Specular)?;
            textures.specular = Some(texture);
        } else if let Ok(col) = mat.get_material_color(CString::new("$clr.specular")?, 0, 0) {
            textures.specular = Some(get_color(col)?);
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
            textures.alphamap = Some(texture);
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

fn get_color(col: Color3D) -> Result<Texture2D, TextureError> {
    Texture2D::from_data(
        1,
        1,
        gl::GL_RGB32F as i32,
        gl::GL_RGB,
        gl::GL_FLOAT,
        &[col.r],
        false,
    )
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
