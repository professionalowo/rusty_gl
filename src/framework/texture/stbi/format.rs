use super::{gl, load_trait::Load};
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct Format {
    pub format: u32,
    pub internal_format: i32,
}

impl Format {
    pub fn try_from_load<L: Load>(channels: i32) -> Result<Self, TryFromIntError> {
        let internal_format = i32::try_from(L::map_channels(channels))?;
        let format = format_from_channels(channels);
        Ok(Self {
            format,
            internal_format,
        })
    }
}

#[inline]
const fn format_from_channels(channels: i32) -> gl::GLenum {
    match channels {
        4 => gl::GL_RGBA,
        3 => gl::GL_RGB,
        2 => gl::GL_RG,
        1 | _ => gl::GL_RED,
    }
}
