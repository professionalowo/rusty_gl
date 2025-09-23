use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("imguiwrapper.cpp")
        .flags(["-Wno-unused-parameter", "-Wno-unused-function"])
        .include("imgui")
        .include("imgui/backends")
        .flag_if_supported("-std=c++17")
        .try_compile("imgui")
        .expect("Could not compile imgui header");

    let bindings = bindgen::Builder::default()
        .header("imguiwrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-Iimgui")
        .clang_arg("-Iimgui/backends")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("imgui_bindings.rs");
    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");
}
