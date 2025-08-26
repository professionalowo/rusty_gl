use crate::gl::*;
use std::path::PathBuf;

pub struct Shader {
    id: u32,
}

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
    pub fn try_from_path(shader_type: u32, path: PathBuf) -> Result<Self, ShaderError> {
        let source = std::fs::read_to_string(path)?;
        Self::try_from_str(shader_type, &source)
    }

    pub fn try_from_str(shader_type: u32, source: impl AsRef<str>) -> Result<Self, ShaderError> {
        let id = unsafe { glCreateShader(shader_type) };
        let c_str = std::ffi::CString::new(source.as_ref())?;
        let c_str_ptr = c_str.as_ptr();
        unsafe {
            glShaderSource(id, 1, &c_str_ptr, std::ptr::null());
            glCompileShader(id);
        }

        let mut status = 0;
        unsafe {
            glGetShaderiv(id, GL_COMPILE_STATUS, &mut status);
        }
        if status == 0 {
            return Err(ShaderError::CompilationError(get_info_log(id)));
        }

        Ok(Shader { id })
    }

    pub const fn id(&self) -> u32 {
        self.id
    }
}

fn get_info_log(shader_id: u32) -> String {
    let mut log_length = 0;
    unsafe {
        glGetShaderiv(shader_id, GL_INFO_LOG_LENGTH, &mut log_length);
    }
    let mut info_log: Vec<u8> = vec![0; log_length as usize];
    unsafe {
        glGetShaderInfoLog(
            shader_id,
            log_length,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut i8,
        );
    }

    unsafe {
        std::ffi::CStr::from_ptr(info_log.as_ptr() as *const i8)
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.id);
        }
    }
}
