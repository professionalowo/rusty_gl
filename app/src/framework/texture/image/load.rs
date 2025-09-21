use super::{GlImageData, ImageResult, format::Format, map_channels::*};
use stbi_sys::{
    bindings::*,
    dimensions::Dimensions,
    load::{Load, LoadData, LoadFloat, LoadInt},
};

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
    } = LoadData::load::<L>(bytes)?;
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
