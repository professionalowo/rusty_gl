use std::ffi::{NulError, c_int, CString};

use crate::glfw::GLFWError;

unsafe extern "C" {
    unsafe fn glfwGetPrimaryMonitor() -> GLFWmonitor;
    unsafe fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const i8,
        monitor: GLFWmonitor,
        share: GLFWwindow,
    ) -> GLFWwindow;
    unsafe fn glfwDestroyWindow(handle: GLFWwindow);
    unsafe fn glfwWindowShouldClose(handle: GLFWwindow) -> c_int;
    unsafe fn glfwPollEvents();
    unsafe fn glfwMakeContextCurrent(handle: GLFWwindow);
    unsafe fn glfwSwapBuffers(handle: GLFWwindow);
}

pub struct Window {
    handle: GLFWwindow,
}

impl Window {
    pub fn try_new(width: u32, height: u32, title: &str) -> Result<Self, NulError> {
        let handle = create_window(width, height, title, None, None)?;
        Ok(Window { handle })
    }

    pub fn should_close(&self) -> Result<bool, GLFWError> {
        unsafe {
            let result = glfwWindowShouldClose(self.handle);
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
            glfwPollEvents();
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glfwSwapBuffers(self.handle);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfwDestroyWindow(self.handle);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct GLFWwindow(*mut std::ffi::c_void);

impl GLFWwindow {
    const fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

#[repr(C)]
pub struct GLFWmonitor(*mut std::ffi::c_void);

impl GLFWmonitor {
    const fn is_null(&self) -> bool {
        self.0.is_null()
    }   
}

fn create_window(
    width: u32,
    height: u32,
    title: &str,
    monitor: Option<GLFWmonitor>,
    share: Option<GLFWwindow>,
) -> Result<GLFWwindow, NulError> {
    unsafe {
        let title_cstr = CString::new(title)?;
        let window = glfwCreateWindow(
            width as c_int,
            height as c_int,
            title_cstr.as_ptr(),
            monitor.unwrap_or(GLFWmonitor(std::ptr::null_mut())),
            share.unwrap_or(GLFWwindow(std::ptr::null_mut())),
        );
        glfwMakeContextCurrent(window);
        Ok(window)
    }
}

fn get_primary_monitor() -> Option<GLFWmonitor> {
    unsafe {
        let monitor = glfwGetPrimaryMonitor();
        if monitor.is_null() {
            None
        } else {
            Some(monitor)
        }
    }
}
