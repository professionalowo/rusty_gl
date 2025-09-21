use core::fmt;
use std::{fs, path::Path};

mod dimensions;
mod format;
mod load;
mod load_trait;

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

pub type ImageResult<T> = std::result::Result<T, ImageError>;

impl GlImageData {
    pub fn try_load(path: impl AsRef<Path>) -> ImageResult<Self> {
        fs::read(path).map(Self::try_load_from_memory)?
    }

    pub fn try_load_from_memory(data: impl AsRef<[u8]>) -> ImageResult<Self> {
        let bytes = data.as_ref();
        if load::is_hdr(bytes) {
            load::try_loadf(bytes)
        } else {
            load::try_load(bytes)
        }
    }
}

#[cfg(test)]
mod tests {}
