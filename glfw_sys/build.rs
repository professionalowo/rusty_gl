use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() {
    print_build_flags();

    let out_path = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR not set");

    let opengl_builder = opengl_builder();

    bind_glfw(opengl_builder, &out_path.join("glfw_bindings.rs"))
        .expect("Failed to build GLFW bindings");
}

fn bind_glfw(builder: bindgen::Builder, out_path: &PathBuf) -> io::Result<()> {
    builder
        .header_contents("glfwwrapper.h", "#include <GLFW/glfw3.h>")
        .allowlist_var("GLFW_.*")
        .allowlist_function("gl.*")
        .allowlist_type("GLFW.*")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate GLFW bindings")
        .write_if_changed(out_path)
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
        .clang_arg("-I/opt/homebrew/include") // For glfw3.h
        .clang_arg(format!("-F{sdk_path}/System/Library/Frameworks")) // For frameworks themselves
}

#[cfg(not(target_os = "macos"))]
fn opengl_builder() -> bindgen::Builder {
    bindgen::builder()
}

fn print_build_flags() {
    println!("cargo:rerun-if-changed=build.rs");
    for entry in glob::glob("c/**/*").expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to read file path");
        println!("cargo:rerun-if-changed={}", path.display());
    }
    print_os_flags();
}

#[cfg(target_os = "macos")]
fn print_os_flags() {
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=dylib=glfw");
}

#[cfg(not(target_os = "macos"))]
fn print_os_flags() {
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
}

#[derive(Debug)]
struct LazyBindings(bindgen::Bindings);
impl LazyBindings {
    fn write_if_changed<P>(&self, out_path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let new_contents = self.0.to_string();

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
