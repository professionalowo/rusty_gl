use std::ffi::{CStr, CString, NulError, c_int};

use crate::gl;
use crate::glfw::GLFWError;
pub struct Window {
    handle: *mut gl::GLFWwindow,
    _title: CString,
}

impl Window {
    pub fn try_new<S>(width: u32, height: u32, title: S) -> Result<Self, NulError>
    where
        S: AsRef<str>,
    {
        let title_cstr = CString::new(title.as_ref())?;
        let handle = create_window(width, height, &title_cstr, None, None);
        Ok(Window {
            handle,
            _title: title_cstr,
        })
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

    pub fn aspect_ratio(&self) -> f32 {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            gl::glfwGetWindowSize(self.handle, &mut width, &mut height);
        }
        if height == 0 {
            1.0 // Avoid division by zero
        } else {
            width as f32 / height as f32
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
    title: &CStr,
    monitor: Option<*mut gl::GLFWmonitor>,
    share: Option<*mut gl::GLFWwindow>,
) -> *mut gl::GLFWwindow {
    unsafe {
        let window = gl::glfwCreateWindow(
            width as c_int,
            height as c_int,
            title.as_ptr(),
            monitor.unwrap_or(std::ptr::null_mut()),
            share.unwrap_or(std::ptr::null_mut()),
        );
        gl::glfwMakeContextCurrent(window);
        window
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
