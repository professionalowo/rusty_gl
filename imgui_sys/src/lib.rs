use crate::bindings::{
    GLFWwindow, ImGui_CreateContext, ImGui_GetIO, ImGui_ImplGlfw_InitForOpenGL,
    ImGui_ImplOpenGL3_Init, ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
    ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard,
};

pub mod bindings;

fn init(window: *mut GLFWwindow) {
    unsafe {
        ImGui_CreateContext(std::ptr::null_mut());
        let io = ImGui_GetIO();
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard as i32;
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad as i32;
        ImGui_ImplGlfw_InitForOpenGL(window, true);
        ImGui_ImplOpenGL3_Init("#version 410 core".as_ptr().cast());
    };
}
