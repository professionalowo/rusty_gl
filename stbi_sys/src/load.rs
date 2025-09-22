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
impl LoadData {
    pub fn try_load<L, B>(bytes: B) -> Result<Self, LoadError>
    where
        L: Load,
        B: AsRef<[u8]>,
    {
        unsafe {
            stbi_set_flip_vertically_on_load(1);
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
        unsafe {
            let ptr = Self::load_from_memory(buffer.as_ptr(), buffer.len() as _, x, y, channels, 0);
            if ptr.is_null() {
                Err(failure_reason().unwrap_or_else(|| String::from("Unknown error")))
            } else {
                Ok(StbiPtr::from_raw_parts(
                    ptr,
                    ((*x) * (*y) * (*channels)) as _,
                ))
            }
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
