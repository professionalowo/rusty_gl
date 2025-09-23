use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let imgui_path = manifest_dir.join("../imgui");

    cc::Build::new()
        .cpp(true)
        .file(manifest_dir.join("imguiwrapper.cpp"))
        .flags(["-Wno-unused-parameter", "-Wno-unused-function"])
        .include(&imgui_path)
        .include(format!("{}/backends", imgui_path.display()))
        .flag_if_supported("-std=c++17")
        .try_compile("imgui")
        .expect("Could not compile STBI header");

    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("imguiwrapper.h").to_str().unwrap())
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", imgui_path.display()))
        .clang_arg(format!("-I{}/backends", imgui_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("imgui_bindings.rs"))
        .expect("Couldn't write bindings!");
}
