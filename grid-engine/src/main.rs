mod math;
mod mesh;
mod camera;
mod screen;
mod renderer;

use minifb::{Key, Window, WindowOptions};
use math::Vec3;
use mesh::Mesh;
use camera::Camera;
use screen::{WIDTH, HEIGHT};
use renderer::render;

fn main() {
    let mut window = Window::new(
        "Minifb 3D Structured",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let mesh = Mesh::cube();
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);
    let mut buffer = vec![0x202020FF; WIDTH * HEIGHT];
    let mut angle_x = 0.0_f32;
    let mut angle_y = 0.0_f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            camera.pos.x += camera.yaw.sin() * 0.1;
            camera.pos.z += camera.yaw.cos() * 0.1;
        }
        if window.is_key_down(Key::S) {
            camera.pos.x -= camera.yaw.sin() * 0.1;
            camera.pos.z -= camera.yaw.cos() * 0.1;
        }
        if window.is_key_down(Key::A) {
            camera.pos.x -= (camera.yaw + std::f32::consts::PI/2.0).sin() * 0.1;
            camera.pos.z -= (camera.yaw + std::f32::consts::PI/2.0).cos() * 0.1;
        }
        if window.is_key_down(Key::D) {
            camera.pos.x += (camera.yaw + std::f32::consts::PI/2.0).sin() * 0.1;
            camera.pos.z += (camera.yaw + std::f32::consts::PI/2.0).cos() * 0.1;
        }
        if window.is_key_down(Key::Q) { camera.pos.y -= 0.1; }
        if window.is_key_down(Key::E) { camera.pos.y += 0.1; }

        if window.is_key_down(Key::Left)  { camera.yaw   -= 0.03; }
        if window.is_key_down(Key::Right) { camera.yaw   += 0.03; }
        if window.is_key_down(Key::Up)    { camera.pitch += 0.03; }
        if window.is_key_down(Key::Down)  { camera.pitch -= 0.03; }

        angle_x += 0.01;
        angle_y += 0.013;

        buffer.fill(0x202020FF);
        render(&mesh, &camera, (angle_x, angle_y), &mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
