use std::{
    ops::{Deref, DerefMut},
    slice,
};

use crate::bindings::stbi_image_free;

/// A buffer that holds image data loaded by stb_image.
/// Safe as long as the pointer is obtained from stb_image and not freed manually.
/// Constructing from a pointer managed by Rust's allocator is undefined behavior.
#[derive(Debug, PartialEq)]
pub struct StbiPtr<T> {
    raw: *mut T,
    len: usize,
}

impl<T> Drop for StbiPtr<T> {
    fn drop(&mut self) {
        //SAFETY: Safe as long as 'raw' was allocated by stb_image
        unsafe { stbi_image_free(self.raw as _) };
    }
}

impl<T> StbiPtr<T> {
    #[inline]
    pub const unsafe fn from_raw_parts(raw: *mut T, len: usize) -> Self {
        //SAFETY: Caller must ensure that the pointer is valid and was allocated by stb_image, len must be smaller than or equal to the number of elements in raw
        Self { raw, len }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        //SAFETY: safe as long as invariant 'from_raw_parts' is followed
        unsafe { slice::from_raw_parts(self.raw, self.len) }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        //SAFETY: safe as long as invariant 'from_raw_parts' is followed
        unsafe { slice::from_raw_parts_mut(self.raw, self.len) }
    }

    #[inline]
    pub fn to_vec(self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_slice().to_vec()
    }

    #[inline]
    pub fn into_boxed_slice(self) -> Box<[T]>
    where
        T: Clone,
    {
        self.to_vec().into_boxed_slice()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> AsRef<[T]> for StbiPtr<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for StbiPtr<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Deref for StbiPtr<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for StbiPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: Clone> From<StbiPtr<T>> for Vec<T> {
    #[inline]
    fn from(value: StbiPtr<T>) -> Self {
        value.to_vec()
    }
}

impl<T: Clone> From<StbiPtr<T>> for Box<[T]> {
    #[inline]
    fn from(value: StbiPtr<T>) -> Self {
        value.into_boxed_slice()
    }
}

impl<T: Clone> IntoIterator for StbiPtr<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}
