use crate::gl::{
    self, GLboolean, GLint, GLsizeiptr, glBindBuffer, glBufferData, glDeleteBuffers,
    glEnableVertexAttribArray, glGenBuffers, glVertexAttribPointer,
};

#[derive(Debug, Clone)]
pub struct Location(pub gl::GLuint);

#[derive(Debug, Clone, Default)]
pub struct VertexBufferObject(pub gl::GLuint);

impl VertexBufferObject {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn gen_buffers() -> Self {
        let mut buffer = 0;
        unsafe {
            glGenBuffers(1, &mut buffer);
        }
        Self(buffer)
    }

    pub fn bind_buffer(n: u32, Self(buffer): Self) {
        unsafe {
            glBindBuffer(n, buffer);
        }
    }

    pub fn delete_buffer(Self(buffer): Self) {
        unsafe {
            glDeleteBuffers(1, &buffer);
        }
    }

    pub fn enable_vertex_attrib_array(Location(index): &Location) {
        unsafe {
            glEnableVertexAttribArray(*index);
        }
    }

    pub fn vertex_attrib_pointer<T>(
        location: &Location,
        size: impl TryInto<GLint>,
        type_: u32,
        normalized: impl TryInto<GLboolean>,
        pointer: Option<*const std::ffi::c_void>,
    ) -> Result<(), VBOError> {
        let size = match size.try_into() {
            Ok(s) => s,
            Err(_) => return Err(VBOError::CastError),
        };
        let stride = size * std::mem::size_of::<T>() as i32;
        let normalized: u8 = match normalized.try_into() {
            Ok(n) => n,
            Err(_) => return Err(VBOError::CastError),
        };
        let Location(index) = *location;
        let pointer = pointer.unwrap_or(std::ptr::null());
        unsafe {
            glVertexAttribPointer(index, size, type_, normalized, stride, pointer);
        }
        Ok(())
    }

    pub fn buffer_data<T>(n: u32, data: &[T], usage: u32) -> Result<(), VBOError> {
        let pointer = data.as_ptr() as *const std::ffi::c_void;
        let size = match GLsizeiptr::try_from(data.len() * std::mem::size_of::<T>()) {
            Ok(s) => s,
            Err(_) => return Err(VBOError::CastError),
        };
        unsafe {
            glBufferData(n, size, pointer, usage);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum VBOError {
    CastError,
}
