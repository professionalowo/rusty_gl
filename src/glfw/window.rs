use std::ffi::{CString, NulError, c_int};

use crate::gl;
use crate::glfw::GLFWError;
pub struct Window {
    handle: *mut gl::GLFWwindow,
}

impl Window {
    pub fn try_new<S>(width: u32, height: u32, title: S) -> Result<Self, NulError>
    where
        S: AsRef<str>,
    {
        let handle = create_window(width, height, title.as_ref(), None, None)?;
        Ok(Window { handle })
    }

    pub fn should_close(&self) -> Result<bool, GLFWError> {
        unsafe {
            let result = gl::glfwWindowShouldClose(self.handle);
            if result == 0 {
                Ok(false)
            } else if result == 1 {
                Ok(true)
            } else {
                Err(GLFWError::from(result))
            }
        }
    }

    pub fn poll_events(&self) {
        unsafe {
            gl::glfwPollEvents();
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            gl::glfwSwapBuffers(self.handle);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            gl::glfwDestroyWindow(self.handle);
        }
    }
}

fn create_window(
    width: u32,
    height: u32,
    title: &str,
    monitor: Option<*mut gl::GLFWmonitor>,
    share: Option<*mut gl::GLFWwindow>,
) -> Result<*mut gl::GLFWwindow, NulError> {
    unsafe {
        let title_cstr = CString::new(title)?;
        let window = gl::glfwCreateWindow(
            width as c_int,
            height as c_int,
            title_cstr.as_ptr(),
            monitor.unwrap_or(std::ptr::null_mut()),
            share.unwrap_or(std::ptr::null_mut()),
        );
        gl::glfwMakeContextCurrent(window);
        Ok(window)
    }
}

fn get_primary_monitor() -> Option<*mut gl::GLFWmonitor> {
    unsafe {
        let monitor = gl::glfwGetPrimaryMonitor();
        if monitor.is_null() {
            None
        } else {
            Some(monitor)
        }
    }
}
