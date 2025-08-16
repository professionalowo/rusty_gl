use std::f32::consts::PI;
use std::path::PathBuf;

use open_gl::framework::camera::Camera;
use open_gl::gl;
use open_gl::gl::program::Program;
use open_gl::gl::shader::Shader;
use open_gl::gl::uniform::UniformLocation;
use open_gl::gl::vao::VertexArrayObject;
use open_gl::gl::vbo::{Location, VertexBufferObject};
use open_gl::glfw;
use open_gl::glfw::input::KeyEvent;
use open_gl::glfw::input::keycode::Keycode;
use open_gl::glfw::input::modifier::Modifier;
use open_gl::glfw::window::Window;
use open_gl::math::mat4::Mat4;
use open_gl::math::vec3::Vec3;

fn main() {
    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfw::window_hint(gl::GLFW_CONTEXT_VERSION_MINOR, 3);
    glfw::window_hint(gl::GLFW_OPENGL_PROFILE, gl::GLFW_OPENGL_CORE_PROFILE);
    glfw::window_hint(gl::GLFW_OPENGL_FORWARD_COMPAT, gl::GLFW_TRUE);

    let mut window = Window::try_new(640, 320, "Rust").expect("Failed to create GLFW window");
    let vao = VertexArrayObject::gen_vertex_arrays();
    let vbo = VertexBufferObject::gen_buffers();
    let cbo = VertexBufferObject::gen_buffers();
    let ibo = VertexBufferObject::gen_buffers();

    const VERTICES: [Vec3<f32>; 5] = [
        Vec3::new(-0.5, -0.5, 0.5),  // 0
        Vec3::new(0.5, -0.5, 0.5),   // 1
        Vec3::new(0.5, -0.5, -0.5),  // 2
        Vec3::new(-0.5, -0.5, -0.5), // 3
        Vec3::new(0.0, 0.5, 0.0),    // 4
    ];

    const COLORS: [Vec3<f32>; 5] = [
        Vec3::new(1.0, 0.0, 0.0), // red
        Vec3::new(0.0, 1.0, 0.0), // green
        Vec3::new(0.0, 0.0, 1.0), // blue
        Vec3::new(1.0, 1.0, 1.0), // white
        Vec3::new(1.0, 1.0, 0.0), // yellow
    ];

    const INDICES: [u8; 18] = [
        0, 1, 2, //
        0, 2, 3, //
        2, 3, 4, //
        3, 0, 4, //
        1, 2, 4, //
        0, 1, 4,
    ];

    VertexArrayObject::bind_vertex_array(vao);

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, vbo);
    VertexBufferObject::buffer_data(gl::GL_ARRAY_BUFFER, &VERTICES, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer vertex data");

    let vertex_loc = Location(0);
    VertexBufferObject::enable_vertex_attrib_array(&vertex_loc);
    VertexBufferObject::vertex_attrib_pointer::<f32>(
        &vertex_loc,
        Vec3::<f32>::size(),
        gl::GL_FLOAT,
        gl::GL_FALSE,
        None,
    )
    .expect("Failed to set vertex attribute pointer");

    let color_loc = Location(1);
    VertexBufferObject::enable_vertex_attrib_array(&color_loc);

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, cbo);
    VertexBufferObject::buffer_data(gl::GL_ARRAY_BUFFER, &COLORS, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer color data");
    VertexBufferObject::vertex_attrib_pointer::<f32>(
        &color_loc,
        Vec3::<f32>::size(),
        gl::GL_FLOAT,
        gl::GL_FALSE,
        None,
    )
    .expect("Failed to set color attribute pointer");

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

    let mut camera = Camera::with_defaults(
        Vec3::new(0.0, 0.0, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let model_loc = UniformLocation::try_for_program(&program, "model")
        .expect("Failed to get uniform location for model");

    let view_loc = UniformLocation::try_for_program(&program, "view")
        .expect("Failed to get uniform location for view");

    let projection_loc = UniformLocation::try_for_program(&program, "projection")
        .expect("Failed to get uniform location for projection");

    gl::enable(gl::GL_DEPTH_TEST);

    while let Ok(false) = window.should_close() {
        gl::clear_color(0.0, 0.0, 0.0, 1.0);
        gl::clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);

        program.bind();

        model_loc.mat4f(false, MODEL_MATRIX);
        view_loc.mat4f(false, camera.view());
        projection_loc.mat4f(false, camera.projection(window.aspect_ratio()));

        gl::draw_elements(gl::GL_TRIANGLES, INDICES.len() as i32, gl::GL_UNSIGNED_BYTE);
        program.unbind();

        window.swap_buffers();
        window.poll_events();

        const TURN_ANGLE: f32 = PI / 2.0;

        if let Some(event) = window.pump_event() {
            match event {
                KeyEvent {
                    keycode: Keycode::Escape,
                    ..
                } => window.set_should_close(true),
                KeyEvent {
                    modifier: Modifier::Shift,
                    keycode,
                    ..
                } => match keycode {
                    Keycode::W => camera.rotate(TURN_ANGLE, &camera.horizontal_rotation_axis()),
                    Keycode::A => camera.rotate(TURN_ANGLE, &camera.up().clone()),
                    Keycode::S => camera.rotate(-TURN_ANGLE, &camera.horizontal_rotation_axis()),
                    Keycode::D => camera.rotate(-TURN_ANGLE, &camera.up().clone()),
                    _ => (),
                },
                KeyEvent { keycode, .. } => match keycode {
                    Keycode::W => camera.move_forward(0.1),
                    Keycode::A => camera.move_left(0.1),
                    Keycode::S => camera.move_backward(0.1),
                    Keycode::D => camera.move_right(0.1),
                    _ => (),
                },
            }
        }
    }
    glfw::terminate();
}

fn get_shader_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join("shaders").join(filename)
}