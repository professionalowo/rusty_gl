use std::ffi::{NulError, c_int};

use crate::glfw::GlfwError;

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

    pub fn should_close(&self) -> Result<bool, GlfwError> {
        unsafe {
            let result = glfwWindowShouldClose(self.handle);
            if result == 0 {
                Ok(false)
            } else if result == 1 {
                Ok(true)
            } else {
                Err(GlfwError(result))
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
#[repr(C)]
pub struct GLFWmonitor(*mut std::ffi::c_void);

fn create_window(
    width: u32,
    height: u32,
    title: &str,
    monitor: Option<GLFWmonitor>,
    share: Option<GLFWwindow>,
) -> Result<GLFWwindow, NulError> {
    unsafe {
        let title_cstr = std::ffi::CString::new(title)?;
        let window = glfwCreateWindow(
            width as c_int,
            height as c_int,
            title_cstr.as_ptr(),
            monitor.unwrap_or(GLFWmonitor(std::ptr::null_mut())),
            share.unwrap_or(GLFWwindow(std::ptr::null_mut())),
        );
        Ok(window)
    }
}

fn get_primary_monitor() -> Result<GLFWmonitor, GlfwError> {
    unsafe {
        let monitor = glfwGetPrimaryMonitor();
        if monitor.0.is_null() {
            Err(GlfwError(-1)) // Replace with appropriate error code
        } else {
            Ok(monitor)
        }
    }
}
