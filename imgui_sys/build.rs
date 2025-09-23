use std::path::PathBuf;

use build_utils::{opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

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
        .include("imgui")
        .include("imgui/backends")
        .flag_if_supported("-std=c++17");

    #[cfg(target_os = "macos")]
    build.include("/opt/homebrew/include");

    build
        .try_compile("imgui")
        .expect("Could not compile imgui header");

    let bindings = opengl_builder()
        .header("imguiwrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-Iimgui")
        .clang_arg("-Iimgui/backends")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("imgui_bindings.rs");
    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");
}
