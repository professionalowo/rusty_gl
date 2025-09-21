pub mod bindings;

pub fn is_hdr<B>(bytes: B) -> bool
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    unsafe { bindings::stbi_is_hdr_from_memory(bytes.as_ptr(), bytes.len() as i32) != 0 }
}

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
