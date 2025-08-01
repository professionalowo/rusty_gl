extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=dylib=glfw");
    }
    let bindings = bindgen::Builder::default()
        .header("glwrapper.h")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("gl_bindings.rs"))
        .expect("Couldn't write bindings!");
}
