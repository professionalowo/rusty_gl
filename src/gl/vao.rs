use crate::gl::{glBindVertexArray, glDeleteVertexArrays, glGenVertexArrays};

#[derive(Debug, Clone)]
pub struct VertexArrayObject(pub u32);

impl VertexArrayObject {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn gen_vertex_arrays() -> Self {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
        }
        Self(vao)
    }

    pub fn bind_vertex_array(vao: Self) {
        unsafe {
            glBindVertexArray(vao.0);
        }
    }

    pub fn delete_vertex_array(vao: Self) {
        unsafe {
            glDeleteVertexArrays(1, &vao.0);
        }
    }
}
