use std::{env, io, path::PathBuf};

use build_utils::LazyBindings;

const HEADER: &'static str = "stb_image.h";

fn main() {
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
