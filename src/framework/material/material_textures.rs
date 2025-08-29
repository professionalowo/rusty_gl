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
    pub const fn new() -> Self {
        Self {
            diffuse: None,
            specular: None,
            normalmap: None,
            alphamap: None,
        }
    }

    pub const fn has_texture(&self, texture_type: MaterialTextureType) -> bool {
        match texture_type {
            MaterialTextureType::Diffuse => self.diffuse.is_some(),
            MaterialTextureType::Specular => self.specular.is_some(),
            MaterialTextureType::NormalMap => self.normalmap.is_some(),
            MaterialTextureType::AlphaMap => self.alphamap.is_some(),
        }
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
            MaterialTextureType::Diffuse => "diffuse",
            MaterialTextureType::Specular => "specular",
            MaterialTextureType::NormalMap => "normalmap",
            MaterialTextureType::AlphaMap => "alphamap",
        }
    }
}
