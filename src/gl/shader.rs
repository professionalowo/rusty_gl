use crate::gl;
use std::{fmt, path::Path};

pub struct Shader(u32);

#[derive(Debug)]
pub enum ShaderError {
    FileSystemError(std::io::Error),
    FFIError(std::ffi::NulError),
    CompilationError(String),
}

impl From<std::io::Error> for ShaderError {
    fn from(value: std::io::Error) -> Self {
        Self::FileSystemError(value)
    }
}

impl From<std::ffi::NulError> for ShaderError {
    fn from(value: std::ffi::NulError) -> Self {
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
    pub fn try_from_path<P>(shader_type: u32, path: P) -> Result<Self, ShaderError>
    where
        P: AsRef<Path>,
    {
        let source = std::fs::read_to_string(path)?;
        Self::try_from_str(shader_type, &source)
    }

    pub fn try_from_str<R>(shader_type: u32, source: R) -> Result<Self, ShaderError>
    where
        R: AsRef<str>,
    {
        let id = unsafe { gl::glCreateShader(shader_type) };
        let c_str = std::ffi::CString::new(source.as_ref())?;
        let c_str_ptr = c_str.as_ptr();
        unsafe {
            gl::glShaderSource(id, 1, &c_str_ptr, std::ptr::null());
            gl::glCompileShader(id);
        }

        let mut status = 0;
        unsafe {
            gl::glGetShaderiv(id, gl::GL_COMPILE_STATUS, &mut status);
        }
        if status == 0 {
            return Err(ShaderError::CompilationError(get_info_log(id)));
        }

        Ok(Shader(id))
    }

    pub const fn id(&self) -> u32 {
        self.0
    }
}

fn get_info_log(id: u32) -> String {
    let mut log_length = 0;
    unsafe {
        gl::glGetShaderiv(id, gl::GL_INFO_LOG_LENGTH, &mut log_length);
    }
    let mut info_log: Vec<i8> = Vec::with_capacity(log_length as usize);
    unsafe {
        gl::glGetShaderInfoLog(id, log_length, std::ptr::null_mut(), info_log.as_mut_ptr());
    }

    unsafe {
        std::ffi::CStr::from_ptr(info_log.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::glDeleteShader(self.0);
        }
    }
}
