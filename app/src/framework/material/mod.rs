use std::{
    fmt,
    path::{Path, PathBuf},
};

use assimp_sys::AiTextureType;
use material_color::material_color;

use super::texture::Texture2D;
use crate::{
    assimp::{AMaterial, AiError, material_key::MaterialKey},
    framework::{material::material_textures::MaterialTextures, texture::TextureError},
};
use gl_sys::{self, program::Program, uniform::UniformLocationError};
use rmath::{vec3::Vec3, vec4::Vec4};

mod material_color;
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

#[derive(Debug)]
pub struct Material {
    //pub name: String,
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

    pub fn from_ai_material(
        mat: &AMaterial,
        base_path: &Path,
    ) -> Result<Self, MaterialConversionError> {
        let k_diff = get_plain_color(mat, AiTextureType::Diffuse)?;

        let k_spec = get_plain_color(mat, AiTextureType::Specular)?;

        let k_amb = get_plain_color(mat, AiTextureType::Ambient)?;

        let diffuse = get_texture_option(AiTextureType::Diffuse, mat, base_path)?;

        let specular = get_texture_option(AiTextureType::Specular, mat, base_path)?;

        let normalmap = None;
        /*
        let normalmap = if mat.get_texture_count(AiTextureType::Height) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Height)?;
            Some(texture)
        } else {
            None
        };
        */

        let alphamap = if mat.get_texture_count(AiTextureType::Opacity) > 0 {
            let texture = get_texture(base_path, mat, AiTextureType::Opacity)?;
            Some(texture)
        } else {
            None
        };

        let textures = MaterialTextures {
            diffuse,
            alphamap,
            normalmap,
            specular,
        };

        Ok(Self {
            textures,
            k_amb,
            k_diff,
            k_spec,
        })
    }
}

fn get_color<C>(col: C) -> Result<Texture2D, TextureError>
where
    C: Into<Vec3<f32>>,
{
    let col: Vec3<f32> = col.into();

    Texture2D::from_data(
        1,
        1,
        gl_sys::bindings::GL_RGB32F as _,
        gl_sys::bindings::GL_RGB,
        gl_sys::bindings::GL_FLOAT,
        &[col],
        false,
    )
}

fn get_plain_color<K>(mat: &AMaterial<'_>, key: K) -> Result<Vec4<f32>, MaterialConversionError>
where
    K: MaterialKey,
{
    Ok(Vec3::from(material_color(mat, key)?).expand(1.0))
}

fn get_texture_option(
    texture_type: AiTextureType,
    mat: &AMaterial<'_>,
    base_path: &Path,
) -> Result<Option<Texture2D>, MaterialConversionError> {
    let text = if mat.get_texture_count(texture_type) > 0 {
        let texture = get_texture(base_path, mat, texture_type)?;
        Some(texture)
    } else if let Ok(col) = material_color(mat, texture_type) {
        Some(get_color(col)?)
    } else {
        None
    };
    Ok(text)
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
