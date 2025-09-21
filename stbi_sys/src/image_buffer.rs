use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::bindings::stbi_image_free;

/// A buffer that holds image data loaded by stb_image.
/// Safe as long as the pointer is obtained from stb_image and not freed manually.
/// Constructing from a pointer managed by Rust's allocator is undefined behavior.
#[derive(Debug, PartialEq)]
pub struct StbImageBuffer {
    ptr: *mut u8,
    len: usize,
}

impl Drop for StbImageBuffer {
    fn drop(&mut self) {
        unsafe { stbi_image_free(self.ptr as _) };
    }
}

impl Default for StbImageBuffer {
    fn default() -> Self {
        Self::null()
    }
}

impl StbImageBuffer {
    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub const unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        //SAFETY: Caller must ensure that the pointer is valid and was allocated by stb_image
        Self { ptr, len }
    }

    pub const fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    pub const fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub const fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl AsRef<[u8]> for StbImageBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for StbImageBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl Deref for StbImageBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for StbImageBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
