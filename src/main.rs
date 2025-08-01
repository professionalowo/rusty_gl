mod gl;
mod glfw;

use std::path::PathBuf;

use glfw::window::Window;
use gl::shader::Shader;

fn main() {
    
    if let Err(e) = glfw::init() {
        panic!("Failed to initialize GLFW: {:?}", e);
    }

    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MINOR, 1);
    glfw::window_hint(glfw::GLFW_OPENGL_PROFILE, glfw::GLFW_OPENGL_CORE_PROFILE);
    glfw::window_hint(glfw::GLFW_OPENGL_FORWARD_COMPAT, glfw::GLFW_TRUE);

    let window = match Window::try_new(640, 320, "Rust") {
        Ok(w) => w,
        Err(e) => panic!("Failed to create window: {:#?}", e),
    };

    let vertex_shader = Shader::try_from_path(gl::shader::GL_VERTEX_SHADER, get_shader_file_path("vertex.vert"))
        .expect("Failed to create vertex shader");

    let fragment_shader = Shader::try_from_path(gl::shader::GL_FRAGMENT_SHADER, get_shader_file_path("fragment.frag"))
        .expect("Failed to create fragment shader");

    while let Ok(false) = window.should_close() {
        gl::clear_color(1.0, 0.0, 0.0, 1.0);
        gl::clear(gl::COLOR_BUFFER_BIT);
        window.swap_buffers();
        window.poll_events();
    }
    glfw::terminate();
}

fn get_shader_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");  // project root
    PathBuf::from(manifest_dir)
        .join("shaders")
        .join(filename)
}
