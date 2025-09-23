use std::ffi::{CString, NulError};

use glfw_sys::bindings::GLFWwindow;

use crate::bindings::{
    ImGui_CreateContext, ImGui_GetIO, ImGui_ImplGlfw_InitForOpenGL, ImGui_ImplOpenGL3_Init,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard,
};

pub mod bindings;

pub fn init<S: Into<Vec<u8>>>(window: *mut GLFWwindow, glsl_version: S) -> Result<(), NulError> {
    let glsl_version = CString::new(glsl_version)?;
    unsafe {
        ImGui_CreateContext(std::ptr::null_mut());
        let io = ImGui_GetIO();
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard as i32;
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad as i32;
        ImGui_ImplGlfw_InitForOpenGL(window, true);
        ImGui_ImplOpenGL3_Init(glsl_version.as_ptr());
    };
    Ok(())
}
