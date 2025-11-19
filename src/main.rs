mod geometry;
mod raytracer;
mod materials;
mod shapes;
mod texture;

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use raytracer::*;
use geometry::*;
use materials::*;
use shapes::*;
use texture::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 384;

#[derive(Clone, Copy)]
struct RenderState {
    scale_factor: usize,
    shadow_mode: raytracer::ShadowMode,
    max_depth: i32,
    ultra_mode: bool,
    checker_phase: bool,
}

fn main() {
    let mut window_options = WindowOptions::default();
    window_options.scale = minifb::Scale::X2;
    window_options.resize = true;
    window_options.title = true;
    window_options.borderless = false;
    
    let mut window = Window::new(
        "Nether Dimension Raytracer - WASD: Move, Mouse: Look, R: Rotate, Scroll: Zoom",
        WIDTH,
        HEIGHT,
        window_options,
    ).unwrap_or_else(|e| panic!("{}", e));
    let _ = window.set_position(100, 100);
    
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    // Cámara inicial posicionada para ver la escena
    let mut camera = Camera::new(
        Vec3::new(20.0, 8.0, 20.0),
        Vec3::new(0.0, 3.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let mut scene = create_nether_scene();
    build_scene_bvh(&mut scene);
    let mut frame_buffer = vec![0u32; WIDTH * HEIGHT];
    let mut prev_full_buffer = vec![0u32; WIDTH * HEIGHT];
    let mut lowres_buffer: Vec<u32> = Vec::new();
    let mut prev_lowres_buffer: Vec<u32> = Vec::new();
    let mut time = 0.0f32;
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut rotation_y = 0.0f32;
    let mut input_state = InputState::default();

    println!("=== Nether Raytracer Controls ===");
    println!("WASD/Arrow Keys: Move camera");
    println!("QE/PageUp/PageDown: Move up/down");
    println!("R: Rotate scene");
    println!("T: Toggle AUTO ambient pulse (hold J/K to scrub when OFF)");
    println!("1-4: Resolution scale, Y/U/I: Shadows None/SunOnly/Full, F/G: Max depth +/-");
    println!("N/M: Animation speed -/+");
    println!("Z: Ultra mode (checkerboard + temporal reuse)");
    println!("Mouse: Look around (drag)");
    println!("Scroll: Zoom in/out");
    println!("ESC: Exit");
    println!("====================================");

    let mut render_state = RenderState { 
        scale_factor: 3, 
        shadow_mode: raytracer::ShadowMode::None, 
        max_depth: 2, 
        ultra_mode: true, 
        checker_phase: false 
    };
    let mut day_speed: f32 = 1.0;  // Velocidad ciclo día/noche

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();
        
        let delta_time = 1.0 / 60.0;
        input_state.move_speed = 5.0 * delta_time;
        handle_input(&window, &mut camera, &mut rotation_y, &mut input_state);
        
        // Ciclo día/noche automático
        time += 0.016;
        update_nether_scene(&mut scene, time, day_speed);
        
        let render_start = Instant::now();
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) { render_state.scale_factor = 1; }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) { render_state.scale_factor = 2; }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) { render_state.scale_factor = 3; }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) { render_state.scale_factor = 4; }
        if window.is_key_pressed(Key::F, minifb::KeyRepeat::No) { render_state.max_depth = (render_state.max_depth + 1).clamp(1, 6); println!("Max depth: {}", render_state.max_depth); }
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) { render_state.max_depth = (render_state.max_depth - 1).clamp(1, 6); println!("Max depth: {}", render_state.max_depth); }
        if window.is_key_pressed(Key::Y, minifb::KeyRepeat::No) { render_state.shadow_mode = raytracer::ShadowMode::None; println!("Shadows: None"); }
        if window.is_key_pressed(Key::U, minifb::KeyRepeat::No) { render_state.shadow_mode = raytracer::ShadowMode::SunOnly; println!("Shadows: SunOnly"); }
        if window.is_key_pressed(Key::I, minifb::KeyRepeat::No) { render_state.shadow_mode = raytracer::ShadowMode::Full; println!("Shadows: Full"); }
        if window.is_key_pressed(Key::Z, minifb::KeyRepeat::No) { render_state.ultra_mode = !render_state.ultra_mode; println!("Ultra mode: {}", if render_state.ultra_mode { "ON" } else { "OFF" }); }
        if window.is_key_pressed(Key::N, minifb::KeyRepeat::No) { day_speed = (day_speed - 0.05).max(0.02); println!("Animation speed: {:.2}", day_speed); }
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) { day_speed = (day_speed + 0.05).min(1.0); println!("Animation speed: {:.2}", day_speed); }
        
        let opts = raytracer::RenderOptions { 
            shadow_mode: render_state.shadow_mode, 
            max_depth: render_state.max_depth, 
            far_simplify_distance: 20.0 
        };
        
        if render_state.ultra_mode {
            render_checkerboard_scaled(
                &scene,
                &camera,
                &mut frame_buffer,
                &mut prev_full_buffer,
                &mut lowres_buffer,
                &mut prev_lowres_buffer,
                time,
                rotation_y,
                render_state.scale_factor,
                opts,
                render_state.checker_phase,
            );
            render_state.checker_phase = !render_state.checker_phase;
        } else {
            render_parallel_scaled(&scene, &camera, &mut frame_buffer, &mut lowres_buffer, time, rotation_y, render_state.scale_factor, opts);
            if render_state.scale_factor <= 1 {
                prev_full_buffer.copy_from_slice(&frame_buffer);
            } else {
                prev_lowres_buffer = lowres_buffer.clone();
            }
        }
        let render_time = render_start.elapsed();
        
        window.update_with_buffer(&frame_buffer, WIDTH, HEIGHT).unwrap();
        
        fps_counter += 1;
        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            let fps = fps_counter as f32 / fps_timer.elapsed().as_secs_f32();
            println!("FPS: {:.1}, Render: {:.2}ms", fps, render_time.as_secs_f32() * 1000.0);
            fps_counter = 0;
            fps_timer = Instant::now();
        }
        
        let frame_time = start_time.elapsed();
        if frame_time.as_millis() < 16 {
            std::thread::sleep(std::time::Duration::from_millis(16 - frame_time.as_millis() as u64));
        }
    }
}

#[derive(Default)]
struct InputState {
    last_mouse_pos: Option<(f32, f32)>,
    move_speed: f32,
}

fn handle_input(
    window: &Window, 
    camera: &mut Camera, 
    rotation_y: &mut f32, 
    input_state: &mut InputState
) {
    let move_speed = if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
        input_state.move_speed * 3.0
    } else {
        input_state.move_speed
    };
    
    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        camera.position = camera.position + camera.get_forward() * move_speed;
    }
    if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        camera.position = camera.position - camera.get_forward() * move_speed;
    }
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        camera.position = camera.position - camera.get_right() * move_speed;
    }
    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        camera.position = camera.position + camera.get_right() * move_speed;
    }
    
    if window.is_key_down(Key::Q) || window.is_key_down(Key::PageDown) {
        camera.position.y -= move_speed;
    }
    if window.is_key_down(Key::E) || window.is_key_down(Key::PageUp) {
        camera.position.y += move_speed;
    }
    
    if window.is_key_down(Key::R) {
        *rotation_y += 0.02 * 5.0;
    }
    
    if let Some((_, scroll_y)) = window.get_scroll_wheel() {
        if scroll_y.abs() > 0.0 {
            let zoom_factor = scroll_y as f32 * 0.5_f32.max(0.05);
            camera.position = camera.position + camera.get_forward() * zoom_factor;
        }
    }
    
    if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
        if let Some((last_x, last_y)) = input_state.last_mouse_pos {
            let dx = x - last_x;
            let dy = y - last_y;
            
            if window.get_mouse_down(minifb::MouseButton::Left) {
                camera.target = camera.target + camera.get_right() * dx * 0.005;
                camera.target = camera.target - camera.get_up() * dy * 0.005;
            }
            
            if window.get_mouse_down(minifb::MouseButton::Right) {
                *rotation_y += dx * 0.02;
            }
        }
        
        input_state.last_mouse_pos = Some((x, y));
    } else {
        input_state.last_mouse_pos = None;
    }
}

fn render_parallel(scene: &Scene, camera: &Camera, buffer: &mut [u32], time: f32, rotation_y: f32, opts: raytracer::RenderOptions) {
    let frame = camera.build_frame(WIDTH, HEIGHT);
    for (y, row) in buffer.chunks_mut(WIDTH).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let ray = frame.get_ray(x as f32, y as f32);
            let color = trace_ray(&ray, scene, 0, time, rotation_y, &opts);
            *pixel = color_to_u32(color);
        }
    }
}

fn render_parallel_scaled(scene: &Scene, camera: &Camera, full_buffer: &mut [u32], lowres_buffer: &mut Vec<u32>, time: f32, rotation_y: f32, scale_factor: usize, opts: raytracer::RenderOptions) {
    if scale_factor <= 1 {
        render_parallel(scene, camera, full_buffer, time, rotation_y, opts);
        return;
    }
    let lw = WIDTH / scale_factor;
    let lh = HEIGHT / scale_factor;
    if lowres_buffer.len() != lw * lh { lowres_buffer.resize(lw * lh, 0); }
    let frame = camera.build_frame(lw, lh);
    for (y, row) in lowres_buffer.chunks_mut(lw).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let ray = frame.get_ray(x as f32, y as f32);
            let color = trace_ray(&ray, scene, 0, time, rotation_y, &opts);
            *pixel = color_to_u32(color);
        }
    }
    for y in 0..HEIGHT {
        let mut src_y = y / scale_factor;
        if src_y >= lh { src_y = lh - 1; }
        let dest_row = &mut full_buffer[y * WIDTH..(y + 1) * WIDTH];
        for x in 0..WIDTH {
            let mut src_x = x / scale_factor;
            if src_x >= lw { src_x = lw - 1; }
            dest_row[x] = lowres_buffer[src_y * lw + src_x];
        }
    }
}

fn render_checkerboard_scaled(
    scene: &Scene,
    camera: &Camera,
    full_buffer: &mut [u32],
    prev_full_buffer: &mut [u32],
    lowres_buffer: &mut Vec<u32>,
    prev_lowres_buffer: &mut Vec<u32>,
    time: f32,
    rotation_y: f32,
    scale_factor: usize,
    opts: raytracer::RenderOptions,
    phase: bool,
) {
    if scale_factor <= 1 {
        let frame = camera.build_frame(WIDTH, HEIGHT);
        for (y, row) in full_buffer.chunks_mut(WIDTH).enumerate() {
            for x in 0..WIDTH {
                let pattern = ((x + y) & 1) == 0;
                if pattern == phase {
                    let ray = frame.get_ray(x as f32, y as f32);
                    let color = trace_ray(&ray, scene, 0, time, rotation_y, &opts);
                    row[x] = color_to_u32(color);
                } else {
                    row[x] = prev_full_buffer[y * WIDTH + x];
                }
            }
        }
        prev_full_buffer.copy_from_slice(&full_buffer);
        return;
    }
    let lw = WIDTH / scale_factor;
    let lh = HEIGHT / scale_factor;
    if lowres_buffer.len() != lw * lh { lowres_buffer.resize(lw * lh, 0); }
    if prev_lowres_buffer.len() != lw * lh { prev_lowres_buffer.resize(lw * lh, 0); }
    let frame = camera.build_frame(lw, lh);
    for (y, row) in lowres_buffer.chunks_mut(lw).enumerate() {
        for x in 0..lw {
            let pattern = ((x + y) & 1) == 0;
            if pattern == phase {
                let ray = frame.get_ray(x as f32, y as f32);
                let color = trace_ray(&ray, scene, 0, time, rotation_y, &opts);
                row[x] = color_to_u32(color);
            } else {
                row[x] = prev_lowres_buffer[y * lw + x];
            }
        }
    }
    for y in 0..HEIGHT {
        let mut src_y = y / scale_factor;
        if src_y >= lh { src_y = lh - 1; }
        let dest_row = &mut full_buffer[y * WIDTH..(y + 1) * WIDTH];
        for x in 0..WIDTH {
            let mut src_x = x / scale_factor;
            if src_x >= lw { src_x = lw - 1; }
            dest_row[x] = lowres_buffer[src_y * lw + src_x];
        }
    }
    *prev_lowres_buffer = lowres_buffer.clone();
}

fn create_nether_scene() -> Scene {
    let mut scene = Scene::new();
    
    let materials = create_nether_materials();
    
    create_nether_terrain(&mut scene, &materials);
    create_bedrock_pillars(&mut scene, &materials);
    create_single_portal(&mut scene, &materials);
    create_sun(&mut scene);
    
    setup_lighting(&mut scene);
    scene.skybox = Some(create_nether_skybox());
    
    scene
}

fn create_nether_materials() -> NetherMaterials {
    NetherMaterials::new()
}

struct NetherMaterials {
    pub netherrack: Material,
    pub lava: Material,
    pub obsidian: Material,
    pub portal: Material,
}

impl NetherMaterials {
    fn new() -> Self {
        Self {
            netherrack: Material::new()
                .with_properties(Vec3::new(1.0, 0.3, 0.3), 0.0, 0.0, 0.3),
            
            lava: Material::emissive(Vec3::new(5.0, 2.5, 0.0), 20.0)
                .with_texture(Texture::animated_fire())
                .with_properties(Vec3::new(2.0, 1.2, 0.3), 0.8, 0.0, 0.3),
            
            obsidian: Material::new()
                .with_texture(Texture::minecraft_obsidian())
                .with_properties(Vec3::new(0.05, 0.02, 0.08), 0.3, 0.0, 0.0),
            
            portal: Material::new()
                .with_texture(Texture::nether_portal())
                .with_properties(Vec3::new(0.5, 0.1, 0.8), 0.1, 0.9, 0.3)
                .with_emissive(Vec3::new(0.4, 0.15, 0.6)),
        }
    }
}

fn create_nether_terrain(scene: &mut Scene, materials: &NetherMaterials) {
    for x in -10..10 {
        for z in -10..10 {
            scene.objects.push(Box::new(Cube::new(
                Vec3::new(x as f32, -1.0, z as f32),
                1.0,
                materials.netherrack.clone(),
            )));
            
            for y in -3..-1 {
                if (x + z + y) % 2 == 0 {
                    scene.objects.push(Box::new(Cube::new(
                        Vec3::new(x as f32, y as f32, z as f32),
                        1.0,
                        materials.netherrack.clone(),
                    )));
                }
            }
            
            if (x + z) % 3 == 0 {
                scene.objects.push(Box::new(Cube::new(
                    Vec3::new(x as f32, -4.0, z as f32),
                    1.0,
                    materials.netherrack.clone(),
                )));
            }
        }
    }
    
    // Pozos de lava en el suelo
    let lava_positions = [
        (-6.0, 0.0, -6.0),
        (5.0, 0.0, 5.0),
    ];
    
    for (lx, ly, lz) in lava_positions.iter() {
        for dx in 0..2 {
            for dz in 0..2 {
                scene.objects.push(Box::new(Cube::new(
                    Vec3::new(lx + dx as f32, *ly, lz + dz as f32),
                    1.0,
                    materials.lava.clone(),
                )));
            }
        }
    }
}

fn create_bedrock_pillars(scene: &mut Scene, materials: &NetherMaterials) {
    let pillar_positions = [
        (8.0_f32, 8.0_f32),
        (-7.0_f32, -7.0_f32),
        (6.0_f32, -6.0_f32),
    ];
    
    for (px, pz) in pillar_positions.iter() {
        let height = 5 + ((px.abs() + pz.abs()) as i32 % 3);
        for y in 0..height {
            scene.objects.push(Box::new(Cube::new(
                Vec3::new(*px, y as f32, *pz),
                1.0,
                materials.obsidian.clone(),
            )));
        }
        
    }
}

fn create_single_portal(scene: &mut Scene, materials: &NetherMaterials) {
    let portal_x = 0.0;
    let portal_z = 0.0;
    let width = 3;
    let height = 5;
    
    // Marco de obsidiana - pilares izquierdo y derecho
    for y in 0..height {
        scene.objects.push(Box::new(Cube::new(
            Vec3::new(portal_x - 1.0, y as f32, portal_z),
            1.0,
            materials.obsidian.clone(),
        )));
        scene.objects.push(Box::new(Cube::new(
            Vec3::new(portal_x + width as f32, y as f32, portal_z),
            1.0,
            materials.obsidian.clone(),
        )));
    }
    
    // Arriba y abajo
    for wx in 0..width {
        scene.objects.push(Box::new(Cube::new(
            Vec3::new(portal_x + wx as f32, -1.0, portal_z),
            1.0,
            materials.obsidian.clone(),
        )));
        scene.objects.push(Box::new(Cube::new(
            Vec3::new(portal_x + wx as f32, height as f32, portal_z),
            1.0,
            materials.obsidian.clone(),
        )));
    }
    
    // Efecto del portal adentro
    for y in 0..height {
        for wx in 0..width {
            scene.objects.push(Box::new(Cube::new(
                Vec3::new(portal_x + wx as f32, y as f32, portal_z),
                1.0,
                materials.portal.clone(),
            )));
        }
    }
}

fn create_sun(scene: &mut Scene) {
    // Crear esfera del sol
    let sun_material = Material::emissive(Vec3::new(1.5, 1.4, 1.0), 15.0)
        .with_properties(Vec3::new(1.0, 1.0, 0.8), 1.0, 0.0, 0.0);
    
    scene.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 30.0, 0.0),
        3.0,
        sun_material,
    )));
}

fn setup_lighting(scene: &mut Scene) {
    // Luz del sol (se mueve con el ciclo)
    scene.lights.push(Light {
        position: Vec3::new(0.0, 30.0, 0.0),
        color: Vec3::new(1.0, 0.95, 0.8),
        intensity: 1.5,
        light_type: LightType::Point,
    });
    
    // Luz del portal central
    scene.lights.push(Light {
        position: Vec3::new(1.5, 2.5, 0.0),
        color: Vec3::new(0.6, 0.2, 0.9),
        intensity: 4.0,
        light_type: LightType::Point,
    });
    
    // Luces de los pozos de lava
    let lava_lights = [
        (-6.0, 0.0, -6.0),
        (5.0, 0.0, 5.0),
    ];
    
    for (lx, ly, lz) in lava_lights.iter() {
        scene.lights.push(Light {
            position: Vec3::new(*lx, *ly + 1.0, *lz),
            color: Vec3::new(1.0, 0.4, 0.1),
            intensity: 3.5,
            light_type: LightType::Point,
        });
    }
    
}

fn create_nether_skybox() -> Skybox {
    Skybox::textured(
        Vec3::new(0.53, 0.81, 0.92),  // Cielo de día
        Vec3::new(1.0, 0.6, 0.3),     // Amanecer/atardecer
        Vec3::new(0.05, 0.05, 0.15),  // Cielo nocturno
        Vec3::new(0.3, 0.5, 0.7)      // Horizonte
    )
}

fn update_nether_scene(scene: &mut Scene, time: f32, speed: f32) {
    // Mover el sol en trayectoria circular
    let angle = time * speed;
    let sun_radius = 40.0;
    let sun_x = angle.cos() * sun_radius;
    let sun_y = angle.sin() * sun_radius;
    
    // Actualizar posición de la luz del sol
    if let Some(sun_light) = scene.lights.get_mut(0) {
        sun_light.position = Vec3::new(sun_x, sun_y.max(5.0), 0.0);
        
        // Ajustar intensidad según altura del sol
        let height_factor = (sun_y / sun_radius).max(0.0);
        sun_light.intensity = 0.3 + height_factor * 1.5;
        
        // Cambios de color: cálido de día, frío de noche
        if sun_y > 0.0 {
            sun_light.color = Vec3::new(1.0, 0.95, 0.8);  // Día
        } else {
            sun_light.color = Vec3::new(0.4, 0.5, 0.7);   // Noche (luna)
        }
    }
    
    if let Some(skybox) = &mut scene.skybox {
        skybox.update_time_of_day_with_speed(time, speed);
    }

    // Luz ambiental cambia con el ciclo
    let ambient_strength = ((time * speed).sin() * 0.5 + 0.5).max(0.1);
    scene.ambient_light = Vec3::new(0.3, 0.3, 0.4) * ambient_strength;
}

fn color_to_u32(color: Vec3) -> u32 {
    let gamma = 1.0 / 2.2;
    let r = (color.x.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.powf(gamma).clamp(0.0, 1.0) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
