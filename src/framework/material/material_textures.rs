use std::ops::Index;

use crate::{
    framework::texture::Texture2D,
    gl::{program::Program, uniform::UniformLocationError},
};

#[derive(Debug, Default, Clone)]
pub struct MaterialTextures {
    pub diffuse: Option<Texture2D>,
    pub specular: Option<Texture2D>,
    pub normalmap: Option<Texture2D>,
    pub alphamap: Option<Texture2D>,
}

impl MaterialTextures {
    pub fn has_texture(&self, texture_type: MaterialTextureType) -> bool {
        self[texture_type].is_some()
    }

    pub fn bind(&self, program: &Program) -> Result<(), UniformLocationError> {
        bind_texture_option(
            program,
            self.diffuse.as_ref(),
            MaterialTextureType::Diffuse,
            0,
        )?;
        bind_texture_option(
            program,
            self.specular.as_ref(),
            MaterialTextureType::Specular,
            1,
        )?;
        bind_texture_option(
            program,
            self.normalmap.as_ref(),
            MaterialTextureType::NormalMap,
            2,
        )?;
        bind_texture_option(
            program,
            self.alphamap.as_ref(),
            MaterialTextureType::AlphaMap,
            3,
        )?;
        Ok(())
    }
    pub fn unbind(&self) {
        let _ = &self.diffuse.as_ref().inspect(|t| t.unbind());
        let _ = &self.specular.as_ref().inspect(|t| t.unbind());
        let _ = &self.normalmap.as_ref().inspect(|t| t.unbind());
        let _ = &self.alphamap.as_ref().inspect(|t| t.unbind());
    }
}

impl Index<MaterialTextureType> for MaterialTextures {
    type Output = Option<Texture2D>;
    fn index(&self, index: MaterialTextureType) -> &Self::Output {
        match index {
            MaterialTextureType::AlphaMap => &self.alphamap,
            MaterialTextureType::Diffuse => &self.diffuse,
            MaterialTextureType::NormalMap => &self.normalmap,
            MaterialTextureType::Specular => &self.specular,
        }
    }
}

fn bind_texture_option(
    program: &Program,
    texture: Option<&Texture2D>,
    texture_type: MaterialTextureType,
    unit: u32,
) -> Result<(), UniformLocationError> {
    if let Some(texture) = texture {
        texture.bind(unit)?;
        program.uniform_opt(texture_type, texture, unit)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum MaterialTextureType {
    Diffuse,
    Specular,
    NormalMap,
    AlphaMap,
}

impl AsRef<str> for MaterialTextureType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Diffuse => "diffuse",
            Self::Specular => "specular",
            Self::NormalMap => "normalmap",
            Self::AlphaMap => "alphamap",
        }
    }
}
