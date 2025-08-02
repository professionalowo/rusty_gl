mod gl;
mod glfw;
mod math;

use std::path::PathBuf;

use gl::program::Program;
use gl::shader::Shader;
use glfw::window::Window;
use math::vec2::Vec2;
use math::vec3::Vec3;

fn main() {
    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MINOR, 1);
    glfw::window_hint(gl::GLFW_OPENGL_PROFILE, gl::GLFW_OPENGL_CORE_PROFILE);
    glfw::window_hint(gl::GLFW_OPENGL_FORWARD_COMPAT, gl::GLFW_TRUE);

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

    let vertices: [Vec2<f32>; 3] = [
        Vec2::new(-0.5, -0.5), // bottom-left
        Vec2::new(0.5, -0.5),  // bottom-right
        Vec2::new(0.0, 0.5),   // top-center
    ];

    let colors: [Vec3<f32>; 3] = [
        Vec3::new(1.0, 0.0, 0.0), // red
        Vec3::new(0.0, 1.0, 0.0), // green
        Vec3::new(0.0, 0.0, 1.0), // blue
    ];

    gl::vao::bind_vertex_array(vao);

    gl::vbo::bind_buffer(gl::GL_ARRAY_BUFFER, vbo);
    gl::vbo::buffer_data(gl::GL_ARRAY_BUFFER, &vertices, gl::GL_STATIC_DRAW);

    gl::vbo::enable_vertex_attrib_array(0);
    gl::vbo::vertex_attrib_pointer::<f32>(0, Vec2::<f32>::size(), gl::GL_FLOAT, gl::GL_FALSE, None);

    gl::vbo::bind_buffer(gl::GL_ARRAY_BUFFER, cbo);
    gl::vbo::buffer_data(gl::GL_ARRAY_BUFFER, &colors, gl::GL_STATIC_DRAW);
    gl::vbo::vertex_attrib_pointer::<f32>(1, Vec3::<f32>::size(), gl::GL_FLOAT, gl::GL_FALSE, None);

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
