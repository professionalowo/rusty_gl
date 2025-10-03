#[macro_export]
macro_rules! wrapper_impl {
    (struct $name:ident($inner:ty)) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct $name(::std::ptr::NonNull<$inner>);

        impl $name {
            #[inline]
            pub const fn as_ref(&self) -> &$inner {
                unsafe { self.0.as_ref() }
            }

            #[inline]
            pub const fn as_mut(&mut self) -> &mut $inner {
                unsafe { self.0.as_mut() }
            }

            #[inline]
            pub const fn new(ptr: *mut $inner) -> Option<Self> {
                match ::std::ptr::NonNull::new(ptr) {
                    None => None,
                    Some(nn) => Some(Self(nn)),
                }
            }

            const fn as_ptr(&self) -> *mut $inner {
                self.0.as_ptr()
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $inner;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }

        impl ::std::convert::AsRef<$inner> for $name {
            #[inline]
            fn as_ref(&self) -> &$inner {
                &(**self)
            }
        }

        impl ::std::convert::AsMut<$inner> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut $inner {
                &mut (**self)
            }
        }

        impl ::std::borrow::Borrow<$inner> for $name {
            #[inline]
            fn borrow(&self) -> &$inner {
                &(**self)
            }
        }

        impl ::std::borrow::BorrowMut<$inner> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $inner {
                &mut (**self)
            }
        }
    };
}
