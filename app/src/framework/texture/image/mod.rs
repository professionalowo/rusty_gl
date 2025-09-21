use core::fmt;
use std::{fs, num::TryFromIntError, path::Path};

use stbi_sys::{
    dimensions::Dimensions,
    load::{Load, LoadData, LoadError, LoadFloat, LoadInt},
};

use super::image::{format::Format, map_channels::MapChannels};

mod format;
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
    pub fn try_load<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        fs::read(path).map(Self::try_load_from_memory)?
    }

    pub fn try_load_from_memory<B: AsRef<[u8]>>(data: B) -> ImageResult<Self> {
        let bytes = data.as_ref();
        if stbi_sys::is_hdr(bytes) {
            load_from_memory::<LoadFloat>(bytes)
        } else {
            load_from_memory::<LoadInt>(bytes)
        }
    }
}

fn load_from_memory<L: Load + MapChannels>(bytes: &[u8]) -> ImageResult<GlImageData> {
    let LoadData {
        ref channels,
        data,
        dimensions: Dimensions { height, width },
    } = LoadData::try_load::<L>(bytes)?;
    let Format {
        format,
        internal_format,
    } = Format::try_from_channels::<L>(channels)?;
    Ok(GlImageData {
        width,
        height,
        format,
        data: data.into_boxed_slice(),
        type_: L::TYPE.data(),
        internal_format,
    })
}
