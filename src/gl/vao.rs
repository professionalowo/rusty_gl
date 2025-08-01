use crate::gl::{glBindVertexArray, glDeleteVertexArrays, glGenVertexArrays};

pub fn gen_vertex_arrays() -> u32 {
    let mut vao = 0;
    unsafe {
        glGenVertexArrays(1, &mut vao);
    }
    vao
}

pub fn bind_vertex_array(vao: u32) {
    unsafe {
        glBindVertexArray(vao);
    }
}

pub fn delete_vertex_array(vao: u32) {
    unsafe {
        glDeleteVertexArrays(1, &vao);
    }
}
