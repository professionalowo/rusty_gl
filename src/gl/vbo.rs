use crate::gl::{
    glBindBuffer, glBufferData, glDeleteBuffers, glEnableVertexAttribArray, glGenBuffers,
    glVertexAttribPointer,
};

pub fn gen_buffers() -> u32 {
    let mut buffer = 0;
    unsafe {
        glGenBuffers(1, &mut buffer);
    }
    buffer
}

pub fn bind_buffer(n: u32, buffer: u32) {
    unsafe {
        glBindBuffer(n, buffer);
    }
}

pub fn delete_buffer(buffer: u32) {
    unsafe {
        glDeleteBuffers(1, &buffer);
    }
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        glEnableVertexAttribArray(index);
    }
}

pub fn vertex_attrib_pointer<T>(
    index: u32,
    size: impl TryInto<i32>,
    type_: u32,
    normalized: impl TryInto<u8>,
    pointer: Option<*const std::ffi::c_void>,
) {
    let size = size.try_into().unwrap_or(2);
    let stride = size * std::mem::size_of::<T>() as i32;
    let normalized: u8 = normalized.try_into().unwrap_or(0);

    let pointer = pointer.unwrap_or(std::ptr::null());
    unsafe {
        glVertexAttribPointer(index, size, type_, normalized, stride, pointer);
    }
}

pub fn buffer_data<T>(n: u32, data: &[T], usage: u32) {
    let pointer = data.as_ptr() as *const std::ffi::c_void;
    let size = (data.len() * std::mem::size_of::<T>()) as i64;
    unsafe {
        glBufferData(n, size, pointer, usage);
    }
}
