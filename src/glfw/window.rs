use std::cell::RefCell;
use std::ffi::{CStr, CString, NulError, c_int, c_void};
use std::rc::Rc;

use crate::gl;
use crate::glfw::GLFWError;
use crate::glfw::input::KeyEvent;
use crate::glfw::input::action::Action;
use crate::glfw::input::keycode::Keycode;
use crate::glfw::input::modifier::Modifier;
pub struct Window {
    handle: *mut gl::GLFWwindow,
    last_event: Rc<RefCell<Option<KeyEvent>>>,
    _title: CString,
}

type KeyCallback = dyn FnMut(KeyEvent);

impl Window {
    pub fn try_new<S>(width: u32, height: u32, title: S) -> Result<Self, NulError>
    where
        S: AsRef<str>,
    {
        let title_cstr = CString::new(title.as_ref())?;
        let handle = create_window(width, height, &title_cstr, None, None);
        let last_event = Rc::new(RefCell::new(None));
        set_key_callback(handle, {
            let last_key_event = last_event.clone();
            move |event| {
                *last_key_event.borrow_mut() = Some(event);
            }
        });
        unsafe {
            gl::glfwSetFramebufferSizeCallback(handle, Some(framebuffer_size_callback));
        }
        Ok(Self {
            handle,
            last_event: last_event.clone(),
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

    pub fn set_should_close(&self, value: bool) {
        unsafe {
            gl::glfwSetWindowShouldClose(self.handle, if value { 1 } else { 0 });
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
            gl::glfwGetFramebufferSize(self.handle, &mut width, &mut height);
        }
        if height == 0 {
            1.0 // Avoid division by zero
        } else {
            width as f32 / height as f32
        }
    }

    pub fn pump_event(&mut self) -> Option<KeyEvent> {
        self.last_event.borrow_mut().take()
    }

	pub fn framebuffer_size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { gl::glfwGetFramebufferSize(self.handle, &mut w, &mut h); }
    (w, h)
}
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let user_ptr = gl::glfwGetWindowUserPointer(self.handle);
            if !user_ptr.is_null() {
                let closure: Box<KeyCallback> = Box::from_raw(user_ptr as *mut Box<KeyCallback>);
                // Drop the closure to unregister the callback
                drop(closure);
            }
            gl::glfwDestroyWindow(self.handle);
        }
    }
}

fn set_key_callback<F>(window: *mut gl::GLFWwindow, callback: F)
where
    F: FnMut(KeyEvent) + 'static,
{
    let closure: Box<KeyCallback> = Box::new(callback);
    let raw = Box::into_raw(Box::new(closure));

    unsafe {
        gl::glfwSetWindowUserPointer(window, raw as *mut c_void);
        gl::glfwSetKeyCallback(window, Some(key_callback_trampoline));
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

extern "C" fn framebuffer_size_callback(window: *mut gl::GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        gl::glViewport(0, 0, width, height);
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
    let event = KeyEvent::new(keycode, action, modifier);

    unsafe {
        let user_ptr = gl::glfwGetWindowUserPointer(window);
        if !user_ptr.is_null() {
            let closure: &mut Box<KeyCallback> = &mut *(user_ptr as *mut Box<KeyCallback>);
            closure(event);
        }
    }
}
