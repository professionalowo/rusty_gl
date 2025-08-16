use crate::gl::{self, GLenum, GLint, GLsizei, GLuint};

pub(super) fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        gl::glBindTexture(target, texture);
    }
}

pub(super) fn gen_textures(texture: &mut GLuint) {
    unsafe {
        gl::glGenTextures(1, texture);
    }
}

pub(super) fn pixel_storei(pname: GLenum, param: GLint) {
    unsafe {
        gl::glPixelStorei(pname, param);
    }
}

pub(super) fn tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        gl::glTexParameteri(target, pname, param);
    }
}

pub(super) fn active_texture(unit: u32) {
    unsafe {
        gl::glActiveTexture(gl::GL_TEXTURE0 + unit);
    }
}

pub(super) fn generate_mipmap(target: GLenum) {
    unsafe {
        gl::glGenerateMipmap(target);
    }
}

pub(super) fn tex_image_2d(
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
