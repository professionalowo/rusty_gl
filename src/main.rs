use std::f32::consts::PI;
use std::path::PathBuf;

use rusty_gl::framework::camera::Camera;
use rusty_gl::framework::mesh;
use rusty_gl::framework::texture::Texture2D;
use rusty_gl::framework::timer::Timer;
use rusty_gl::gl;
use rusty_gl::gl::program::Program;
use rusty_gl::gl::shader::Shader;
use rusty_gl::gl::uniform::UniformLocation;
use rusty_gl::gl::vao::VertexArrayObject;
use rusty_gl::gl::vbo::{Location, VertexBufferObject};
use rusty_gl::glfw;
use rusty_gl::glfw::input::KeyEvent;
use rusty_gl::glfw::input::keycode::Keycode;
use rusty_gl::glfw::input::modifier::Modifier;
use rusty_gl::glfw::window::Window;
use rusty_gl::math::mat4::Mat4;
use rusty_gl::math::vec3::Vec3;

const BACKGROUND: Vec3<f32> = Vec3::rgb(0.12, 0.12, 0.12);

fn main() {
    let mut args = std::env::args();
    let entrypoint = args.nth(1).expect("No object specified");

    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MAJOR, 3).expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MINOR, 3).expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_OPENGL_PROFILE, glfw::GLFW_OPENGL_CORE_PROFILE)
        .expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_OPENGL_FORWARD_COMPAT, glfw::GLFW_TRUE)
        .expect("Failed to set window hint");

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
        Vec3::rgb(1.0, 0.0, 0.0), // red
        Vec3::rgb(0.0, 1.0, 0.0), // green
        Vec3::rgb(0.0, 0.0, 1.0), // blue
        Vec3::rgb(1.0, 1.0, 1.0), // white
        Vec3::rgb(1.0, 1.0, 0.0), // yellow
    ];

    const INDICES: [u8; 18] = [
        0, 1, 2, //
        0, 2, 3, //
        2, 3, 4, //
        3, 0, 4, //
        1, 2, 4, //
        0, 1, 4,
    ];

    VertexArrayObject::bind_vertex_array(&vao);

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &vbo);
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

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &cbo);
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

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &VertexBufferObject::zero());

    VertexBufferObject::bind_buffer(gl::GL_ELEMENT_ARRAY_BUFFER, &ibo);

    VertexBufferObject::buffer_data(gl::GL_ELEMENT_ARRAY_BUFFER, &INDICES, gl::GL_STATIC_DRAW)
        .expect("Failed to buffer index data");

    VertexBufferObject::bind_buffer(gl::GL_ARRAY_BUFFER, &VertexBufferObject::zero());

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
        Vec3::new(-516.0, 584.0, -138.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    const POINTLIGHT_POS: Vec3<f32> = Vec3::new(1.0, 5.0, 1.0);
    const POINTLIGHT_COLOR: Vec3<f32> = Vec3::rgb(1.0, 1.0, 1.0);
    const POINTLIGHT_INTENSITY: f32 = 0.2;

    gl::enable(gl::GL_DEPTH_TEST);

    let scene = mesh::load_mesh(get_model_file_path(&entrypoint)).expect("Failed to load model");

    let mut timer: Timer<60> = Timer::new();

    while let Ok(false) = window.should_close() {
        timer.start();

        window.poll_events();

        const TURN_ANGLE: f32 = PI / 2.0;
        const MOVE_DISTANCE: f32 = 10.0;

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
                    Keycode::W => camera.rotate_horizontal(-TURN_ANGLE),
                    Keycode::A => camera.rotate_vertical(TURN_ANGLE),
                    Keycode::S => camera.rotate_horizontal(TURN_ANGLE),
                    Keycode::D => camera.rotate_vertical(-TURN_ANGLE),
                    _ => (),
                },
                KeyEvent { keycode, .. } => match keycode {
                    Keycode::W => camera.move_forward(MOVE_DISTANCE),
                    Keycode::A => camera.move_left(MOVE_DISTANCE),
                    Keycode::S => camera.move_backward(MOVE_DISTANCE),
                    Keycode::D => camera.move_right(MOVE_DISTANCE),
                    _ => (),
                },
            }
        }
        if timer.should_render() {
            gl::clear_color_vec(&BACKGROUND);
            gl::clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);

            for element in scene.iter().as_slice() {
                element
                    .bind(&program, &camera, window.aspect_ratio())
                    .expect("Failed to bind element");

                program
                    .uniform("pointlight_pos", &POINTLIGHT_POS)
                    .expect("Failed to set pointlight position");
                program
                    .uniform("pointlight_color", &POINTLIGHT_COLOR)
                    .expect("Failed to set pointlight color");
                program
                    .uniform("pointlight_intensity", POINTLIGHT_INTENSITY)
                    .expect("Failed to set pointlight intensity");
                program
                    .uniform("camera_pos", camera.position())
                    .expect("Failed to set camera position");
                element
                    .draw(&program, &MODEL_MATRIX)
                    .expect("Failed to draw element");
                element.unbind(&program);
            }

            window.swap_buffers();
            timer.rendered();
        }
    }
    glfw::terminate();
}

#[allow(dead_code)]
fn get_model_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir)
        .join("render-data")
        .join(filename)
}

#[allow(dead_code)]
fn get_shader_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join("shaders").join(filename)
}

#[allow(dead_code)]
fn get_texture_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join("textures").join(filename)
}
