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
    mipmap: bool,
    data: Box<[u8]>,
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
        Ok(Self {
            id: 0,
            width,
            height,
            internal_format,
            format,
            type_,
            mipmap,
            data: raw.into_boxed_slice(),
        })
    }

    pub fn upload(&mut self) {
        upload::gen_textures(&mut self.id);
        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        upload::pixel_storei(gl::GL_UNPACK_ALIGNMENT, 1);
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
