use std::{ffi::c_int, fmt, num};

use crate::{
    bindings::{
        stbi_load_from_memory, stbi_loadf_from_memory, stbi_set_flip_vertically_on_load, stbi_uc,
    },
    channels::Channels,
    dimensions::Dimensions,
    failure_reason,
    stbi_ptr::StbiPtr,
};

#[derive(Debug)]
pub struct LoadData {
    pub dimensions: Dimensions,
    pub channels: Channels,
    pub data: StbiPtr<u8>,
}

#[derive(Debug)]
pub enum LoadError {
    StbiError(String),
    TryFromIntError(num::TryFromIntError),
}

impl From<num::TryFromIntError> for LoadError {
    fn from(value: num::TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromIntError(err) => fmt::Display::fmt(err, f),
            Self::StbiError(err) => write!(f, "STBI Error: {}", err),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LoadOptions {
    pub flip_vertically_on_load: bool,
}

impl Default for LoadOptions {
    #[inline]
    fn default() -> Self {
        Self {
            flip_vertically_on_load: true,
        }
    }
}

impl LoadData {
    pub fn try_load<L, B>(bytes: B, options: LoadOptions) -> Result<Self, LoadError>
    where
        L: Load,
        B: AsRef<[u8]>,
    {
        if options.flip_vertically_on_load {
            unsafe {
                stbi_set_flip_vertically_on_load(1);
            }
        }

        let mut dimensions = Dimensions::default();
        let mut channels = Channels::default();

        let data = L::load(bytes, &mut dimensions, &mut channels).map_err(LoadError::StbiError)?;
        Ok(Self {
            dimensions,
            channels,
            data,
        })
    }
}

pub trait Load {
    fn load<B: AsRef<[u8]>>(
        bytes: B,
        Dimensions {
            width: x,
            height: y,
        }: &mut Dimensions,
        Channels(channels): &mut Channels,
    ) -> Result<StbiPtr<u8>, String> {
        let buffer = bytes.as_ref();
        let ptr = unsafe {
            Self::load_from_memory(buffer.as_ptr(), buffer.len() as _, x, y, channels, 0)
        };
        let len = (*x) * (*y) * (*channels);
        if ptr.is_null() {
            Err(failure_reason().unwrap_or_else(|| String::from("Unknown error")))
        } else {
            Ok(unsafe { StbiPtr::from_raw_parts_unchecked(ptr, len as _) })
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

pub struct LoadFloat;

impl Load for LoadFloat {
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

pub struct LoadInt;

impl Load for LoadInt {
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

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: [u8; 103] = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x66,
        0xBC, 0x3A, 0x25, 0x00, 0x00, 0x00, 0x03, 0x50, 0x4C, 0x54, 0x45, 0xB5, 0xD0, 0xD0, 0x63,
        0x04, 0x16, 0xEA, 0x00, 0x00, 0x00, 0x1F, 0x49, 0x44, 0x41, 0x54, 0x68, 0x81, 0xED, 0xC1,
        0x01, 0x0D, 0x00, 0x00, 0x00, 0xC2, 0xA0, 0xF7, 0x4F, 0x6D, 0x0E, 0x37, 0xA0, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBE, 0x0D, 0x21, 0x00, 0x00, 0x01, 0x9A, 0x60, 0xE1,
        0xD5, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    #[test]
    fn test_empty_data() {
        assert!(LoadData::try_load::<LoadInt, _>([], Default::default()).is_err())
    }

    #[test]
    fn test_minimal_data() {
        assert!(LoadData::try_load::<LoadInt, _>(MINIMAL, Default::default(),).is_ok())
    }
}
