extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    cc::Build::new()
        .file("stb_image_impl.c")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("stb_image");

    let mut gl_builder = bindgen::Builder::default()
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
        gl_builder = gl_builder
            .clang_arg(format!(
                "-I{}/System/Library/Frameworks/OpenGL.framework/Headers",
                sdk_path
            ))
            .clang_arg(format!("-F{}/System/Library/Frameworks", sdk_path)) // For frameworks themselves
            .clang_arg("-I/opt/homebrew/include") // Still useful for other Homebrew headers
            .clang_arg("-I/opt/homebrew/opt/glfw/include");
    }

    let gl_bindings = gl_builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    gl_bindings
        .write_to_file(out_path.join("gl_bindings.rs"))
        .expect("Couldn't write bindings!");

    let stbi_builder = bindgen::Builder::default()
        .header("stb_image.h")
        .allowlist_function("stbi_loadf")
        .allowlist_function("stbi_load")
        .allowlist_function("stbi_set_flip_vertically_on_load")
        .allowlist_function("stbi_is_hdr")
        .clang_arg("-DSTB_IMAGE_IMPLEMENTATION");
    let stbi_bindings = stbi_builder
        .generate()
        .expect("Unable to generate bindings");

    stbi_bindings
        .write_to_file(out_path.join("stbi_bindings.rs"))
        .expect("Couldn't write STBI bindings!");
}
