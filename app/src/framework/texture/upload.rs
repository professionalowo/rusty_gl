pub(super) fn bind_texture(target: gl_sys::bindings::GLenum, texture: gl_sys::bindings::GLuint) {
    unsafe {
        gl_sys::bindings::glBindTexture(target, texture);
    }
}

pub(super) fn gen_textures(texture: &mut gl_sys::bindings::GLuint) {
    unsafe {
        gl_sys::bindings::glGenTextures(1, texture);
    }
}

pub(super) fn pixel_storei(pname: gl_sys::bindings::GLenum, param: gl_sys::bindings::GLint) {
    unsafe {
        gl_sys::bindings::glPixelStorei(pname, param);
    }
}

pub(super) fn tex_parameteri(
    target: gl_sys::bindings::GLenum,
    pname: gl_sys::bindings::GLenum,
    param: gl_sys::bindings::GLint,
) {
    unsafe {
        gl_sys::bindings::glTexParameteri(target, pname, param);
    }
}

pub(super) fn active_texture(unit: u32) {
    unsafe {
        gl_sys::bindings::glActiveTexture(gl_sys::bindings::GL_TEXTURE0 + unit);
    }
}

pub(super) fn generate_mipmap(target: gl_sys::bindings::GLenum) {
    unsafe {
        gl_sys::bindings::glGenerateMipmap(target);
    }
}

pub(super) fn tex_image_2d(
    target: gl_sys::bindings::GLenum,
    level: gl_sys::bindings::GLint,
    internalformat: gl_sys::bindings::GLint,
    width: gl_sys::bindings::GLsizei,
    height: gl_sys::bindings::GLsizei,
    border: gl_sys::bindings::GLint,
    format: gl_sys::bindings::GLenum,
    type_: gl_sys::bindings::GLenum,
    pixels: *const std::ffi::c_void,
) {
    unsafe {
        gl_sys::bindings::glTexImage2D(
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
