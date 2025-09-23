use std::ptr::NonNull;

pub mod bindings;
pub mod channels;
pub mod dimensions;
pub mod load;
pub mod stbi_ptr;

pub fn is_hdr<B>(bytes: B) -> bool
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    unsafe { bindings::stbi_is_hdr_from_memory(bytes.as_ptr(), bytes.len() as _) != 0 }
}

pub fn failure_reason() -> Option<String> {
    let ptr = unsafe { bindings::stbi_failure_reason() }.cast_mut();

    match NonNull::new(ptr) {
        None => None,
        Some(n) => Some(
            unsafe { std::ffi::CStr::from_ptr(n.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
        ),
    }
}
