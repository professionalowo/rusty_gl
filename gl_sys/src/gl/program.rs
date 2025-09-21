use crate::gl::{
    GL_INFO_LOG_LENGTH, GL_LINK_STATUS, GL_VALIDATE_STATUS, glAttachShader, glCreateProgram,
    glDeleteProgram, glGetProgramInfoLog, glGetProgramiv, glLinkProgram, glUseProgram,
    glValidateProgram,
    shader::Shader,
    uniform::{UniformLocation, UniformLocationError, uniform_trait::Uniform},
};

pub struct Program(pub u32);

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { glCreateProgram() };

        for shader in shaders {
            unsafe { glAttachShader(id, shader.id()) };
        }

        let mut link_status = 0;
        unsafe {
            glLinkProgram(id);
            glGetProgramiv(id, GL_LINK_STATUS, &mut link_status);
        }

        let mut validate_status = 0;
        unsafe {
            glValidateProgram(id);
            glGetProgramiv(id, GL_VALIDATE_STATUS, &mut validate_status);
        };

        if link_status == 0 || validate_status == 0 {
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
        Ok(Program(id))
    }

    pub fn uniform<U: Uniform>(
        &self,
        name: impl AsRef<str>,
        value: U,
    ) -> Result<(), UniformLocationError> {
        let name = name.as_ref();
        let loc = UniformLocation::try_for_program(self, name)?;
        UniformLocation::provide(&loc, value);
        Ok(())
    }
    pub fn uniform_opt<U: Uniform>(
        &self,
        name: impl AsRef<str>,
        value: U,
        opt: U::Options,
    ) -> Result<(), UniformLocationError> {
        let name = name.as_ref();
        let loc = UniformLocation::try_for_program(self, name)?;
        UniformLocation::provide_opt(&loc, value, opt);
        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.id());
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glUseProgram(0);
        }
    }

    pub const fn id(&self) -> u32 {
        self.0
    }
}
