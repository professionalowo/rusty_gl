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

pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

unsafe extern "C" {
    unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn glClear(mask: u32);
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha) };
}

pub fn clear(mask: u32) {
    unsafe { glClear(mask) };
}