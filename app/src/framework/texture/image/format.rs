use super::map_channels::MapChannels;

use stbi_sys::channels::Channels;
use std::num::TryFromIntError;

#[inline(always)]
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
    pub fn try_from_channels<M: MapChannels>(channels: &Channels) -> Result<Self, TryFromIntError> {
        let internal_format = M::map_channels(channels).try_into()?;
        let format = format_channels(channels);
        Ok(Self {
            format,
            internal_format,
        })
    }
}
