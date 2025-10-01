use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo},
    ptr::NonNull,
    slice,
};

use crate::bindings::stbi_image_free;

pub mod iter;

/// A buffer that holds image data loaded by stb_image.
/// Safe as long as the pointer is obtained from stb_image and not freed manually.
/// Constructing from a pointer managed by Rust's allocator is undefined behavior.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct StbiPtr<T>(NonNull<[T]>);

impl<T> Drop for StbiPtr<T> {
    fn drop(&mut self) {
        //SAFETY: Safe as long as 'raw' was allocated by stb_image
        unsafe { stbi_image_free(self.as_mut_ptr() as _) };
    }
}

impl<T> StbiPtr<T> {
    pub const fn from_raw_parts(data: *mut T, len: usize) -> Option<Self> {
        match NonNull::new(data) {
            None => None,
            //SAFETY: data is not null
            Some(data) => Some(Self(NonNull::slice_from_raw_parts(data, len))),
        }
    }

    pub const unsafe fn from_raw_parts_unchecked(data: *mut T, len: usize) -> Self {
        unsafe {
            let ptr = slice::from_raw_parts_mut(data, len);
            Self(NonNull::new_unchecked(ptr))
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { self.0.as_ref() }
    }

    #[inline]
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { self.0.as_mut() }
    }

    #[inline]
    pub fn to_vec(self) -> Vec<T>
    where
        T: Clone,
    {
        (*self).to_vec()
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
        self.0.len()
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

impl<T> AsRef<[T]> for StbiPtr<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &(**self)
    }
}

impl<T> AsMut<[T]> for StbiPtr<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut (**self)
    }
}

impl<T> Borrow<[T]> for StbiPtr<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &(**self)
    }
}

impl<T> BorrowMut<[T]> for StbiPtr<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut (**self)
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

macro_rules! index_impl {
    (for<$($gen:tt),*> $base:ty [$idx:ty] => $output:ty) => {
        impl<$($gen),*> Index<$idx> for $base {
            type Output = $output;
            #[inline]
            fn index(&self, index: $idx) -> &Self::Output {
                &(**self)[index]
            }
        }

        impl<$($gen),*> IndexMut<$idx> for $base {
            #[inline]
            fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
                &mut (**self)[index]
            }
        }
    };
}

index_impl! {for<T> StbiPtr<T>[usize] => T}
index_impl! {for<T> StbiPtr<T>[Range<usize>] => [T]}
index_impl! {for<T> StbiPtr<T>[RangeFrom<usize>] => [T]}
index_impl! {for<T> StbiPtr<T>[RangeTo<usize>] => [T]}
index_impl! {for<T> StbiPtr<T>[RangeInclusive<usize>] => [T]}
index_impl! {for<T> StbiPtr<T>[RangeFull] => [T]}

impl<'a, T> IntoIterator for &'a StbiPtr<T> {
    type Item = &'a T;
    type IntoIter = iter::IntoIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self)
    }
}
