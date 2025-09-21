use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::bindings;

#[derive(Debug)]
pub struct ImageBuffer {
    ptr: *mut u8,
    len: usize,
}

impl ImageBuffer {
    pub unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        ImageBuffer { ptr, len }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn into_boxed_slice(self) -> Box<[u8]> {
        let slice = self.as_slice();
        slice.to_vec().into_boxed_slice()
    }
}

impl AsRef<[u8]> for ImageBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for ImageBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for ImageBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl Drop for ImageBuffer {
    fn drop(&mut self) {
        unsafe { bindings::stbi_image_free(self.ptr as _) };
    }
}
