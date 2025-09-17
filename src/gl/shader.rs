use std::{ffi, fmt, fs, io, path::Path, ptr};

use crate::gl;

#[derive(Debug)]
pub struct Shader(u32);

#[derive(Debug)]
pub enum ShaderError {
    FileSystemError(io::Error),
    FFIError(ffi::NulError),
    CompilationError(String),
}

#[derive(Debug)]
pub enum ShaderType {
    Fragment,
    Vertex,
}

impl ShaderType {
    pub const fn key(&self) -> gl::GLenum {
        match self {
            Self::Fragment => gl::GL_FRAGMENT_SHADER,
            Self::Vertex => gl::GL_VERTEX_SHADER,
        }
    }
}

impl From<io::Error> for ShaderError {
    fn from(value: io::Error) -> Self {
        Self::FileSystemError(value)
    }
}

impl From<ffi::NulError> for ShaderError {
    fn from(value: ffi::NulError) -> Self {
        Self::FFIError(value)
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileSystemError(e) => fmt::Display::fmt(e, f),
            Self::FFIError(e) => fmt::Display::fmt(e, f),
            Self::CompilationError(m) => write!(f, "{}", m),
        }
    }
}

impl Shader {
    pub fn try_from_path<P>(shader_type: ShaderType, path: P) -> Result<Self, ShaderError>
    where
        P: AsRef<Path>,
    {
        Self::try_from_bytes(shader_type, fs::read(path)?)
    }

    pub fn try_from_bytes<B>(shader_type: ShaderType, source: B) -> Result<Self, ShaderError>
    where
        Vec<u8>: From<B>,
    {
        let shader = unsafe { gl::glCreateShader(shader_type.key()) };
        let c_str = ffi::CString::new(source)?;
        let c_str_ptr = c_str.as_ptr();
        unsafe {
            gl::glShaderSource(shader, 1, &c_str_ptr, ptr::null());
            gl::glCompileShader(shader);
        }

        if get_shader_iv(shader, gl::GL_COMPILE_STATUS) == 0 {
            return Err(ShaderError::CompilationError(get_info_log(shader)));
        }

        Ok(Shader(shader))
    }

    pub const fn id(&self) -> u32 {
        self.0
    }
}

fn get_info_log(shader: gl::GLuint) -> String {
    let log_length = get_shader_iv(shader, gl::GL_INFO_LOG_LENGTH);
    let mut info_log = Vec::with_capacity(log_length as usize);
    unsafe {
        gl::glGetShaderInfoLog(shader, log_length, ptr::null_mut(), info_log.as_mut_ptr());
        ffi::CStr::from_ptr(info_log.as_ptr())
    }
    .to_string_lossy()
    .into_owned()
}

pub fn get_shader_iv(shader: gl::GLuint, pname: gl::GLenum) -> i32 {
    let mut params = 0;
    unsafe {
        gl::glGetShaderiv(shader, pname, &mut params);
    }
    params
    }
    .to_string_lossy()
    .into_owned()
}

pub fn get_shader_iv(shader: gl::GLuint, pname: gl::GLenum) -> i32 {
    let mut params = 0;
    unsafe {
        gl::glGetShaderiv(shader, pname, &mut params);
    }
    params
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::glDeleteShader(self.0);
        }
    }
}
