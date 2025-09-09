use super::{gl, load_trait::Load};
use std::num::TryFromIntError;

#[derive(Debug, Default)]
pub struct Channels(pub i32);

impl Channels {
    #[inline]
    pub const fn format(&self) -> gl::GLenum {
        match self.0 {
            4 => gl::GL_RGBA,
            3 => gl::GL_RGB,
            2 => gl::GL_RG,
            1 | _ => gl::GL_RED,
        }
    }
}

impl From<i32> for Channels {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Format {
    pub format: u32,
    pub internal_format: i32,
}

impl Format {
    pub fn try_from_load<L: Load>(channels: &Channels) -> Result<Self, TryFromIntError> {
        let internal_format = i32::try_from(L::map_channels(channels))?;
        Ok(Self {
            format: channels.format(),
            internal_format,
        })
    }
}
