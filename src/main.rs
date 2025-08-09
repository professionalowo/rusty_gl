mod framework;
mod gl;
mod glfw;
mod math;

use std::path::PathBuf;

use crate::framework::camera::Camera;
use crate::gl::program::Program;
use crate::gl::shader::Shader;
use crate::gl::uniform::UniformLocation;
use crate::gl::vao::VertexArrayObject;
use crate::gl::vbo::{Location, VertexBufferObject};
use crate::glfw::window::Window;
use crate::math::mat4::Mat4;
use crate::math::vec3::Vec3;

fn main() {
    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(gl::GLFW_OPENGL_PROFILE, gl::GLFW_OPENGL_CORE_PROFILE);
    glfw::window_hint(gl::GLFW_OPENGL_FORWARD_COMPAT, gl::GLFW_TRUE);

    let window = Window::try_new(640, 320, "Rust").expect("Failed to create GLFW window");

    let vao = VertexArrayObject::gen_vertex_arrays();
    let vbo = VertexBufferObject::gen_buffers();
    let cbo = VertexBufferObject::gen_buffers();
    let ibo = VertexBufferObject::gen_buffers();

    const VERTICES: [Vec3<f32>; 4] = [
        Vec3::new(-0.5, -0.5, 0.0),   // bottom-left
        Vec3::new(0.5, -0.5, 0.0),    // bottom-right
        Vec3::new(0.0, 0.5, 0.0),     // top-center
        Vec3::new(-0.25, -0.25, 0.5), // back-center
    ];

    const COLORS: [Vec3<f32>; 4] = [
        Vec3::new(1.0, 0.0, 0.0), // red
        Vec3::new(0.0, 1.0, 0.0), // green
        Vec3::new(0.0, 0.0, 1.0), // blue
        Vec3::new(1.0, 1.0, 1.0), // white
    ];

    const INDICES: [u8; 12] = [0, 1, 2, 0, 1, 3, 1, 2, 3, 0, 2, 3];

    VertexArrayObject::bind_vertex_array(vao);

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, vbo);
    VertexBufferObject::buffer_data(gl::GL_ARRAY_BUFFER, &VERTICES, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer vertex data");

    VertexBufferObject::enable_vertex_attrib_array(Location(0));
    VertexBufferObject::vertex_attrib_pointer::<f32>(
        0,
        Vec3::<f32>::size(),
        gl::GL_FLOAT,
        gl::GL_FALSE,
        None,
    )
    .expect("Failed to set vertex attribute pointer");

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, cbo);
    VertexBufferObject::buffer_data(gl::GL_ARRAY_BUFFER, &COLORS, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer color data");
    VertexBufferObject::vertex_attrib_pointer::<f32>(
        1,
        Vec3::<f32>::size(),
        gl::GL_FLOAT,
        gl::GL_FALSE,
        None,
    )
    .expect("Failed to set color attribute pointer");

    VertexBufferObject::enable_vertex_attrib_array(Location(1));

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, VertexBufferObject::zero());

    VertexBufferObject::bind_buffer(gl::GL_ELEMENT_ARRAY_BUFFER, ibo);

    VertexBufferObject::buffer_data(gl::GL_ELEMENT_ARRAY_BUFFER, &INDICES, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer index data");

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, VertexBufferObject::zero());

    let vertex_shader =
        Shader::try_from_path(gl::GL_VERTEX_SHADER, get_shader_file_path("vertex.vert"))
            .expect("Failed to create vertex shader");

    let fragment_shader = Shader::try_from_path(
        gl::GL_FRAGMENT_SHADER,
        get_shader_file_path("fragment.frag"),
    )
    .expect("Failed to create fragment shader");

    let program = Program::from_shaders(&[vertex_shader, fragment_shader])
        .expect("Failed to create shader program");

    const MODEL_MATRIX: Mat4<f32> = Mat4::identity();

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        70.0,
        0.01,
        1000.0,
    );

    const CENTER: Vec3<f32> = Vec3::new(0.0, 0.0, 0.0);

    let model_loc = UniformLocation::try_for_program(&program, "model")
        .expect("Failed to get uniform location for model");

    let view_loc = UniformLocation::try_for_program(&program, "view")
        .expect("Failed to get uniform location for view");

    let projection_loc = UniformLocation::try_for_program(&program, "projection")
        .expect("Failed to get uniform location for projection");

    while let Ok(false) = window.should_close() {
        gl::clear_color(0.0, 0.0, 0.0, 1.0);
        gl::clear(gl::GL_COLOR_BUFFER_BIT);

        program.bind();

        model_loc.mat4f(false, MODEL_MATRIX);
        view_loc.mat4f(false, camera.view());
        projection_loc.mat4f(false, camera.projection(window.aspect_ratio()));

        gl::draw_elements(gl::GL_TRIANGLES, INDICES.len() as i32, gl::GL_UNSIGNED_BYTE);
        program.unbind();

        camera.look_at(&CENTER);

        window.swap_buffers();
        window.poll_events();
    }
    glfw::terminate();
}

fn get_shader_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join("shaders").join(filename)
}
