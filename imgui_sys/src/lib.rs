use std::ffi::{CString, NulError};

use glfw_sys::bindings::GLFWwindow;

use crate::bindings::{
    ImGui_CreateContext, ImGui_DestroyContext, ImGui_GetDrawData, ImGui_GetIO,
    ImGui_ImplGlfw_InitForOpenGL, ImGui_ImplGlfw_NewFrame, ImGui_ImplGlfw_Shutdown,
    ImGui_ImplOpenGL3_Init, ImGui_ImplOpenGL3_NewFrame, ImGui_ImplOpenGL3_RenderDrawData,
    ImGui_ImplOpenGL3_Shutdown, ImGui_NewFrame, ImGui_Render,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard, ImGuiContext,
};

pub mod bindings;

#[derive(Debug)]
#[repr(transparent)]
pub struct Context(*mut ImGuiContext);

#[must_use]
pub fn init<S: Into<Vec<u8>>>(
    window: *mut GLFWwindow,
    glsl_version: S,
) -> Result<Context, NulError> {
    let glsl_version = CString::new(glsl_version)?;
    let c = unsafe {
        let ctx = ImGui_CreateContext(std::ptr::null_mut());
        let io = ImGui_GetIO();
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard as i32;
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad as i32;
        //FIXME: may overwrite existing callbacks
        ImGui_ImplGlfw_InitForOpenGL(window, true);
        ImGui_ImplOpenGL3_Init(glsl_version.as_ptr());
        ctx
    };
    Ok(Context(c))
}

pub fn shutdown(Context(ctx): Context) {
    unsafe {
        ImGui_ImplOpenGL3_Shutdown();
        ImGui_ImplGlfw_Shutdown();
        ImGui_DestroyContext(ctx);
    }
}

pub fn begin_drawing() {
    unsafe {
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui_NewFrame();
    }
}
pub fn end_drawing() {
    unsafe {
        ImGui_Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui_GetDrawData());
    }
}
