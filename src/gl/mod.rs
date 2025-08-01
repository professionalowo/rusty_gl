pub mod shader;
pub mod program;
pub mod vao;
pub mod vbo;

#[cfg(target_os = "macos")]
#[link(name = "OpenGL", kind = "framework")]
#[allow(missing_abi)]
unsafe extern {}

#[cfg(target_os = "linux")]
#[link(name = "GL")]
#[allow(missing_abi)]
unsafe extern {}

#[cfg(target_os = "windows")]
#[link(name = "opengl32")]
#[allow(missing_abi)]
unsafe extern {}

pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_FALSE: u8 = 0;
pub const GL_TRUE: u8 = 1;
pub const GL_TRIANGLES: u32 = 0x0004;

unsafe extern "C" {
    unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn glClear(mask: u32);
    unsafe fn glDrawArrays(mode: u32, first: i32, count: i32);
}

pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe { glDrawArrays(mode, first, count) };
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha) };
}

pub fn clear(mask: u32) {
    unsafe { glClear(mask) };
}