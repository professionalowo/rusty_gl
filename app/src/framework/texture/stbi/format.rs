use crate::framework::texture::stbi::map_channels::MapChannels;

use stbi_sys::channels::Channels;
use std::num::TryFromIntError;

#[inline]
pub const fn format_channels(Channels(c): &Channels) -> gl_sys::bindings::GLenum {
    match c {
        4 => gl_sys::bindings::GL_RGBA,
        3 => gl_sys::bindings::GL_RGB,
        2 => gl_sys::bindings::GL_RG,
        1 | _ => gl_sys::bindings::GL_RED,
    }
}

#[derive(Debug)]
pub struct Format {
    pub format: u32,
    pub internal_format: i32,
}

impl Format {
    pub fn try_from_load<M: MapChannels>(channels: &Channels) -> Result<Self, TryFromIntError> {
        let internal_format = i32::try_from(M::map_channels(channels))?;
        Ok(Self {
            format: format_channels(channels),
            internal_format,
        })
    }
}
