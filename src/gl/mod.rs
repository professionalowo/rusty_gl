#[link(name = "OpenGL", kind = "framework")]
unsafe extern "C" {
    unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn glClear(mask: u32);
}
