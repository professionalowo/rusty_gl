use std::{env, path::PathBuf};

use build_utils::{LazyBindings, opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    let out_path: PathBuf = env::var("OUT_DIR").expect("OUT_DIR not set").into();

    opengl_builder()
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_function("glfw.*")
        .allowlist_type("GLFW.*")
        .allowlist_var("GLFW.*")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate GLFW bindings")
        .write_if_changed(out_path.join("glfw_bindings.rs"))
        .expect("Could not write glfw bindings");
}
