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

#[cold]
pub fn failure_reason() -> Option<String> {
    let ptr = unsafe { bindings::stbi_failure_reason() };

    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}
