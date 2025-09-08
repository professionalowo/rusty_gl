use std::slice;

use super::{load_trait::*, *};

pub(super) fn is_hdr(bytes: &[u8]) -> bool {
    unsafe { stbi_is_hdr_from_memory(bytes.as_ptr(), bytes.len() as i32) != 0 }
}

pub(super) fn try_loadf(bytes: &[u8]) -> ImageResult<GlImageData> {
    try_load_opt::<LoadFloat>(bytes)
}

pub(super) fn try_load(bytes: &[u8]) -> ImageResult<GlImageData> {
    try_load_opt::<LoadInt>(bytes)
}

fn try_load_opt<L>(bytes: &[u8]) -> ImageResult<GlImageData>
where
    L: Load,
{
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let LoadData {
        width,
        height,
        channels,
        data,
    } = load::<L>(bytes)?;
    let internal_format = i32::try_from(L::map_channels(channels))?;
    let format = format_from_channels(channels);
    Ok(GlImageData {
        width,
        height,
        format,
        data: Box::from(data),
        type_: L::TYPE.data(),
        internal_format,
    })
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

#[derive(Debug)]
struct LoadData<'a> {
    width: i32,
    height: i32,
    channels: i32,
    data: &'a [u8],
}

fn load<'a, L>(bytes: &'a [u8]) -> ImageResult<LoadData<'a>>
where
    L: Load,
{
    let mut width = 0;
    let mut height = 0;
    let mut channels = 0;
    let data = unsafe {
        let ptr = L::load(bytes, &mut width, &mut height, &mut channels);
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                failure_reason().unwrap_or_else(|| String::from("Unknown error")),
            ));
        }
        slice::from_raw_parts(ptr, (width * height * channels).try_into()?)
    };
    Ok(LoadData {
        width,
        height,
        channels,
        data,
    })
}

pub fn failure_reason() -> Option<String> {
    let ptr = unsafe { stbi_failure_reason() };

    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}
