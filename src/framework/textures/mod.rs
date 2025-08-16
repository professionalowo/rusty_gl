use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::gl::{self, GLenum, GLint, GLsizei, GLuint};

mod upload;

pub struct Texture2D {
    id: GLuint,
    width: u32,
    height: u32,
    internal_format: GLint,
    format: GLenum,
    type_: GLenum,
}

pub enum TextureError {
    LoadFailed,
    UnsupportedFormat,
}

impl Texture2D {
    pub fn try_from_file(path: PathBuf, mipmap: bool) -> Result<Self, TextureError> {
        let img = ImageReader::open(path)
            .map_err(|_| TextureError::LoadFailed)?
            .decode()
            .map_err(|_| TextureError::LoadFailed)?;
        let (width, height) = img.dimensions();
        let (internal_format, format, type_, raw) = match img {
            DynamicImage::ImageRgb8(buffer) => (
                GLint::try_from(gl::GL_RGB8).map_err(|_| TextureError::UnsupportedFormat)?,
                gl::GL_RGB,
                gl::GL_UNSIGNED_BYTE,
                buffer.into_raw(),
            ),
            DynamicImage::ImageRgba8(buffer) => (
                GLint::try_from(gl::GL_RGBA8).map_err(|_| TextureError::UnsupportedFormat)?,
                gl::GL_RGBA,
                gl::GL_UNSIGNED_BYTE,
                buffer.into_raw(),
            ),
            _ => return Err(TextureError::UnsupportedFormat),
        };
        let mut texture = Self {
            id: 0,
            width,
            height,
            internal_format,
            format,
            type_,
        };
        texture.upload(&raw, mipmap);
        Ok(texture)
    }

    fn upload(&mut self, data: &[u8], mipmap: bool) {
        upload::gen_textures(&mut self.id);
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        upload::pixel_storei(gl::GL_UNPACK_ALIGNMENT, 1);
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_WRAP_S,
            gl::GL_REPEAT as GLint,
        );
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_WRAP_R,
            gl::GL_REPEAT as GLint,
        );
        let depth = self.internal_format == gl::GL_DEPTH_COMPONENT as i32
            || self.internal_format == gl::GL_DEPTH_COMPONENT16 as i32
            || self.internal_format == gl::GL_DEPTH_COMPONENT24 as i32
            || self.internal_format == gl::GL_DEPTH_COMPONENT32 as i32;
        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_MAG_FILTER,
            if depth {
                gl::GL_NEAREST as GLint
            } else {
                gl::GL_LINEAR as GLint
            },
        );

        upload::tex_parameteri(
            gl::GL_TEXTURE_2D,
            gl::GL_TEXTURE_MIN_FILTER,
            if mipmap {
                gl::GL_LINEAR_MIPMAP_LINEAR as GLint
            } else if depth {
                gl::GL_NEAREST as GLint
            } else {
                gl::GL_LINEAR as GLint
            },
        );

        upload::tex_image_2d(
            gl::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width as GLsizei,
            self.height as GLsizei,
            0,
            self.format,
            self.type_,
            data.as_ptr() as *const std::ffi::c_void,
        );

        if mipmap {
            upload::generate_mipmap(gl::GL_TEXTURE_2D);
        }
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
    }

    pub fn bind(&self, unit: u32) {
        upload::active_texture(gl::GL_TEXTURE0 + unit);
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
    }

    pub fn unbind(&self) {
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        upload::tex_image_2d(
            gl::GL_TEXTURE_2D,
            0,
            self.internal_format,
            self.width as GLsizei,
            self.height as GLsizei,
            0,
            self.format,
            self.type_,
            std::ptr::null(),
        );
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
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
