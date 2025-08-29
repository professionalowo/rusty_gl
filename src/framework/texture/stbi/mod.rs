#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/stbi_bindings.rs"));

use core::fmt;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use memmap2::Mmap;

use crate::gl;

mod load;
mod load_trait;

#[derive(Debug)]
pub(super) struct ImageData {
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
    pub fn try_load(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let file = &File::open(&path)?;
        let ref data = unsafe { Mmap::map(file) }?;
        if is_hdr(data) {
            load::try_loadf(data)
        } else {
            load::try_load(data)
        }
    }
}

pub fn is_hdr(data: &[u8]) -> bool {
    unsafe { stbi_is_hdr_from_memory(data.as_ptr(), data.len() as i32) != 0 }
}
