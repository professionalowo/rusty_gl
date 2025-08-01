use std::path::PathBuf;
use crate::gl::*;

pub struct Shader {
    id: u32,
}

#[derive(Debug)]
pub enum ShaderError {
    FileSystemError(std::io::Error),
    FFIError(std::ffi::NulError),
    CompilationError(String),
}

impl Shader {
    pub fn try_from_path(shader_type: u32, path: PathBuf) -> Result<Self, ShaderError> {
        let source = std::fs::read_to_string(path).map_err(ShaderError::FileSystemError)?;
        Self::try_from_str(shader_type, &source)
    }

    pub fn try_from_str(shader_type: u32, source: impl AsRef<str>) -> Result<Self, ShaderError> {
        let shader_id = unsafe { glCreateShader(shader_type) };
        let c_str = std::ffi::CString::new(source.as_ref()).map_err(ShaderError::FFIError)?;
        let c_str_ptr = c_str.as_ptr();
        unsafe {
            glShaderSource(shader_id, 1, &c_str_ptr, std::ptr::null());
            glCompileShader(shader_id);
        }

        let mut status = 0;
        unsafe {
            glGetShaderiv(shader_id, GL_COMPILE_STATUS, &mut status);
        }
        if status == 0 {
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
            let error_message = info_log
                .iter()
                .take_while(|&&c| c != 0)
                .cloned()
                .collect::<Vec<u8>>();
            return Err(ShaderError::CompilationError(
                String::from_utf8_lossy(&error_message).to_string(),
            ));
        }

        Ok(Shader { id: shader_id })
    }

    pub const fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.id);
        }
    }
}
