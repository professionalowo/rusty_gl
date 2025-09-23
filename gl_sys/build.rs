use std::{env, io, path::PathBuf};

use build_utils::{LazyBindings, bindgen, opengl_builder, print_build_flags};

fn main() {
    print_build_flags();

    let out_path = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR not set");

    let opengl_builder = opengl_builder();

    bind_gl(opengl_builder, &out_path.join("gl_bindings.rs"))
        .expect("Failed to build OpenGL bindings");
}

fn bind_gl(builder: bindgen::Builder, out_path: &PathBuf) -> io::Result<()> {
    builder
        .header("glwrapper.h")
        .allowlist_var("GL_.*")
        .allowlist_function("gl.*")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate OpenGL bindings")
        .write_if_changed(out_path)
}
