use stbi_sys::{
    channels::Channels,
    load::{LoadFloat, LoadInt},
};
pub(super) enum GlType {
    Float,
    UnsignedByte,
}

impl GlType {
    pub const fn data(&self) -> u32 {
        match self {
            Self::Float => gl_sys::bindings::GL_FLOAT,
            Self::UnsignedByte => gl_sys::bindings::GL_UNSIGNED_BYTE,
        }
    }
}

pub(super) trait MapChannels {
    const TYPE: GlType;

    fn map_channels(channels: &Channels) -> u32;
}

impl MapChannels for LoadFloat {
    const TYPE: GlType = GlType::Float;

    #[inline]
    fn map_channels(Channels(channels): &Channels) -> u32 {
        match channels {
            4 => gl_sys::bindings::GL_RGBA32F,
            3 => gl_sys::bindings::GL_RGB32F,
            2 => gl_sys::bindings::GL_RG32F,
            1 => gl_sys::bindings::GL_R32F,
            _ => gl_sys::bindings::GL_R32F,
        }
    }
}

impl MapChannels for LoadInt {
    const TYPE: GlType = GlType::UnsignedByte;
    #[inline]
    fn map_channels(Channels(channels): &Channels) -> u32 {
        match channels {
            1 => gl_sys::bindings::GL_RED,
            2 => gl_sys::bindings::GL_RG,
            3 => gl_sys::bindings::GL_RGB,
            4 => gl_sys::bindings::GL_RGBA,
            _ => gl_sys::bindings::GL_RED,
        }
    }
}
