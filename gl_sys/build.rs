use std::{env, path::PathBuf};

use build_utils::{LazyBindings, opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    let out_path: PathBuf = env::var("OUT_DIR").expect("OUT_DIR not set").into();

    opengl_builder()
        .header("glwrapper.h")
        .allowlist_var("GL_.*")
        .allowlist_function("gl.*")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate OpenGL bindings")
        .write_if_changed(out_path.join("gl_bindings.rs"))
        .expect("Failed to build OpenGL bindings");
}
