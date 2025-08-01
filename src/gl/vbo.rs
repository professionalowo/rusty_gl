use core::str;

use crate::gl::GL_ARRAY_BUFFER;

unsafe extern "C" {
    unsafe fn glGenBuffers(n: u32, buffers: *mut u32);
    unsafe fn glBindBuffer(n: u32, buffer: u32);
    unsafe fn glDeleteBuffers(n: u32, buffers: *const u32);
    unsafe fn glEnableVertexAttribArray(index: u32);
    unsafe fn glVertexAttribPointer(n: u32, size: u32, type_: u32, normalized: u8, stride: u32, pointer: *const std::ffi::c_void);
    unsafe fn glBufferData(n: u32, size: isize, data: *const std::ffi::c_void, usage: u32);
}
pub fn gen_buffers() -> u32 {
    let mut buffer = 0;
    unsafe {
        glGenBuffers(1, &mut buffer);
    }
    buffer
}

pub fn bind_buffer(n:u32, buffer: u32) {
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

pub fn vertex_attrib_pointer(
    index: u32,
    size: u32,
    type_: u32,
    normalized: u8,
    stride: u32,
    pointer: *const std::ffi::c_void,
) {
    unsafe {
        glVertexAttribPointer(index, size, type_, normalized, stride, pointer);
    }
}

pub fn buffer_data(n: u32, size: isize, data: *const std::ffi::c_void, usage: u32) {
    unsafe {
        glBufferData(n, size, data, usage);
    }
}