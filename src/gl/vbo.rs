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
    size: i32,
    type_: u32,
    normalized: u8,
    pointer: *const std::ffi::c_void,
) {
    unsafe {
        glVertexAttribPointer(
            index,
            size,
            type_,
            normalized,
            size * std::mem::size_of::<T>() as i32,
            pointer,
        );
    }
}

pub fn buffer_data<T>(n: u32, data: &[T], usage: u32) {
    unsafe {
        glBufferData(
            n,
            (data.len() * std::mem::size_of::<T>()) as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }
}
