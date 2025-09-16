extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for entry in glob::glob("c/**/*").expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to read file path");
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=dylib=glfw");
    } else {
        println!("cargo:rustc-link-lib=glfw");
        println!("cargo:rustc-link-lib=GL");
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
    let bindings = opengl_builder()
        .header("c/glwrapper.h")
        .allowlist_var("GL_.*")
        .allowlist_function("gl.*")
        .generate()
        .expect("Unable to generate bindings");

    write_bindings_if_changed(bindings, out_path.join(bindings_file))
}

fn build_glfw<P>(out_path: &PathBuf, bindings_file: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let bindings = opengl_builder()
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_var("GLFW_.*")
        .allowlist_function("gl.*")
        .allowlist_type("GLFW.*")
        .generate()
        .expect("Unable to generate GLFW bindings");

    write_bindings_if_changed(bindings, out_path.join(bindings_file))
}

fn build_stbi<P>(out_path: &PathBuf, bindings_file: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut build = cc::Build::new();
    build
        .file("c/stb_image.h") // header only
        .flag("-x") // next argument specifies language
        .flag("c") // treat as C
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .define("STBI_NO_STDIO", None) // enable implementation
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-march=native");

    if cfg!(all(target_arch = "aarch64", target_feature = "neon")) {
        build
            .define("STBI_NEON", None)
            .flag_if_supported("-mfpu=neon");
    }
    build.compile("stb_image"); // produces libstb_image.a

    let bindings = bindgen::Builder::default()
        .header("c/stb_image.h")
        .allowlist_function("stbi_loadf_from_memory")
        .allowlist_function("stbi_load_from_memory")
        .allowlist_function("stbi_set_flip_vertically_on_load")
        .allowlist_function("stbi_is_hdr_from_memory")
        .allowlist_function("stbi_failure_reason")
        .generate()
        .expect("Unable to generate bindings");

    write_bindings_if_changed(bindings, out_path.join(bindings_file))
}

fn opengl_builder() -> bindgen::Builder {
    let builder = bindgen::Builder::default().clang_arg("-DGL_GLEXT_PROTOTYPES");
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

fn write_bindings_if_changed(
    bindings: bindgen::Bindings,
    out_path: PathBuf,
) -> std::io::Result<()> {
    let new_contents = bindings.to_string();

    // Check if the file already exists
    if let Ok(existing_contents) = std::fs::read_to_string(&out_path) {
        if existing_contents == new_contents {
            // No change, skip writing
            return Ok(());
        }
    }

    // Write new contents
    std::fs::write(out_path, new_contents)
}
