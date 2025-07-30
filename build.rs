fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=dylib=glfw");
    }
}
