use std::{f32::consts::PI, path::PathBuf, process::ExitCode};

use rusty_gl::{
    framework::{
        camera::Camera, material::material_textures::MaterialTextureType, mesh,
        mesh::normalize::NormalizeOptions, timer::Timer,
    },
    gl,
    gl::{program::Program, shader::Shader, vao::VertexArrayObject},
    glfw,
    glfw::{
        input::{KeyEvent, keycode::Keycode, modifier::Modifier},
        window::Window,
    },
    math::{mat4::Mat4, vec3::Vec3},
};

const BACKGROUND: Vec3<f32> = Vec3::rgb(0.0, 0.1, 0.333);

fn usage() -> ExitCode {
    eprintln!("Usage: rusty_gl <path_to_obj_file>");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let entrypoint = match std::env::args().nth(1) {
        Some(s) => s,
        None => return usage(),
    };

    glfw::init().expect("Failed to initialize GLFW");

    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MAJOR, 3).expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_CONTEXT_VERSION_MINOR, 3).expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_OPENGL_PROFILE, glfw::GLFW_OPENGL_CORE_PROFILE)
        .expect("Failed to set window hint");
    glfw::window_hint(glfw::GLFW_OPENGL_FORWARD_COMPAT, glfw::GLFW_TRUE)
        .expect("Failed to set window hint");

    let mut window = Window::try_new(640, 320, "Rust").expect("Failed to create GLFW window");

    let scene = mesh::load_mesh(
        get_model_file_path(&entrypoint),
        NormalizeOptions::Scale(200),
    )
    .expect("Failed to load model");

    let vertex_shader =
        Shader::try_from_path(gl::GL_VERTEX_SHADER, get_shader_file_path("vertex.vert"))
            .expect("Failed to create vertex shader");

    let fragment_shader = Shader::try_from_path(
        gl::GL_FRAGMENT_SHADER,
        get_shader_file_path("fragment.frag"),
    )
    .expect("Failed to create fragment shader");

    let vao = VertexArrayObject::gen_vertex_arrays();
    VertexArrayObject::bind_vertex_array(&vao);
    let program = Program::from_shaders(&[vertex_shader, fragment_shader])
        .expect("Failed to create shader program");
    VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
    const MODEL_MATRIX: Mat4<f32> = Mat4::identity();

    let mut camera = Camera::with_defaults(
        Vec3::new(0.5, 2.00, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    const POINTLIGHT_POS: Vec3<f32> = Vec3::new(75.0, 100.0, -50.0);
    const POINTLIGHT_COLOR: Vec3<f32> = Vec3::rgb(1.0, 0.58, 0.16);
    const POINTLIGHT_INTENSITY: f32 = 1.5;

    const DIRLIGHT_DIR: Vec3<f32> = Vec3::new(0.25, -0.93, -0.25);
    const DIRLIGHT_COLOR: Vec3<f32> = Vec3::rgb(1.0, 0.97, 0.8);
    const DIRLIGHT_INTENSITY: f32 = 1.5;

    gl::enable(gl::GL_DEPTH_TEST);
    let mut timer: Timer<60> = Timer::new();
    while let Ok(false) = window.should_close() {
        timer.start();

        window.poll_events();

        const TURN_ANGLE: f32 = PI / 2.0;
        const MOVE_DISTANCE: f32 = 0.8;

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

            let aspect_ratio = window.aspect_ratio();
            for element in &scene {
                element
                    .bind(&program, &camera, aspect_ratio)
                    .expect("Failed to bind element");

                program
                    .uniform("pointlight_pos", &POINTLIGHT_POS)
                    .expect("Failed to set pointlight position");
                program
                    .uniform(
                        "pointlight_color",
                        &Vec3::pow(POINTLIGHT_COLOR, Vec3::new(2.2, 2.2, 2.2)),
                    )
                    .expect("Failed to set pointlight color");
                program
                    .uniform("pointlight_intensity", POINTLIGHT_INTENSITY)
                    .expect("Failed to set pointlight intensity");
                program
                    .uniform("dirlight_dir", &DIRLIGHT_DIR)
                    .expect("Failed to set dirlight direction");
                program
                    .uniform(
                        "dirlight_color",
                        &Vec3::pow(DIRLIGHT_COLOR, Vec3::new(2.2, 2.2, 2.2)),
                    )
                    .expect("Failed to set dirlight color");
                program
                    .uniform("dirlight_intensity", DIRLIGHT_INTENSITY)
                    .expect("Failed to set dirlight intensity");
                program
                    .uniform("camera_pos", camera.position())
                    .expect("Failed to set camera position");
                program
                    .uniform(
                        "has_alphamap",
                        if element
                            .material
                            .textures
                            .has_texture(MaterialTextureType::AlphaMap)
                        {
                            1
                        } else {
                            0
                        },
                    )
                    .expect("Failed to se has_alphamap");
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
    ExitCode::SUCCESS
}

#[allow(dead_code)]
fn get_model_file_path(filename: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // project root
    PathBuf::from(manifest_dir).join(filename)
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
