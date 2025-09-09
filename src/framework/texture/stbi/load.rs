use self::format::{Channels, Format};
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
        ref channels,
        data,
    } = load::<L>(bytes)?;
    let Format {
        format,
        internal_format,
    } = Format::try_from_load::<L>(channels)?;
    Ok(GlImageData {
        width,
        height,
        format,
        data: Box::from(data),
        type_: L::TYPE.data(),
        internal_format,
    })
}

#[derive(Debug)]
pub struct LoadData<'a> {
    width: i32,
    height: i32,
    channels: Channels,
    data: &'a [u8],
}

fn load<L>(bytes: &[u8]) -> ImageResult<LoadData<'_>>
where
    L: Load,
{
    let mut width = 0;
    let mut height = 0;
    let mut channels = Channels(0);
    let data = unsafe {
        let ptr = L::load(bytes, &mut width, &mut height, &mut channels.0);
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                failure_reason().unwrap_or_else(|| String::from("Unknown error")),
            ));
        }
        slice::from_raw_parts(ptr, (width * height * channels.0).try_into()?)
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
