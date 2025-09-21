use std::{fmt, path::Path};

use crate::framework::texture::image::GlImageData;

use gl_sys::{
    self,
    uniform::{UniformLocation, UniformLocationError, uniform_trait::Uniform},
};

mod image;
mod upload;

#[derive(Debug, Default, Clone)]
pub struct Texture2D {
    id: gl_sys::bindings::GLuint,
    width: gl_sys::bindings::GLsizei,
    height: gl_sys::bindings::GLsizei,
    internal_format: gl_sys::bindings::GLint,
    format: gl_sys::bindings::GLenum,
    type_: gl_sys::bindings::GLenum,
}

#[derive(Debug)]
pub enum TextureError {
    LoadFailed(image::ImageError),
    GLError(gl_sys::GLError),
}

impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Texture2D {}

impl From<gl_sys::GLError> for TextureError {
    fn from(err: gl_sys::GLError) -> Self {
        Self::GLError(err)
    }
}

impl From<image::ImageError> for TextureError {
    fn from(err: image::ImageError) -> Self {
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
        width: gl_sys::bindings::GLsizei,
        height: gl_sys::bindings::GLsizei,
        internal_format: gl_sys::bindings::GLint,
        format: gl_sys::bindings::GLenum,
        type_: gl_sys::bindings::GLenum,
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
        gl_sys::get_error()?;
        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, self.id);
        gl_sys::get_error()?;
        Ok(())
    }

    pub fn unbind(&self) {
        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, 0);
    }

    pub fn resize(&mut self, w: gl_sys::bindings::GLsizei, h: gl_sys::bindings::GLsizei) {
        self.width = w;
        self.height = h;
        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, self.id);
        upload::tex_image_2d(
            gl_sys::bindings::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width,
            self.height,
            0,
            self.format,
            self.type_,
            std::ptr::null(),
        );
        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, 0);
    }

    fn upload<T>(&mut self, data: &[T], mipmap: bool) -> Result<(), gl_sys::GLError> {
        upload::gen_textures(&mut self.id);
        gl_sys::get_error()?;

        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, self.id);
        gl_sys::get_error()?;
        upload::pixel_storei(gl_sys::bindings::GL_UNPACK_ALIGNMENT, 1);
        gl_sys::get_error()?;
        upload::tex_parameteri(
            gl_sys::bindings::GL_TEXTURE_2D,
            gl_sys::bindings::GL_TEXTURE_WRAP_S,
            gl_sys::bindings::GL_REPEAT as gl_sys::bindings::GLint,
        );
        upload::tex_parameteri(
            gl_sys::bindings::GL_TEXTURE_2D,
            gl_sys::bindings::GL_TEXTURE_WRAP_R,
            gl_sys::bindings::GL_REPEAT as gl_sys::bindings::GLint,
        );
        upload::tex_parameteri(
            gl_sys::bindings::GL_TEXTURE_2D,
            gl_sys::bindings::GL_TEXTURE_MAG_FILTER,
            gl_sys::bindings::GL_NEAREST as gl_sys::bindings::GLint,
        );

        upload::tex_parameteri(
            gl_sys::bindings::GL_TEXTURE_2D,
            gl_sys::bindings::GL_TEXTURE_MIN_FILTER,
            if mipmap {
                gl_sys::bindings::GL_LINEAR_MIPMAP_LINEAR as gl_sys::bindings::GLint
            } else {
                gl_sys::bindings::GL_LINEAR as gl_sys::bindings::GLint
            },
        );
        gl_sys::get_error()?;

        upload::tex_image_2d(
            gl_sys::bindings::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width as gl_sys::bindings::GLsizei,
            self.height as gl_sys::bindings::GLsizei,
            0,
            self.format,
            self.type_,
            data.as_ptr() as *const std::ffi::c_void,
        );
        gl_sys::get_error()?;

        if mipmap {
            upload::generate_mipmap(gl_sys::bindings::GL_TEXTURE_2D);
            gl_sys::get_error()?;
        }
        upload::bind_texture(gl_sys::bindings::GL_TEXTURE_2D, 0);
        gl_sys::get_error()?;
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
    ) -> Result<Self, gl_sys::GLError> {
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
                gl_sys::bindings::glDeleteTextures(1, &self.id);
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
            gl_sys::bindings::glUniform1i(location, unit as i32);
        }
    }
}
