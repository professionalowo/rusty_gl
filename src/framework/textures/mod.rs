use crate::gl::{self, GLenum, GLint, GLsizei, GLuint};

pub struct Texture2D {
    id: GLuint,
    width: u32,
    height: u32,
    internal_format: GLint,
    format: GLenum,
    type_: GLenum,
}

impl Texture2D {
    pub fn bind(&self, unit: u32) {
        active_texture(gl::GL_TEXTURE0 + unit);
        bind_texture(gl::GL_TEXTURE_2D, self.id);
    }

    pub fn unbind(&self) {
        bind_texture(gl::GL_TEXTURE_2D, 0);
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
        bind_texture(gl::GL_TEXTURE_2D, self.id);
        tex_image_2d(
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
        bind_texture(gl::GL_TEXTURE_2D, 0);
    }
}

fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        gl::glBindTexture(target, texture);
    }
}

fn active_texture(unit: u32) {
    unsafe {
        gl::glActiveTexture(gl::GL_TEXTURE0 + unit);
    }
}

fn tex_image_2d(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const std::ffi::c_void,
) {
    unsafe {
        gl::glTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        );
    }
}
