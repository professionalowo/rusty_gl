use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::bindings::ImGuiIO;

#[derive(Debug)]
#[repr(transparent)]
pub struct IO(NonNull<ImGuiIO>);

impl IO {
    #[inline]
    pub const fn new(ptr: *mut ImGuiIO) -> Option<Self> {
        match NonNull::new(ptr) {
            None => None,
            Some(nn) => Some(Self(nn)),
        }
    }
}

impl Deref for IO {
    type Target = ImGuiIO;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref() }
    }
}

impl DerefMut for IO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
