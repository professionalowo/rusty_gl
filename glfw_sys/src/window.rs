use std::{
    cell::RefCell,
    ffi::{CStr, CString, NulError, c_int},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

use gl_sys::bindings::glViewport;

use crate::{
    GLFWError, bindings,
    input::{KeyEvent, action::Action, keycode::Keycode, modifier::Modifier},
};
pub struct Window {
    handle: NonNull<bindings::GLFWwindow>,
    last_event: Rc<RefCell<Option<KeyEvent>>>,
    _title: CString,
}

type KeyCallback = dyn FnMut(KeyEvent);

impl Window {
    pub fn try_new<B>(width: u32, height: u32, title: B) -> Result<Self, NulError>
    where
        B: Into<Vec<u8>>,
    {
        let title_cstr = CString::new(title)?;
        let handle = unsafe {
            NonNull::new_unchecked(create_window(width, height, &title_cstr, None, None))
        };
        let last_event = Rc::new(RefCell::new(None));
        set_key_callback(handle.as_ptr(), {
            let last_key_event = last_event.clone();
            move |event| {
                *last_key_event.borrow_mut() = Some(event);
            }
        });
        unsafe {
            bindings::glfwSetFramebufferSizeCallback(
                handle.as_ptr(),
                Some(framebuffer_size_callback),
            );
        }
        Ok(Self {
            handle,
            last_event: last_event.clone(),
            _title: title_cstr,
        })
    }

    pub fn should_close(&self) -> Result<bool, GLFWError> {
        unsafe {
            let result = bindings::glfwWindowShouldClose(self.handle.as_ptr());
            if result == 0 {
                Ok(false)
            } else if result == 1 {
                Ok(true)
            } else {
                Err(GLFWError::from(result))
            }
        }
    }

    #[inline]
    pub fn set_should_close(&self, value: bool) {
        unsafe {
            bindings::glfwSetWindowShouldClose(self.handle.as_ptr(), if value { 1 } else { 0 });
        }
    }

    #[inline]
    pub fn poll_events(&self) {
        unsafe {
            bindings::glfwPollEvents();
        }
    }

    #[inline]
    pub fn swap_buffers(&self) {
        unsafe {
            bindings::glfwSwapBuffers(self.handle.as_ptr());
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        let (width, height) = self.framebuffer_size();
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
        unsafe {
            bindings::glfwGetFramebufferSize(self.handle.as_ptr(), &mut w, &mut h);
        }
        (w, h)
    }

    pub fn size(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            bindings::glfwGetWindowSize(self.handle.as_ptr(), &mut w, &mut h);
        }
        (w, h)
    }

    pub const fn as_ptr(&self) -> *const bindings::GLFWwindow {
        self.handle.as_ptr()
    }

    pub const fn as_mut_ptr(&mut self) -> *mut bindings::GLFWwindow {
        self.handle.as_ptr()
    }
}

impl Deref for Window {
    type Target = bindings::GLFWwindow;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.handle.as_ref() }
    }
}

impl DerefMut for Window {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.handle.as_mut() }
    }
}

impl AsRef<bindings::GLFWwindow> for Window {
    #[inline]
    fn as_ref(&self) -> &bindings::GLFWwindow {
        &(**self)
    }
}

impl AsMut<bindings::GLFWwindow> for Window {
    #[inline]
    fn as_mut(&mut self) -> &mut bindings::GLFWwindow {
        &mut (**self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let user_ptr = bindings::glfwGetWindowUserPointer(self.handle.as_ptr());
            if !user_ptr.is_null() {
                let closure = Box::from_raw(user_ptr as *mut Box<KeyCallback>);
                // Drop the closure to unregister the callback
                drop(closure);
            }
            bindings::glfwDestroyWindow(self.handle.as_ptr());
        }
    }
}

fn set_key_callback<F>(window: *mut bindings::GLFWwindow, callback: F)
where
    F: FnMut(KeyEvent) + 'static,
{
    let closure: Box<KeyCallback> = Box::new(callback);
    let raw = Box::into_raw(Box::new(closure));

    unsafe {
        bindings::glfwSetWindowUserPointer(window, raw as _);
        bindings::glfwSetKeyCallback(window, Some(key_callback_trampoline));
    }
}

fn create_window(
    width: u32,
    height: u32,
    title: &CStr,
    monitor: Option<*mut bindings::GLFWmonitor>,
    share: Option<*mut bindings::GLFWwindow>,
) -> *mut bindings::GLFWwindow {
    unsafe {
        let window = bindings::glfwCreateWindow(
            width as c_int,
            height as c_int,
            title.as_ptr(),
            monitor.unwrap_or(std::ptr::null_mut()),
            share.unwrap_or(std::ptr::null_mut()),
        );
        bindings::glfwMakeContextCurrent(window);
        window
    }
}

extern "C" fn framebuffer_size_callback(
    _window: *mut bindings::GLFWwindow,
    width: c_int,
    height: c_int,
) {
    unsafe {
        glViewport(0, 0, width, height);
    }
}

// DO NOT PANIC HERE EVER
extern "C" fn key_callback_trampoline(
    window: *mut bindings::GLFWwindow,
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
        let user_ptr = bindings::glfwGetWindowUserPointer(window);
        if !user_ptr.is_null() {
            let closure = &mut *(user_ptr as *mut Box<KeyCallback>);
            closure(event);
        }
    }
}
