use std::{
    env,
    f32::consts::PI,
    path::{Path, PathBuf},
    process::ExitCode,
};

use rusty_gl::{
    UniformWrapper,
    framework::{
        camera::Camera,
        color::rgb::ColorRGB,
        material::material_textures::MaterialTextureType,
        mesh::{self, normalize::NormalizeOptions},
    },
};

use gmath::{mat4::Mat4, vec3::Vec3};

use gl_sys::{
    self,
    program::Program,
    shader::{Shader, ShaderType},
    vao::VertexArrayObject,
};

use glfw_sys::{
    self,
    input::{KeyEvent, keycode::Keycode, modifier::Modifier},
    timer::Timer,
    window::Window,
};

const BACKGROUND: ColorRGB = ColorRGB::new(0.0, 0.1, 0.333);

fn usage() -> ExitCode {
    eprintln!("Usage: rusty_gl <path_to_obj_file>");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let entrypoint = match env::args().nth(1) {
        Some(s) => s,
        None => return usage(),
    };

    glfw_sys::init().expect("Failed to initialize GLFW");

    glfw_sys::window_hint(glfw_sys::bindings::GLFW_CONTEXT_VERSION_MAJOR, 4)
        .expect("Failed to set window hint");
    glfw_sys::window_hint(glfw_sys::bindings::GLFW_CONTEXT_VERSION_MINOR, 1)
        .expect("Failed to set window hint");
    glfw_sys::window_hint(
        glfw_sys::bindings::GLFW_OPENGL_PROFILE,
        glfw_sys::bindings::GLFW_OPENGL_CORE_PROFILE,
    )
    .expect("Failed to set window hint");
    glfw_sys::window_hint(
        glfw_sys::bindings::GLFW_OPENGL_FORWARD_COMPAT,
        glfw_sys::bindings::GLFW_TRUE,
    )
    .expect("Failed to set window hint");

    let mut window = Window::try_new(1920, 1080, "Rust").expect("Failed to create GLFW window");

    let scene =
        mesh::load_mesh(entrypoint, NormalizeOptions::Scale(200)).expect("Failed to load model");

    let vertex_shader =
        Shader::try_from_path(ShaderType::Vertex, get_shader_file_path("vertex.vert"))
            .expect("Failed to create vertex shader");

    let fragment_shader =
        Shader::try_from_path(ShaderType::Fragment, get_shader_file_path("fragment.frag"))
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
    const POINTLIGHT_COLOR: ColorRGB = ColorRGB::new(1.0, 0.58, 0.16);
    const POINTLIGHT_INTENSITY: f32 = 1.5;

    const DIRLIGHT_DIR: Vec3<f32> = Vec3::new(0.25, -0.93, -0.25);
    const DIRLIGHT_COLOR: ColorRGB = ColorRGB::new(1.0, 0.97, 0.8);
    const DIRLIGHT_INTENSITY: f32 = 1.5;

    const COLOR_EXP: ColorRGB = ColorRGB::new(2.2, 2.2, 2.2);

    gl_sys::enable(gl_sys::bindings::GL_DEPTH_TEST);
    let mut timer = Timer::<144>::new();
    while let Ok(false) = window.should_close() {
        timer.start();

        window.poll_events();

        const TURN_ANGLE: f32 = PI / 2.4;
        const MOVE_DISTANCE: f32 = 1.2;

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
            gl_sys::clear_color(BACKGROUND.r(), BACKGROUND.g(), BACKGROUND.g(), 0.0);
            gl_sys::clear(
                gl_sys::bindings::GL_COLOR_BUFFER_BIT | gl_sys::bindings::GL_DEPTH_BUFFER_BIT,
            );

            let aspect_ratio = window.aspect_ratio();
            for element in &scene {
                element
                    .bind(&program, &camera, aspect_ratio)
                    .expect("Failed to bind element");

                program
                    .uniform("pointlight_pos", UniformWrapper(&POINTLIGHT_POS))
                    .expect("Failed to set pointlight position");
                program
                    .uniform(
                        "pointlight_color",
                        UniformWrapper(&Vec3::pow(POINTLIGHT_COLOR, COLOR_EXP)),
                    )
                    .expect("Failed to set pointlight color");
                program
                    .uniform("pointlight_intensity", POINTLIGHT_INTENSITY)
                    .expect("Failed to set pointlight intensity");
                program
                    .uniform("dirlight_dir", UniformWrapper(&DIRLIGHT_DIR))
                    .expect("Failed to set dirlight direction");
                program
                    .uniform(
                        "dirlight_color",
                        UniformWrapper(&Vec3::pow(DIRLIGHT_COLOR, COLOR_EXP)),
                    )
                    .expect("Failed to set dirlight color");
                program
                    .uniform("dirlight_intensity", DIRLIGHT_INTENSITY)
                    .expect("Failed to set dirlight intensity");
                program
                    .uniform("camera_pos", UniformWrapper(camera.position()))
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
    glfw_sys::terminate();
    ExitCode::SUCCESS
}

fn get_shader_file_path<S: AsRef<str>>(filename: S) -> PathBuf {
    get_asset(["shaders", filename.as_ref()])
}

fn get_asset<P, I>(paths: I) -> PathBuf
where
    P: AsRef<Path>,
    I: IntoIterator<Item = P>,
{
    let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pb.extend(paths);
    pb
}
