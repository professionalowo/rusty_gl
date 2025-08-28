use crate::gl::{self, glBindVertexArray, glDeleteVertexArrays, glGenVertexArrays};

#[derive(Debug, Clone, Default)]
pub struct VertexArrayObject(pub gl::GLuint);

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

    pub fn bind_vertex_array(vao: &Self) {
        let Self(index) = vao;
        unsafe {
            glBindVertexArray(*index);
        }
    }

    pub fn delete_vertex_array(vao: &Self) {
        let Self(index) = vao;
        unsafe {
            glDeleteVertexArrays(1, index);
        }
    }
}
