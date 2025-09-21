use std::{env, io, path::PathBuf};

use build_utils::{LazyBindings, opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    let out_path = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR not set");

    let opengl_builder = opengl_builder();

    bind_glfw(opengl_builder, &out_path.join("glfw_bindings.rs"))
        .expect("Failed to build GLFW bindings");
}

fn bind_glfw(builder: bindgen::Builder, out_path: &PathBuf) -> io::Result<()> {
    builder
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_var("GLFW_.*")
        .allowlist_function("gl.*")
        .allowlist_type("GLFW.*")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate GLFW bindings")
        .write_if_changed(out_path)
}
