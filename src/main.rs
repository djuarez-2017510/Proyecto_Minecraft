mod geometry;
mod raytracer;
mod materials;
mod shapes;
mod texture;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::time::Instant;

use geometry::*;
use raytracer::*;
use materials::*;
use shapes::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 384;

#[derive(Default)]
struct InputState {
    last_mouse_pos: Option<(f32, f32)>,
    move_speed: f32,
}

fn main() {
    let mut window_options = WindowOptions::default();
    window_options.resize = true;
    window_options.scale = minifb::Scale::X2;

    let mut window = Window::new(
        "Raytracer - test scene with shading options",
        WIDTH,
        HEIGHT,
        window_options,
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    let mut frame_buffer = vec![0u32; WIDTH * HEIGHT];

    let mut camera = Camera::new(
        Vec3::new(6.0, 3.0, 10.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let scene = create_test_scene_v2();
    let mut input_state = InputState::default();
    let mut rotation_y = 0.0f32;

    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();

    let mut shadow_mode = ShadowMode::Full;
    let mut max_depth: i32 = 3;

    println!("WASD/Arrows: move, mouse drag: look, ESC: exit");
    println!("Y/U/I: change shadow mode (None/SunOnly/Full)");
    println!("F/G: increase/decrease max depth");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        let delta_time = 1.0 / 60.0;
        input_state.move_speed = 5.0 * delta_time;

        handle_input(&window, &mut camera, &mut rotation_y, &mut input_state);

        if window.is_key_pressed(Key::Y, minifb::KeyRepeat::No) {
            shadow_mode = ShadowMode::None;
            println!("Shadows: None");
        }
        if window.is_key_pressed(Key::U, minifb::KeyRepeat::No) {
            shadow_mode = ShadowMode::SunOnly;
            println!("Shadows: SunOnly");
        }
        if window.is_key_pressed(Key::I, minifb::KeyRepeat::No) {
            shadow_mode = ShadowMode::Full;
            println!("Shadows: Full");
        }

        if window.is_key_pressed(Key::F, minifb::KeyRepeat::No) {
            max_depth = (max_depth + 1).clamp(1, 6);
            println!("Max depth: {}", max_depth);
        }
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            max_depth = (max_depth - 1).clamp(1, 6);
            println!("Max depth: {}", max_depth);
        }

        let opts = RenderOptions {
            shadow_mode,
            max_depth,
            far_simplify_distance: 1000.0,
        };

        render_parallel(&scene, &camera, &mut frame_buffer, 0.0, rotation_y, opts);

        window
            .update_with_buffer(&frame_buffer, WIDTH, HEIGHT)
            .unwrap();

        fps_counter += 1;
        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            let fps = fps_counter as f32 / fps_timer.elapsed().as_secs_f32();
            println!("FPS: {:.1}", fps);
            fps_counter = 0;
            fps_timer = Instant::now();
        }

        let frame_time = frame_start.elapsed();
        if frame_time.as_millis() < 16 {
            std::thread::sleep(std::time::Duration::from_millis(
                16 - frame_time.as_millis() as u64,
            ));
        }
    }
}

fn create_test_scene_v2() -> Scene {
    let mut scene = Scene::new();

    let red = Material::new()
        .with_properties(Vec3::new(0.9, 0.2, 0.2), 0.3, 0.0, 0.2);
    let blue = Material::new()
        .with_properties(Vec3::new(0.2, 0.2, 0.9), 0.4, 0.0, 0.4);
    let glass = Material::new()
        .with_properties(Vec3::new(0.9, 0.9, 1.0), 0.1, 0.8, 0.1);
    let floor_mat = Material::new()
        .with_properties(Vec3::new(0.8, 0.8, 0.8), 0.1, 0.0, 0.0);

    // Esfera difusa roja
    scene.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        red,
    )));

    // Esfera “cristal”
    scene.objects.push(Box::new(Sphere::new(
        Vec3::new(-2.5, 1.0, -1.0),
        1.0,
        glass,
    )));

    // Cubo azul reflectivo
    scene.objects.push(Box::new(Cube::new(
        Vec3::new(2.5, 0.5, -1.0),
        1.0,
        blue,
    )));

    // Suelo
    scene.objects.push(Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        floor_mat,
    )));

    // Luz principal
    scene.lights.push(Light::point(
        Vec3::new(5.0, 8.0, 5.0),
        Vec3::new(1.0, 1.0, 1.0),
        3.0,
    ));

    // Luz secundaria
    scene.lights.push(Light::point(
        Vec3::new(-5.0, 6.0, -4.0),
        Vec3::new(0.8, 0.8, 1.0),
        2.0,
    ));

    scene
}

fn handle_input(
    window: &Window,
    camera: &mut Camera,
    rotation_y: &mut f32,
    input_state: &mut InputState,
) {
    let move_speed = if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
        input_state.move_speed * 3.0
    } else {
        input_state.move_speed
    };

    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        camera.position = camera.position + camera.get_forward() * move_speed;
        camera.target = camera.target + camera.get_forward() * move_speed;
    }
    if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        camera.position = camera.position - camera.get_forward() * move_speed;
        camera.target = camera.target - camera.get_forward() * move_speed;
    }
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        camera.position = camera.position - camera.get_right() * move_speed;
        camera.target = camera.target - camera.get_right() * move_speed;
    }
    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        camera.position = camera.position + camera.get_right() * move_speed;
        camera.target = camera.target + camera.get_right() * move_speed;
    }

    if window.is_key_down(Key::Q) || window.is_key_down(Key::PageDown) {
        camera.position.y -= move_speed;
        camera.target.y -= move_speed;
    }
    if window.is_key_down(Key::E) || window.is_key_down(Key::PageUp) {
        camera.position.y += move_speed;
        camera.target.y += move_speed;
    }

    if window.is_key_down(Key::R) {
        *rotation_y += 0.02 * 5.0;
    }

    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
        if let Some((last_x, last_y)) = input_state.last_mouse_pos {
            let dx = x - last_x;
            let dy = y - last_y;

            if window.get_mouse_down(MouseButton::Left) {
                camera.target =
                    camera.target + camera.get_right() * dx * 0.005 - camera.get_up() * dy * 0.005;
            }
        }
        input_state.last_mouse_pos = Some((x, y));
    } else {
        input_state.last_mouse_pos = None;
    }
}

fn render_parallel(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [u32],
    time: f32,
    rotation_y: f32,
    opts: RenderOptions,
) {
    let frame = camera.build_frame(WIDTH, HEIGHT);
    for (y, row) in buffer.chunks_mut(WIDTH).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let ray = frame.get_ray(x as f32, y as f32);
            let color = trace_ray(&ray, scene, 0, time, rotation_y, &opts);
            *pixel = color_to_u32(color);
        }
    }
}

fn color_to_u32(color: Vec3) -> u32 {
    let gamma = 1.0 / 2.2;
    let r = (color.x.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
