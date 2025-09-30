use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut, Index, IndexMut, Range},
    ptr::NonNull,
    slice,
};

use crate::bindings::stbi_image_free;

pub mod iter;

/// A buffer that holds image data loaded by stb_image.
/// Safe as long as the pointer is obtained from stb_image and not freed manually.
/// Constructing from a pointer managed by Rust's allocator is undefined behavior.
#[derive(Debug, PartialEq)]
pub struct StbiPtr<T> {
    inner: NonNull<T>,
    len: usize,
}

impl<T> Drop for StbiPtr<T> {
    fn drop(&mut self) {
        //SAFETY: Safe as long as 'raw' was allocated by stb_image
        unsafe { stbi_image_free(self.inner.as_ptr() as _) };
    }
}

impl<T> StbiPtr<T> {
    #[inline]
    pub const unsafe fn from_raw_parts_unchecked(ptr: *mut T, len: usize) -> Self {
        //SAFETY: Caller must ensure that the pointer is valid and was allocated by stb_image, len must be smaller than or equal to the number of elements in raw
        Self {
            inner: unsafe { NonNull::new_unchecked(ptr) },
            len,
        }
    }

    #[inline]
    pub const fn from_raw_parts(ptr: *mut T, len: usize) -> Option<Self> {
        match NonNull::new(ptr) {
            None => None,
            Some(inner) => Some(Self { inner, len }),
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.inner.as_ptr(), self.len) }
    }

    #[inline]
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.inner.as_ptr(), self.len) }
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
        self.as_slice_mut()
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
        self.as_slice_mut()
    }
}

impl<T> Borrow<[T]> for StbiPtr<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> BorrowMut<[T]> for StbiPtr<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
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

impl<T> Index<usize> for StbiPtr<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for StbiPtr<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl<T> Index<Range<usize>> for StbiPtr<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<Range<usize>> for StbiPtr<T> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl<'a, T> IntoIterator for &'a StbiPtr<T> {
    type Item = &'a T;
    type IntoIter = iter::IntoIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self)
    }
}
