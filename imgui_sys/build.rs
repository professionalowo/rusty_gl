use std::path::PathBuf;

use build_utils::{opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    const CXXSTD: &'static str = "-std=c++14";

    const INCLUDES: [&'static str; 2] = ["imgui", "imgui/backends"];

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .files([
            "imgui/imgui.cpp",
            "imgui/imgui_draw.cpp",
            "imgui/imgui_tables.cpp",
            "imgui/imgui_widgets.cpp",
            "imgui/backends/imgui_impl_glfw.cpp",
            "imgui/backends/imgui_impl_opengl3.cpp",
        ])
        .file("imguiwrapper.cpp")
        .flags(["-Wno-unused-parameter", "-Wno-unused-function"])
        .includes(INCLUDES)
        .flag_if_supported(CXXSTD);

    #[cfg(target_os = "macos")]
    build.include("/opt/homebrew/include");

    build
        .try_compile("imgui")
        .expect("Could not compile imgui header");

    let bindings = opengl_builder()
        .header("imguiwrapper.h")
        .clang_args(["-x", "c++"])
        .clang_arg(CXXSTD)
        .clang_args(INCLUDES.map(|i| format!("-I{i}")))
        .allowlist_function("ImGui.*")
        .allowlist_type("ImGui.*")
        .allowlist_var("ImGui.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("imgui_bindings.rs");
    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");
}
