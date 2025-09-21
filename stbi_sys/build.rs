use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() {
    print_build_flags();

    let out_path = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR not set");

    bind_stbi(&out_path.join("stbi_bindings.rs")).expect("Failed to build STBI bindings");
}

fn bind_stbi(out_path: &PathBuf) -> io::Result<()> {
    fn with_simd(mut build: cc::Build) -> cc::Build {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            build.flag_if_supported("-msse2");
        } else if cfg!(all(target_arch = "aarch64", target_feature = "neon")) {
            build.define("STBI_NEON", None);
            build.flag_if_supported("-mfpu=neon");
        }
        build
    }

    const HEADER: &'static str = "stb_image.h";

    with_simd(cc::Build::new())
        .file(HEADER)
        .flags(["-x", "c"])
        .flags(["-Wno-unused-parameter", "-Wno-unused-function"])
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .define("STBI_NO_STDIO", None)
        .try_compile("stb_image")
        .expect("Could not compile STBI header");

    bindgen::builder()
        .header(HEADER)
        .allowlist_function("stbi_loadf_from_memory")
        .allowlist_function("stbi_load_from_memory")
        .allowlist_function("stbi_set_flip_vertically_on_load")
        .allowlist_function("stbi_is_hdr_from_memory")
        .allowlist_function("stbi_failure_reason")
        .generate()
        .map(LazyBindings)
        .expect("Unable to generate STBI bindings")
        .write_if_changed(out_path)
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
