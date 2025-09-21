use super::{
    GlImageData, ImageError, ImageResult, dimensions::Dimensions, format::Format, load_trait::*,
};
use stbi_sys::{bindings::*, channels::Channels};
use std::slice;

pub(super) fn try_loadf(bytes: &[u8]) -> ImageResult<GlImageData> {
    try_load_opt::<LoadFloat>(bytes)
}

pub(super) fn try_load(bytes: &[u8]) -> ImageResult<GlImageData> {
    try_load_opt::<LoadInt>(bytes)
}

fn try_load_opt<L>(bytes: &[u8]) -> ImageResult<GlImageData>
where
    L: Load + MapChannels,
{
    unsafe {
        stbi_set_flip_vertically_on_load(1);
    }
    let LoadData {
        dimensions: Dimensions { width, height },
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
    dimensions: Dimensions,
    channels: Channels,
    data: &'a [u8],
}

fn load<L>(bytes: &[u8]) -> ImageResult<LoadData<'_>>
where
    L: Load,
{
    let mut dimensions = Dimensions::default();
    let mut channels = Channels::default();
    let data = unsafe {
        let ptr = L::load(bytes, &mut dimensions, &mut channels);
        if ptr.is_null() {
            return Err(ImageError::StbiError(
                stbi_sys::failure_reason().unwrap_or_else(|| String::from("Unknown error")),
            ));
        }
        slice::from_raw_parts(ptr, dimensions.volume_with_channels(&channels).try_into()?)
    };
    Ok(LoadData {
        dimensions,
        channels,
        data,
    })
}
