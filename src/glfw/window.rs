use std::ffi::{CStr, CString, NulError, c_int, c_void};

use crate::gl;
use crate::glfw::GLFWError;
use crate::glfw::input::action::Action;
use crate::glfw::input::keycode::Keycode;
use crate::glfw::input::modifier::Modifier;
pub struct Window {
    handle: *mut gl::GLFWwindow,
    _title: CString,
}

type KeyCallback = dyn FnMut(Keycode, Action, Modifier);

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

    pub fn set_key_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Keycode, Action, Modifier) + 'static,
    {
        let closure: Box<KeyCallback> = Box::new(callback);
        let raw = Box::into_raw(Box::new(closure));

        unsafe {
            gl::glfwSetWindowUserPointer(self.handle, raw as *mut c_void);
            gl::glfwSetKeyCallback(self.handle, Some(key_callback_trampoline));
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

// DO NOT PANIC HERE EVER
extern "C" fn key_callback_trampoline(
    window: *mut gl::GLFWwindow,
    key: c_int,
    _scancode: c_int,
    action: c_int,
    mods: c_int,
) {
    let keycode = Keycode::from(key as u32);
    let action = Action::from(action as u32);
    let modifier = Modifier::from(mods as u32);

    unsafe {
        let user_ptr = gl::glfwGetWindowUserPointer(window);
        if !user_ptr.is_null() {
            let closure: &mut Box<KeyCallback> = &mut *(user_ptr as *mut Box<KeyCallback>);
            closure(keycode, action, modifier);
        }
    }
}
