use self::dimensions::Dimensions;
use super::{format::Channels, *};
use stbi_sys::bindings::*;
use std::ffi::c_int;
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

pub(super) trait Load {
    const TYPE: GlType;

    fn map_channels(channels: &Channels) -> u32;

    unsafe fn load(
        bytes: impl AsRef<[u8]>,
        Dimensions { width, height }: &mut Dimensions,
        Channels(channels): &mut Channels,
    ) -> *const u8 {
        let buffer = bytes.as_ref();
        unsafe {
            Self::load_from_memory(
                buffer.as_ptr(),
                buffer.len() as i32,
                width,
                height,
                channels,
                0,
            )
            .cast_const()
        }
    }

    unsafe fn load_from_memory(
        buffer: *const stbi_uc,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut u8;
}

pub(super) struct LoadFloat;

impl Load for LoadFloat {
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

    #[inline]
    unsafe fn load_from_memory(
        buffer: *const stbi_uc,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut u8 {
        unsafe { stbi_loadf_from_memory(buffer, len, x, y, channels_in_file, desired_channels) }
            .cast::<u8>()
    }
}

pub(super) struct LoadInt;

impl Load for LoadInt {
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

    #[inline]
    unsafe fn load_from_memory(
        buffer: *const stbi_uc,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut u8 {
        unsafe { stbi_load_from_memory(buffer, len, x, y, channels_in_file, desired_channels) }
    }
}
