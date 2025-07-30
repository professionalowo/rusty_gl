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
extern {}

unsafe extern "C" {
    unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn glClear(mask: u32);
}
