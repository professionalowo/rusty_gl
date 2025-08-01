extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut builder = bindgen::Builder::default()
        .header("glwrapper.h")
        .clang_arg("-DGL_GLEXT_PROTOTYPES");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=dylib=glfw");

        let sdk_path_output = Command::new("xcrun")
            .arg("--sdk")
            .arg("macosx")
            .arg("--show-sdk-path")
            .output()
            .expect("Failed to execute xcrun to get SDK path");

        let sdk_path = String::from_utf8(sdk_path_output.stdout)
            .expect("Failed to parse xcrun output as UTF-8")
            .trim()
            .to_string();

        // Add the SDK include path
        builder = builder
            .clang_arg(format!(
                "-I{}/System/Library/Frameworks/OpenGL.framework/Headers",
                sdk_path
            ))
            .clang_arg(format!("-F{}/System/Library/Frameworks", sdk_path)) // For frameworks themselves
            .clang_arg("-I/opt/homebrew/include") // Still useful for other Homebrew headers
            .clang_arg("-I/opt/homebrew/opt/glfw/include");
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("gl_bindings.rs"))
        .expect("Couldn't write bindings!");
}
