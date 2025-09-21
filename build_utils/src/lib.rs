use std::{fs, io, path::Path};

#[cfg(target_os = "macos")]
pub fn opengl_builder() -> bindgen::Builder {
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
pub fn opengl_builder() -> bindgen::Builder {
    bindgen::builder()
}

pub fn print_build_flags() {
    println!("cargo:rerun-if-changed=build.rs");
    print_os_flags();
}

#[cfg(target_os = "macos")]
pub fn print_os_flags() {
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=dylib=glfw");
}

#[cfg(not(target_os = "macos"))]
pub fn print_os_flags() {
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
}

#[derive(Debug)]
pub struct LazyBindings(pub bindgen::Bindings);
impl LazyBindings {
    pub fn write_if_changed<P>(&self, out_path: P) -> io::Result<()>
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
