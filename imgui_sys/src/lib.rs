use std::ffi::{CString, NulError};

use glfw_sys::bindings::GLFWwindow;

use crate::{
    bindings::{
        ImGui_Begin, ImGui_CreateContext, ImGui_DestroyContext, ImGui_End, ImGui_GetDrawData,
        ImGui_GetIO, ImGui_ImplGlfw_InitForOpenGL, ImGui_ImplGlfw_NewFrame,
        ImGui_ImplGlfw_Shutdown, ImGui_ImplOpenGL3_Init, ImGui_ImplOpenGL3_NewFrame,
        ImGui_ImplOpenGL3_RenderDrawData, ImGui_ImplOpenGL3_Shutdown, ImGui_NewFrame, ImGui_Render,
        ImGui_SetNextWindowPos, ImGui_SetNextWindowSize, ImGui_Text, ImGuiCond,
        ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
        ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard, ImGuiContext, ImVec2,
    },
    io::IO,
};

pub mod bindings;
pub mod io;
mod wrapper;

#[derive(Debug)]
pub enum ImGuiError {
    CreateContextFailed,
    GetIoFailed,
    NulError(NulError),
}

impl From<NulError> for ImGuiError {
    fn from(value: NulError) -> Self {
        Self::NulError(value)
    }
}

wrapper_impl! {
    struct Context(ImGuiContext)
}

impl Context {
    #[must_use]
    pub fn init<S: Into<Vec<u8>>>(
        window: &mut GLFWwindow,
        glsl_version: S,
    ) -> Result<Self, ImGuiError> {
        let glsl_version = CString::new(glsl_version)?;
        let ctx = match Self::new(unsafe { ImGui_CreateContext(std::ptr::null_mut()) }) {
            None => return Err(ImGuiError::CreateContextFailed),
            Some(ctx) => ctx,
        };
        let mut io = match IO::new(unsafe { ImGui_GetIO() }) {
            None => return Err(ImGuiError::GetIoFailed),
            Some(io) => io,
        };
        io.ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard as i32;
        io.ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad as i32;
        unsafe {
            //FIXME: may overwrite existing callbacks
            ImGui_ImplGlfw_InitForOpenGL(window, true);
            ImGui_ImplOpenGL3_Init(glsl_version.as_ptr());
        };
        Ok(ctx)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ImGui_ImplOpenGL3_Shutdown();
            ImGui_ImplGlfw_Shutdown();
            ImGui_DestroyContext(self.as_ptr());
        }
    }
}

#[inline]
pub fn begin(title: impl Into<Vec<u8>>) -> Result<(), NulError> {
    unsafe {
        ImGui_Begin(CString::new(title)?.as_ptr(), std::ptr::null_mut(), 0);
    }
    Ok(())
}

#[inline]
pub fn end() {
    unsafe {
        ImGui_End();
    }
}

#[macro_export]
macro_rules! text {
	($s:expr) => {{
		$crate::text($s)
	}};

	($fmt:expr, $($arg:tt)*) => {{
		let str = format!($fmt, $($arg)*);
        $crate::text(str.as_str())
    }};
}

#[inline]
pub fn text(title: impl Into<Vec<u8>>) -> Result<(), NulError> {
    unsafe {
        ImGui_Text(CString::new(title)?.as_ptr());
    }
    Ok(())
}

#[inline]
pub fn set_next_window_size<V: Into<ImVec2>, C: Into<ImGuiCond>>(size: V, cond: C) {
    unsafe {
        ImGui_SetNextWindowSize(&size.into(), cond.into());
    }
}

#[inline]
pub fn set_next_window_pos<V: Into<ImVec2>, C: Into<ImGuiCond>, P: Into<ImVec2>>(
    pos: V,
    cond: C,
    pivot: P,
) {
    unsafe {
        ImGui_SetNextWindowPos(&pos.into(), cond.into(), &pivot.into());
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

#[derive(Debug)]
pub enum ImGuiCondition {
    Once,
    Always,
    Appearing,
    FirstEverUse,
    None,
}

impl From<ImGuiCondition> for ImGuiCond {
    fn from(value: ImGuiCondition) -> Self {
        let v = match value {
            ImGuiCondition::Always => bindings::ImGuiCond__ImGuiCond_Always,
            ImGuiCondition::Appearing => bindings::ImGuiCond__ImGuiCond_Appearing,
            ImGuiCondition::FirstEverUse => bindings::ImGuiCond__ImGuiCond_FirstUseEver,
            ImGuiCondition::Once => bindings::ImGuiCond__ImGuiCond_Once,
            ImGuiCondition::None => bindings::ImGuiCond__ImGuiCond_None,
        };
        v as _
    }
}
