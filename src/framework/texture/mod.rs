use std::{fmt, path::Path};

use crate::{
    framework::texture::stbi::GlImageData,
    gl::{
        self,
        uniform::{UniformLocation, UniformLocationError, uniform_trait::Uniform},
    },
};

mod stbi;
mod upload;

#[derive(Debug, Default, Clone)]
pub struct Texture2D {
    id: gl::GLuint,
    width: gl::GLsizei,
    height: gl::GLsizei,
    internal_format: gl::GLint,
    format: gl::GLenum,
    type_: gl::GLenum,
}

#[derive(Debug)]
pub enum TextureError {
    LoadFailed(stbi::ImageError),
    GLError(gl::GLError),
}

impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Texture2D {}

impl From<gl::GLError> for TextureError {
    fn from(err: gl::GLError) -> Self {
        Self::GLError(err)
    }
}

impl From<stbi::ImageError> for TextureError {
    fn from(err: stbi::ImageError) -> Self {
        Self::LoadFailed(err)
    }
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(err) => fmt::Display::fmt(err, f),
            Self::GLError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Texture2D {
    pub fn try_from_file(path: impl AsRef<Path>, mipmap: bool) -> Result<Self, TextureError> {
        let texture = Self::upload_image_data(GlImageData::try_load(path)?, mipmap)?;
        Ok(texture)
    }

    pub fn from_data<T>(
        width: gl::GLsizei,
        height: gl::GLsizei,
        internal_format: gl::GLint,
        format: gl::GLenum,
        type_: gl::GLenum,
        data: &[T],
        mipmap: bool,
    ) -> Result<Self, TextureError> {
        let mut s = Self {
            id: 0,
            width,
            height,
            format,
            internal_format,
            type_,
        };
        s.upload(data, mipmap)?;
        Ok(s)
    }

    pub fn bind(&self, unit: u32) -> Result<(), UniformLocationError> {
        upload::active_texture(unit);
        gl::get_error()?;
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        gl::get_error()?;
        Ok(())
    }

    pub fn unbind(&self) {
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
    }

    pub fn resize(&mut self, w: gl::GLsizei, h: gl::GLsizei) {
        self.width = w;
        self.height = h;
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        upload::tex_image_2d(
            gl::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width,
            self.height,
            0,
            self.format,
            self.type_,
            std::ptr::null(),
        );
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
    }

    fn upload<T>(&mut self, data: &[T], mipmap: bool) -> Result<(), gl::GLError> {
        upload::gen_textures(&mut self.id);
        gl::get_error()?;

        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        gl::get_error()?;
        upload::pixel_storei(gl::GL_UNPACK_ALIGNMENT, 1);
        gl::get_error()?;
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_WRAP_S,
            gl::GL_REPEAT as gl::GLint,
        );
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_WRAP_R,
            gl::GL_REPEAT as gl::GLint,
        );
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_MAG_FILTER,
            gl::GL_NEAREST as gl::GLint,
        );

        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_MIN_FILTER,
            if mipmap {
                gl::GL_LINEAR_MIPMAP_LINEAR as gl::GLint
            } else {
                gl::GL_LINEAR as gl::GLint
            },
        );
        gl::get_error()?;

        upload::tex_image_2d(
            gl::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width as gl::GLsizei,
            self.height as gl::GLsizei,
            0,
            self.format,
            self.type_,
            data.as_ptr() as *const std::ffi::c_void,
        );
        gl::get_error()?;

        if mipmap {
            upload::generate_mipmap(gl::GL_TEXTURE_2D);
            gl::get_error()?;
        }
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
        gl::get_error()?;
        Ok(())
    }

    fn upload_image_data(
        GlImageData {
            width,
            height,
            format,
            internal_format,
            type_,
            ref data,
        }: GlImageData,
        mipmap: bool,
    ) -> Result<Self, gl::GLError> {
        let mut texture = Self {
            id: 0,
            width,
            height,
            internal_format,
            format,
            type_,
        };
        texture.upload(data, mipmap)?;
        Ok(texture)
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::glDeleteTextures(1, &self.id);
            }
        }
    }
}

impl Uniform for &Texture2D {
    type Options = u32; // texture unit

    fn set(&self, options: Option<Self::Options>, location: &UniformLocation) {
        let unit = options.unwrap_or(0);
        let UniformLocation(location) = *location;
        self.bind(unit).expect("Couldnt bind texture");
        unsafe {
            gl::glUniform1i(location, unit as i32);
        }
    }
}
