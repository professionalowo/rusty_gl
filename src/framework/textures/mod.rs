use crate::gl::{GLenum, GLint, GLuint};

pub struct Texture2D {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
    pub internal_format: GLint,
    pub format: GLenum,
    pub type_: GLenum,
}
