mod geometry;
mod raytracer;
mod materials;
mod shapes;
mod texture;

use minifb::{Key, Window, WindowOptions};

use geometry::*;
use raytracer::*;
use materials::*;
use shapes::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 384;

fn main() {
    let mut window_options = WindowOptions::default();
    window_options.resize = true;
    window_options.scale = minifb::Scale::X2;

    let mut window = Window::new(
        "Nether Raytracer - first image",
        WIDTH,
        HEIGHT,
        window_options,
    ).unwrap_or_else(|e| panic!("{}", e));

    let mut frame_buffer = vec![0u32; WIDTH * HEIGHT];

    // Cámara fija, simple
    let camera = Camera::new(
        Vec3::new(6.0, 3.0, 10.0), // posición
        Vec3::new(0.0, 1.0, 0.0),  // mira hacia la esfera
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let scene = create_test_scene();

    let opts = raytracer::RenderOptions {
        shadow_mode: raytracer::ShadowMode::None,
        max_depth: 2,
        far_simplify_distance: 1000.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render_frame(&scene, &camera, &mut frame_buffer, opts);
        window
            .update_with_buffer(&frame_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    let sphere_mat = Material::new()
        .with_properties(Vec3::new(0.8, 0.3, 0.3), 0.2, 0.0, 0.1);

    let floor_mat = Material::new()
        .with_properties(Vec3::new(0.8, 0.8, 0.8), 0.1, 0.0, 0.0);

    // Esfera central
    scene.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        sphere_mat,
    )));

    // Plano como suelo
    scene
        .objects
        .push(Box::new(Plane::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            floor_mat,
        )));

    // Una luz puntual simple
    scene.lights.push(Light::point(
        Vec3::new(5.0, 8.0, 5.0),
        Vec3::new(1.0, 1.0, 1.0),
        3.0,
    ));

    scene
}

fn render_frame(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [u32],
    opts: raytracer::RenderOptions,
) {
    let frame = camera.build_frame(WIDTH, HEIGHT);

    for (y, row) in buffer.chunks_mut(WIDTH).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let ray = frame.get_ray(x as f32, y as f32);
            let color = trace_ray(&ray, scene, 0, 0.0, 0.0, &opts);
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
