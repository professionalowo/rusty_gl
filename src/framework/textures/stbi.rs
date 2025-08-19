#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/stbi_bindings.rs"));

use std::num::TryFromIntError;

use crate::gl;

#[derive(Debug)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub format: gl::GLenum,
    pub internal_format: gl::GLint,
    pub type_: gl::GLenum,
    pub data: Box<[u8]>,
}

#[derive(Debug)]
pub enum ImageError {
    StbiError(String),
    CastError(TryFromIntError),
}

impl From<TryFromIntError> for ImageError {
    fn from(err: TryFromIntError) -> Self {
        Self::CastError(err)
    }
}

pub fn loadf(path: impl AsRef<str>) -> Result<ImageData, ImageError> {
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = stbi_loadf(
            path.as_ref().as_ptr() as *const i8,
            &mut width,
            &mut height,
            &mut channels,
            0,
        );
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                failure_reason().unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }
        let data = ptr.cast::<u8>();
        Box::from_raw(std::slice::from_raw_parts_mut(
            data,
            (width * height).try_into()?,
        ))
    };
    let internal_format = match channels {
        4 => gl::GL_RGBA32F,
        3 => gl::GL_RGB32F,
        2 => gl::GL_RG32F,
        1 => gl::GL_R32F,
        _ => gl::GL_R32F,
    };
    let format = format_from_channels(channels);
    Ok(ImageData {
        width: width.try_into()?,
        height: height.try_into()?,
        format,
        data,
        type_: gl::GL_FLOAT,
        internal_format: internal_format.try_into()?,
    })
}

fn format_from_channels(channels: i32) -> gl::GLenum {
    match channels {
        4 => gl::GL_RGBA,
        3 => gl::GL_RGB,
        2 => gl::GL_RG,
        1 => gl::GL_RED,
        _ => gl::GL_RED,
    }
}

pub fn load(path: impl AsRef<str>) -> Result<ImageData, ImageError> {
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = stbi_load(
            path.as_ref().as_ptr() as *const i8,
            &mut width,
            &mut height,
            &mut channels,
            0,
        );
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                failure_reason().unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }
        Box::from_raw(std::slice::from_raw_parts_mut(
            ptr,
            (width * height).try_into()?,
        ))
    };
    let internal_format = match channels {
        1 => gl::GL_RED,
        2 => gl::GL_RG,
        3 => gl::GL_RGB,
        4 => gl::GL_RGBA,
        _ => gl::GL_RED,
    };
    let format = format_from_channels(channels);
    Ok(ImageData {
        width: width.try_into()?,
        height: height.try_into()?,
        format,
        data,
        type_: gl::GL_UNSIGNED_BYTE,
        internal_format: internal_format.try_into()?,
    })
}

pub fn is_hdr(path: impl AsRef<str>) -> bool {
    unsafe { stbi_is_hdr(path.as_ref().as_ptr() as *const i8) != 0 }
}

pub fn failure_reason() -> Option<String> {
    use std::ffi::CStr;

    unsafe {
        let ptr = stbi_failure_reason();
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }
}
