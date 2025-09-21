use core::fmt;
use std::{fs, num::TryFromIntError, path::Path};

use stbi_sys::load::LoadError;

mod format;
mod load;
mod map_channels;

#[derive(Debug, PartialEq)]
pub(super) struct GlImageData {
    pub width: gl_sys::bindings::GLsizei,
    pub height: gl_sys::bindings::GLsizei,
    pub format: gl_sys::bindings::GLenum,
    pub internal_format: gl_sys::bindings::GLint,
    pub type_: gl_sys::bindings::GLenum,
    pub data: Box<[u8]>,
}

#[derive(Debug)]
pub enum ImageError {
    IoError(std::io::Error),
    TryFromIntError(TryFromIntError),
    LoadError(LoadError),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => fmt::Display::fmt(err, f),
            Self::LoadError(err) => fmt::Display::fmt(err, f),
            Self::TryFromIntError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<TryFromIntError> for ImageError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl From<LoadError> for ImageError {
    fn from(value: LoadError) -> Self {
        Self::LoadError(value)
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

pub type ImageResult<T> = std::result::Result<T, ImageError>;

impl GlImageData {
    pub fn try_load(path: impl AsRef<Path>) -> ImageResult<Self> {
        fs::read(path).map(Self::try_load_from_memory)?
    }

    pub fn try_load_from_memory(data: impl AsRef<[u8]>) -> ImageResult<Self> {
        let bytes = data.as_ref();
        if stbi_sys::is_hdr(bytes) {
            load::try_loadf(bytes)
        } else {
            load::try_load(bytes)
        }
    }
}
