use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
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

    let opengl_builder = opengl_builder();

    bind_gl(opengl_builder.clone(), &out_path, "gl_bindings.rs")
        .expect("Failed to build OpenGL bindings");
    bind_glfw(opengl_builder, &out_path, "glfw_bindings.rs")
        .expect("Failed to build GLFW bindings");
    bind_stbi(&out_path, "stbi_bindings.rs").expect("Failed to build STBI bindings");
}

fn bind_gl<P>(builder: bindgen::Builder, out_path: &PathBuf, bindings_file: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let bindings = builder
        .header("c/glwrapper.h")
        .allowlist_var("GL_.*")
        .allowlist_function("gl.*")
        .clang_arg("-DGL_GLEXT_PROTOTYPES")
        .generate()
        .expect("Unable to generate bindings");

    LazyBindings(bindings).write_if_changed(out_path.join(bindings_file))
}

fn bind_glfw<P>(builder: bindgen::Builder, out_path: &PathBuf, bindings_file: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let bindings = builder
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_var("GLFW_.*")
        .allowlist_function("gl.*")
        .allowlist_type("GLFW.*")
        .generate()
        .expect("Unable to generate GLFW bindings");

    LazyBindings(bindings).write_if_changed(out_path.join(bindings_file))
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
        .flags(["-x", "c"])
        .flags(["-Wno-unused-parameter", "-Wno-unused-function"])
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .define("STBI_NO_STDIO", None)
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

    LazyBindings(bindings).write_if_changed(out_path.join(bindings_file))
}

#[cfg(target_os = "macos")]
fn opengl_builder() -> bindgen::Builder {
    use std::process::{Command, Output};

    let Output { ref stdout, .. } = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("Failed to execute xcrun to get SDK path");

    let sdk_path = str::from_utf8(stdout)
        .expect("Failed to parse xcrun output as UTF-8")
        .trim();

    bindgen::builder()
        .clang_arg("-I/opt/homebrew/include")
        .clang_arg(format!("-F{}/System/Library/Frameworks", sdk_path)) // For frameworks themselves
}

#[cfg(not(target_os = "macos"))]
fn opengl_builder() -> bindgen::Builder {
    bindgen::builder()
}

#[derive(Debug)]
struct LazyBindings(bindgen::Bindings);
impl LazyBindings {
    fn write_if_changed<P>(&self, out_path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let new_contents = self.to_string();

        // Check if the file already exists
        if let Ok(existing_contents) = fs::read_to_string(&out_path)
            && existing_contents == new_contents
        {
            Ok(())
        } else {
            fs::write(&out_path, new_contents.as_str())
        }
    }
}

impl fmt::Display for LazyBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
