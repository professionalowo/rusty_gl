#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

pub mod program;
pub mod shader;
pub mod uniform;
pub mod vao;
pub mod vbo;

#[cfg(target_os = "macos")]
#[link(name = "OpenGL", kind = "framework")]
#[allow(missing_abi)]
unsafe extern "C" {}

#[cfg(target_os = "linux")]
#[link(name = "GL")]
#[allow(missing_abi)]
unsafe extern "C" {}

#[cfg(target_os = "windows")]
#[link(name = "opengl32")]
#[allow(missing_abi)]
unsafe extern "C" {}

pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe { glDrawArrays(mode, first, count) };
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha) };
}

pub fn clear(mask: u32) {
    unsafe { glClear(mask) };
}
