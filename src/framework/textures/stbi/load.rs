use std::ffi::c_int;

use super::*;

pub(super) fn try_loadf(bytes: &[u8]) -> Result<ImageData, ImageError> {
    try_load_opt::<LoadFloat>(bytes)
}

pub(super) fn try_load(bytes: &[u8]) -> Result<ImageData, ImageError> {
    try_load_opt::<LoadInt>(bytes)
}

fn try_load_opt<L: Load>(bytes: &[u8]) -> Result<ImageData, ImageError> {
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = L::load_from_memory(
            bytes.as_ptr(),
            bytes.len() as i32,
            &mut width,
            &mut height,
            &mut channels,
            0,
        );
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                failure_reason().unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }
        Box::from_raw(std::slice::from_raw_parts_mut(
            ptr,
            (width * height * channels).try_into()?,
        ))
    };
    let internal_format = L::map_channels(channels);
    let format = format_from_channels(channels);
    Ok(ImageData {
        width,
        height,
        format,
        data,
        type_: gl::GL_FLOAT,
        internal_format: internal_format.try_into()?,
    })
}

fn format_from_channels(channels: i32) -> gl::GLenum {
    match channels {
        4 => gl::GL_RGBA,
        3 => gl::GL_RGB,
        2 => gl::GL_RG,
        1 => gl::GL_RED,
        _ => gl::GL_RED,
    }
}

trait Load {
    fn map_channels(channels: i32) -> u32;
    unsafe fn load_from_memory(
        buffer: *const stbi_uc,
        len: ::std::os::raw::c_int,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        channels_in_file: *mut ::std::os::raw::c_int,
        desired_channels: ::std::os::raw::c_int,
    ) -> *mut u8;
}

struct LoadFloat;

impl Load for LoadFloat {
    fn map_channels(channels: i32) -> u32 {
        match channels {
            4 => gl::GL_RGBA32F,
            3 => gl::GL_RGB32F,
            2 => gl::GL_RG32F,
            1 => gl::GL_R32F,
            _ => gl::GL_R32F,
        }
    }

    unsafe fn load_from_memory(
        buffer: *const stbi_uc,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut u8 {
        let ptr = unsafe {
            stbi_loadf_from_memory(buffer, len, x, y, channels_in_file, desired_channels)
        };
        ptr.cast::<u8>()
    }
}

struct LoadInt;

impl Load for LoadInt {
    fn map_channels(channels: i32) -> u32 {
        match channels {
            1 => gl::GL_RED,
            2 => gl::GL_RG,
            3 => gl::GL_RGB,
            4 => gl::GL_RGBA,
            _ => gl::GL_RED,
        }
    }

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
