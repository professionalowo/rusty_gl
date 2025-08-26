use self::stbi_uc;
use super::*;
use std::ffi::c_int;
pub trait Load {
    const TYPE: u32;

    fn map_channels(channels: i32) -> u32;
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
    const TYPE: u32 = gl::GL_FLOAT;

    #[inline]
    fn map_channels(channels: i32) -> u32 {
        match channels {
            4 => gl::GL_RGBA32F,
            3 => gl::GL_RGB32F,
            2 => gl::GL_RG32F,
            1 => gl::GL_R32F,
            _ => gl::GL_R32F,
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
    const TYPE: u32 = gl::GL_UNSIGNED_BYTE;
    #[inline]
    fn map_channels(channels: i32) -> u32 {
        match channels {
            1 => gl::GL_RED,
            2 => gl::GL_RG,
            3 => gl::GL_RGB,
            4 => gl::GL_RGBA,
            _ => gl::GL_RED,
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
