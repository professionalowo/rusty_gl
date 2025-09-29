use std::ffi::{CString, NulError};

use glfw_sys::bindings::GLFWwindow;

use crate::bindings::{
    ImGui_Begin, ImGui_CreateContext, ImGui_DestroyContext, ImGui_End, ImGui_GetDrawData,
    ImGui_GetIO, ImGui_ImplGlfw_InitForOpenGL, ImGui_ImplGlfw_NewFrame, ImGui_ImplGlfw_Shutdown,
    ImGui_ImplOpenGL3_Init, ImGui_ImplOpenGL3_NewFrame, ImGui_ImplOpenGL3_RenderDrawData,
    ImGui_ImplOpenGL3_Shutdown, ImGui_NewFrame, ImGui_Render, ImGui_SetNextWindowPos,
    ImGui_SetNextWindowSize, ImGui_Text, ImGuiCond, ImGuiCond__ImGuiCond_Always,
    ImGuiCond__ImGuiCond_Appearing, ImGuiCond__ImGuiCond_FirstUseEver, ImGuiCond__ImGuiCond_None,
    ImGuiCond__ImGuiCond_Once, ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard, ImGuiContext, ImVec2,
};

pub mod bindings;

#[derive(Debug)]
#[repr(transparent)]
pub struct Context(*mut ImGuiContext);
impl Context {
    #[must_use]
    pub fn init<S: Into<Vec<u8>>, W: AsMut<GLFWwindow>>(
        window: &mut W,
        glsl_version: S,
    ) -> Result<Self, NulError> {
        let glsl_version = CString::new(glsl_version)?;
        let window = window.as_mut();
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
        Ok(Self(c))
    }

    pub fn shutdown(self) {
        unsafe {
            ImGui_ImplOpenGL3_Shutdown();
            ImGui_ImplGlfw_Shutdown();
            ImGui_DestroyContext(self.0);
        }
    }
}

pub fn begin<T: Into<Vec<u8>>>(title: T) -> Result<(), NulError> {
    unsafe {
        ImGui_Begin(CString::new(title)?.as_ptr(), std::ptr::null_mut(), 0);
    }
    Ok(())
}

pub fn end() {
    unsafe {
        ImGui_End();
    }
}

#[macro_export]
macro_rules! text {
	($($arg:tt)*) => {
        $crate::text(format!($($arg)*))
    };
}

pub fn text(title: impl AsRef<[u8]>) -> Result<(), NulError> {
    unsafe {
        ImGui_Text(CString::new(title.as_ref())?.as_ptr());
    }
    Ok(())
}

pub fn set_next_window_size<V: Into<ImVec2>, C: Into<ImGuiCond>>(size: V, cond: C) {
    unsafe {
        ImGui_SetNextWindowSize(&size.into(), cond.into());
    }
}

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
            ImGuiCondition::Always => ImGuiCond__ImGuiCond_Always,
            ImGuiCondition::Appearing => ImGuiCond__ImGuiCond_Appearing,
            ImGuiCondition::FirstEverUse => ImGuiCond__ImGuiCond_FirstUseEver,
            ImGuiCondition::Once => ImGuiCond__ImGuiCond_Once,
            ImGuiCondition::None => ImGuiCond__ImGuiCond_None,
        };
        v as _
    }
}
