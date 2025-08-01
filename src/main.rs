mod gl;
mod glfw;

use std::path::PathBuf;

use gl::program::Program;
use gl::shader::Shader;
use glfw::window::Window;

fn main() {
    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MINOR, 1);
    glfw::window_hint(glfw::GLFW_OPENGL_PROFILE, glfw::GLFW_OPENGL_CORE_PROFILE);
    glfw::window_hint(glfw::GLFW_OPENGL_FORWARD_COMPAT, glfw::GLFW_TRUE);

    let window = match Window::try_new(640, 320, "Rust") {
        Ok(w) => w,
        Err(e) => panic!("Failed to create window: {:#?}", e),
    };

    let vertex_shader =
        Shader::try_from_path(gl::GL_VERTEX_SHADER, get_shader_file_path("vertex.vert"))
            .expect("Failed to create vertex shader");

    let fragment_shader = Shader::try_from_path(
        gl::GL_FRAGMENT_SHADER,
        get_shader_file_path("fragment.frag"),
    )
    .expect("Failed to create fragment shader");

    let program = Program::from_shaders(vec![vertex_shader, fragment_shader])
        .expect("Failed to create shader program");

    let vao = gl::vao::gen_vertex_arrays();
    let vbo = gl::vbo::gen_buffers();
    let cbo = gl::vbo::gen_buffers();

    let vertices: [f32; 6] = [
        -0.5, -0.5, // bottom-left
        0.5, -0.5, // bottom-right
        0.0, 0.5, // top-center
    ];

    let colors: [f32; 9] = [
        1.0, 0.0, 0.0, // red
        0.0, 1.0, 0.0, // green
        0.0, 0.0, 1.0, // blue
    ];

    gl::vao::bind_vertex_array(vao);

    gl::vbo::bind_buffer(gl::GL_ARRAY_BUFFER, vbo);
    gl::vbo::buffer_data(gl::GL_ARRAY_BUFFER, &vertices, gl::GL_STATIC_DRAW);

    gl::vbo::enable_vertex_attrib_array(0);
    gl::vbo::vertex_attrib_pointer::<f32>(
        0,
        2, // size
        gl::GL_FLOAT,
        gl::GL_FALSE,
        std::ptr::null(), // offset
    );

    gl::vbo::bind_buffer(gl::GL_ARRAY_BUFFER, cbo);
    gl::vbo::buffer_data(gl::GL_ARRAY_BUFFER, &colors, gl::GL_STATIC_DRAW);
    gl::vbo::vertex_attrib_pointer::<f32>(
        1, // layout location
        3, // vec3
        gl::GL_FLOAT,
        gl::GL_FALSE,
        std::ptr::null(),
    );

    gl::vbo::enable_vertex_attrib_array(1);

    gl::vbo::bind_buffer(gl::GL_ARRAY_BUFFER, 0);
    gl::vao::bind_vertex_array(0);

    while let Ok(false) = window.should_close() {
        gl::clear_color(0.0, 0.0, 0.0, 1.0);
        gl::clear(gl::GL_COLOR_BUFFER_BIT);

        program.bind();

        gl::vao::bind_vertex_array(vao);
        gl::draw_arrays(gl::GL_TRIANGLES, 0, 3);
        gl::vao::bind_vertex_array(0);

        program.unbind();

        window.swap_buffers();
        window.poll_events();
    }
    glfw::terminate();
}

fn get_shader_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join("shaders").join(filename)
}
