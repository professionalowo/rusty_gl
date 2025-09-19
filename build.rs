use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
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
    let out_path = env::var("OUT_DIR").expect("OUT_DIR not set").into();
    bind_gl(&out_path, "gl_bindings.rs").expect("Failed to build OpenGL bindings");
    bind_glfw(&out_path, "glfw_bindings.rs").expect("Failed to build GLFW bindings");
    bind_stbi(&out_path, "stbi_bindings.rs").expect("Failed to build STBI bindings");
}

fn bind_gl<P>(out_path: &PathBuf, bindings_file: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let bindings = opengl_builder()
        .header("c/glwrapper.h")
        .allowlist_var("GL_.*")
        .allowlist_function("gl.*")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .expect("Unable to generate bindings");

    write_bindings_if_changed(bindings, out_path.join(bindings_file))
}

fn bind_glfw<P>(out_path: &PathBuf, bindings_file: P) -> io::Result<()>
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

fn bind_stbi<P>(out_path: &PathBuf, bindings_file: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    fn with_simd(mut build: cc::Build) -> cc::Build {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            build.flag_if_supported("-msse2");
        } else if cfg!(all(target_arch = "aarch64", target_feature = "neon")) {
            build.define("STBI_NEON", None);
            build.flag_if_supported("-mfpu=neon");
        }
        build
    }

    with_simd(cc::Build::new())
        .file("c/stb_image.h")
        .flag("-x")
        .flag("c")
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .define("STBI_NO_STDIO", None)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .compile("stb_image");

    let bindings = bindgen::builder()
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
    let builder = bindgen::builder();
    if cfg!(target_os = "macos") {
        let Output { ref stdout, .. } = Command::new("xcrun")
            .arg("--sdk")
            .arg("macosx")
            .arg("--show-sdk-path")
            .output()
            .expect("Failed to execute xcrun to get SDK path");

        let sdk_path = str::from_utf8(stdout)
            .expect("Failed to parse xcrun output as UTF-8")
            .trim();

        builder
            .clang_arg("-I/opt/homebrew/include")
            .clang_arg(format!("-F{}/System/Library/Frameworks", sdk_path)) // For frameworks themselves
    } else {
        builder
    }
}

fn write_bindings_if_changed(
    bindings: bindgen::Bindings,
    out_path: impl AsRef<Path>,
) -> io::Result<()> {
    let new_contents = bindings.to_string();

    // Check if the file already exists
    if let Ok(existing_contents) = fs::read_to_string(&out_path)
        && existing_contents == new_contents
    {
        Ok(())
    } else {
        fs::write(&out_path, new_contents.as_str())
    }
}
