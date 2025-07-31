mod gl;
mod glfw;

use glfw::window::Window;

fn main() {
    match glfw::init() {
        Ok(_) => println!("GLFW initialized successfully!"),
        Err(e) => panic!("Failed to initialize GLFW: {:?}", e),
    }
    let window = match Window::try_new(640, 320, "Rust") {
        Ok(w) => w,
        Err(e) => panic!("Failed to create window: {:#?}", e),
    };
    while let Ok(false) = window.should_close() {
        window.swap_buffers();
        window.poll_events();
    }
    glfw::terminate();
}
