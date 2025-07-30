use std::ffi::{c_int, NulError};

pub mod window;

const GLFW_TRUE: c_int = 1;

#[link(name = "glfw", kind = "dylib")]
unsafe extern "C" {
    unsafe fn glfwInit() -> c_int;
    unsafe fn glfwTerminate();
}

pub fn init() -> Result<(), GlfwError> {
    unsafe {
        let code = glfwInit();
        if code != GLFW_TRUE {
            return Err(GlfwError(code));
        }
    }
    Ok(())
}

pub fn terminate() {
    unsafe {
        glfwTerminate();
    }
}
#[derive(Debug)]
pub struct GlfwError(i32);
