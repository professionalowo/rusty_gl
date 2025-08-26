#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/stbi_bindings.rs"));

use core::fmt;
use std::{fs::File, path::PathBuf};

use memmap2::Mmap;

use crate::gl;

#[derive(Debug)]
pub struct ImageData {
    pub width: gl::GLsizei,
    pub height: gl::GLsizei,
    pub format: gl::GLenum,
    pub internal_format: gl::GLint,
    pub type_: gl::GLenum,
    pub data: Box<[u8]>,
}

#[derive(Debug)]
pub enum ImageError {
    IoError(std::io::Error),
    CastError(std::num::TryFromIntError),
    StbiError(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => fmt::Display::fmt(err, f),
            Self::CastError(err) => fmt::Display::fmt(err, f),
            Self::StbiError(err) => write!(f, "STBI Error: {}", err),
        }
    }
}

impl From<std::num::TryFromIntError> for ImageError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::CastError(err)
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl ImageData {
    pub fn try_load(path: PathBuf) -> Result<Self, ImageError> {
        let file = &File::open(&path)?;
        let ref data = unsafe { Mmap::map(file) }?;
        if is_hdr(data) {
            try_loadf(data)
        } else {
            try_load(data)
        }
    }
}

fn try_loadf(bytes: &[u8]) -> Result<ImageData, ImageError> {
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = stbi_loadf_from_memory(
            bytes.as_ptr(),
            bytes.len() as i32,
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
        width,
        height,
        format,
        data,
        type_: gl::GL_FLOAT,
        internal_format: internal_format.try_into()?,
    })
}

fn try_load(bytes: &[u8]) -> Result<ImageData, ImageError> {
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = stbi_load_from_memory(
            bytes.as_ptr(),
            bytes.len() as i32,
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
        width,
        height,
        format,
        data,
        type_: gl::GL_UNSIGNED_BYTE,
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

pub fn is_hdr(data: &[u8]) -> bool {
    unsafe { stbi_is_hdr_from_memory(data.as_ptr(), data.len() as i32) != 0 }
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
