use std::{env, path::PathBuf};

use build_utils::{LazyBindings, opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    let out_path: PathBuf = env::var("OUT_DIR").expect("OUT_DIR not set").into();

    const PREFIX: &'static str = "GLFW.*";

    opengl_builder()
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_function(PREFIX.to_lowercase())
        .allowlist_type(PREFIX)
        .allowlist_var(PREFIX)
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate GLFW bindings")
        .write_if_changed(out_path.join("glfw_bindings.rs"))
        .expect("Could not write glfw bindings");
}
