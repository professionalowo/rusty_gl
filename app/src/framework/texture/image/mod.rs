use std::{fmt, fs, io, num, path::Path};

use stbi_sys::{
    dimensions::Dimensions,
    load::{Load, LoadData, LoadError, LoadFloat, LoadInt, LoadOptions},
    stbi_ptr::StbiPtr,
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
    pub data: StbiPtr<u8>,
}

#[derive(Debug)]
pub enum ImageError {
    IoError(io::Error),
    TryFromIntError(num::TryFromIntError),
    LoadError(LoadError),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => fmt::Display::fmt(err, f),
            Self::LoadError(err) => fmt::Display::fmt(err, f),
            Self::TryFromIntError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl From<num::TryFromIntError> for ImageError {
    fn from(value: num::TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl From<LoadError> for ImageError {
    fn from(value: LoadError) -> Self {
        Self::LoadError(value)
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

pub type ImageResult<T> = std::result::Result<T, ImageError>;

impl GlImageData {
    pub fn try_load<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        fs::read(path).map(Self::try_load_from_memory)?
    }

    pub fn try_load_from_memory<B: AsRef<[u8]>>(data: B) -> ImageResult<Self> {
        if stbi_sys::is_hdr(&data) {
            load_from_memory::<LoadFloat, _>(&data)
        } else {
            load_from_memory::<LoadInt, _>(&data)
        }
    }
}

fn load_from_memory<L: Load + MapChannels, B: AsRef<[u8]>>(bytes: B) -> ImageResult<GlImageData> {
    let LoadData {
        ref channels,
        data,
        dimensions: Dimensions { height, width },
    } = LoadData::try_load::<L, _>(bytes, LoadOptions::default())?;
    let Format {
        format,
        internal_format,
    } = Format::try_from_channels::<L>(channels)?;
    Ok(GlImageData {
        width,
        height,
        format,
        data,
        type_: L::TYPE.data(),
        internal_format,
    })
}
