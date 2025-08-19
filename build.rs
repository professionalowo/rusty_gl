extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=dylib=glfw");
    }
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    build_gl(&out_path, "gl_bindings.rs").expect("Failed to build OpenGL bindings");
    build_glfw(&out_path, "glfw_bindings.rs").expect("Failed to build GLFW bindings");
    build_stbi(&out_path, "stbi_bindings.rs").expect("Failed to build STBI bindings");
}

fn build_gl<P>(out_path: &PathBuf, bindings_file: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    opengl_builder()
        .header("glwrapper.h")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join(bindings_file))
}

fn build_glfw<P>(out_path: &PathBuf, bindings_file: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    opengl_builder()
        .header("glfwwrapper.h")
        .generate()
        .expect("Unable to generate GLFW bindings")
        .write_to_file(out_path.join(bindings_file))
}

fn build_stbi<P>(out_path: &PathBuf, bindings_file: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    cc::Build::new()
        .file("stb_image_impl.c")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .compile("stb_image");

    bindgen::Builder::default()
        .header("stb_image.h")
        .allowlist_function("stbi_loadf_from_memory")
        .allowlist_function("stbi_load_from_memory")
        .allowlist_function("stbi_set_flip_vertically_on_load")
        .allowlist_function("stbi_is_hdr_from_memory")
        .allowlist_function("stbi_failure_reason")
        .clang_arg("-DSTB_IMAGE_IMPLEMENTATION")
        .clang_arg("-DSTBI_ONLY_PNG")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join(bindings_file))
}

fn opengl_builder() -> bindgen::Builder {
    let builder = bindgen::Builder::default();
    if cfg!(target_os = "macos") {
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
        builder
            .clang_arg("-I/opt/homebrew/include")
            .clang_arg("-I/opt/homebrew/opt/glfw/include")
            .clang_arg(format!(
                "-I{}/System/Library/Frameworks/OpenGL.framework/Headers",
                sdk_path
            ))
            .clang_arg(format!("-F{}/System/Library/Frameworks", sdk_path)) // For frameworks themselves
    } else {
        builder
    }
}
