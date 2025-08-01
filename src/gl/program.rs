use crate::gl::{
    GL_INFO_LOG_LENGTH, GL_LINK_STATUS, glAttachShader, glCreateProgram, glDeleteProgram,
    glGetProgramInfoLog, glGetProgramiv, glLinkProgram, glUseProgram, shader::Shader,
};

pub struct Program {
    id: u32,
    _shaders: Vec<Shader>,
}

impl Program {
    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Self, String> {
        let id = unsafe { glCreateProgram() };

        for shader in &shaders {
            unsafe { glAttachShader(id, shader.id()) };
        }

        unsafe { glLinkProgram(id) };

        let mut link_status = 0;
        unsafe {
            glGetProgramiv(id, GL_LINK_STATUS, &mut link_status);
        }

        if link_status == 0 {
            let mut log_length = 0;
            unsafe {
                glGetProgramiv(id, GL_INFO_LOG_LENGTH, &mut log_length);
            }
            let mut info_log = vec![0u8; log_length as usize];
            unsafe {
                glGetProgramInfoLog(
                    id,
                    log_length,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
            }

            let error_msg = String::from_utf8_lossy(&info_log).to_string();
            unsafe {
                glDeleteProgram(id);
            }
            return Err(error_msg);
        }
        Ok(Program {
            id,
            _shaders: shaders,
        })
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glUseProgram(0);
        }
    }

    pub const fn id(&self) -> u32 {
        self.id
    }
}
