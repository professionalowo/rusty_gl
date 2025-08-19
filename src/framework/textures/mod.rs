use crate::{
    framework::textures::stbi::ImageData,
    gl::{self, GLenum, GLint, GLsizei, GLuint},
};

mod stbi;
mod upload;

pub struct Texture2D {
    id: GLuint,
    width: u32,
    height: u32,
    internal_format: GLint,
    format: GLenum,
    type_: GLenum,
}

#[derive(Debug)]
pub enum TextureError {
    LoadFailed(stbi::ImageError),
    GLError(gl::GLError),
}

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

impl Texture2D {
    pub fn try_from_file(path: impl AsRef<str>, mipmap: bool) -> Result<Self, TextureError> {
        let p = path.as_ref();
        let data = ImageData::load(p)?;
        let texture = upload_image_data(data, mipmap)?;
        Ok(texture)
    }

    fn upload(&mut self, data: &[u8], mipmap: bool) -> Result<(), gl::GLError> {
        upload::gen_textures(&mut self.id);
        gl::get_error()?;

        upload::bind_texture(gl::GL_TEXTURE_2D, self.id);
        gl::get_error()?;
        upload::pixel_storei(gl::GL_UNPACK_ALIGNMENT, 1);
        gl::get_error()?;
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
        gl::get_error()?;

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
        gl::get_error()?;

        if mipmap {
            upload::generate_mipmap(gl::GL_TEXTURE_2D);
            gl::get_error()?;
        }
        upload::bind_texture(gl::GL_TEXTURE_2D, 0);
        gl::get_error()?;
        Ok(())
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

fn upload_image_data(data: ImageData, mipmap: bool) -> Result<Texture2D, gl::GLError> {
    let ImageData {
        width,
        height,
        format,
        internal_format,
        type_,
        ref data,
    } = data;
    let mut texture = Texture2D {
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

impl Drop for Texture2D {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::glDeleteTextures(1, &self.id);
            }
        }
    }
}
