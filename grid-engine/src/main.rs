use minifb::{Key, Window, WindowOptions};
use rust3d::{Map, Point3};
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Simple camera with position and orientation
struct Camera {
    pub pos: Point3,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Camera {
    pub fn new(pos: Point3) -> Self {
        Camera { pos, yaw: 0.0, pitch: -0.4, speed: 5.0, rotation_speed: 1.5 }
    }
    /// Transforms world point to camera space
    pub fn world_to_camera(&self, p: Point3) -> Point3 {
        let mut x = p.x - self.pos.x;
        let mut y = p.y - self.pos.y;
        let mut z = p.z - self.pos.z;
        // yaw
        let (sy, cy) = self.yaw.sin_cos();
        let tx = cy * x + sy * z;
        let tz = -sy * x + cy * z;
        x = tx; z = tz;
        // pitch
        let (sp, cp) = self.pitch.sin_cos();
        let ty = cp * y - sp * z;
        let tz2 = sp * y + cp * z;
        y = ty; z = tz2;
        Point3::new(x, y, z)
    }
}

fn project(p: Point3, width: usize, height: usize) -> Option<(i32,i32)> {
    if p.z <= 0.1 { return None; }
    let fov = 90.0_f32.to_radians();
    let aspect = width as f32 / height as f32;
    let px = (p.x / p.z) * (1.0/(fov/2.0).tan()) * aspect;
    let py = (p.y / p.z) * (1.0/(fov/2.0).tan());
    let x = ((px + 1.0)*(width as f32/2.0)) as i32;
    let y = ((-py + 1.0)*(height as f32/2.0)) as i32;
    Some((x,y))
}

fn draw_line(buf: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx=(x1-x0).abs(); let dy=-(y1-y0).abs();
    let sx=if x0<x1 {1} else {-1}; let sy=if y0<y1 {1} else {-1};
    let mut err=dx+dy; let(mut x, mut y)=(x0,y0);
    while x!=x1||y!=y1 {
        if (0..WIDTH as i32).contains(&x) && (0..HEIGHT as i32).contains(&y) {
            buf[y as usize*WIDTH + x as usize] = color;
        }
        let e2=2*err;
        if e2>=dy { err+=dy; x+=sx; }
        if e2<=dx { err+=dx; y+=sy; }
    }
}

fn fill_triangle(buf: &mut [u32], a:(i32,i32), b:(i32,i32), c:(i32,i32), color:u32) {
    let min_x = a.0.min(b.0).min(c.0).max(0).min((WIDTH-1) as i32);
    let max_x = a.0.max(b.0).max(c.0).max(0).min((WIDTH-1) as i32);
    let min_y = a.1.min(b.1).min(c.1).max(0).min((HEIGHT-1) as i32);
    let max_y = a.1.max(b.1).max(c.1).max(0).min((HEIGHT-1) as i32);
    let v0=(b.0-a.0,b.1-a.1);
    let v1=(c.0-a.0,c.1-a.1);
    let d00=(v0.0*v0.0+v0.1*v0.1) as f32;
    let d01=(v0.0*v1.0+v0.1*v1.1) as f32;
    let d11=(v1.0*v1.0+v1.1*v1.1) as f32;
    let denom=d00*d11-d01*d01;
    if denom==0.0 { return; }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let v2=(x-a.0,y-a.1);
            let d20=(v2.0*v0.0+v2.1*v0.1) as f32;
            let d21=(v2.0*v1.0+v2.1*v1.1) as f32;
            let v=(d11*d20-d01*d21)/denom;
            let w=(d00*d21-d01*d20)/denom;
            let u=1.0-v-w;
            if u>=0.0 && v>=0.0 && w>=0.0 {
                buf[y as usize*WIDTH+x as usize]=color;
            }
        }
    }
}

fn main() {
    let mut window = Window::new("3D Demo Map", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Failed to open window: {}",e));
    window.set_target_fps(60);
    window.set_cursor_visibility(false);

    let mut camera = Camera::new(Point3::new(0.0,1.0,-5.0));
    let mut last=Instant::now();
    let map=Map::demomap();
    let mut buf=vec![0_u32;WIDTH*HEIGHT];

    while window.is_open() {
        if window.is_key_down(Key::Escape) { return; }
        let now=Instant::now();
        let dt=now.duration_since(last).as_secs_f32();
        last=now;

        // arrow keys rotate camera
        if window.is_key_down(Key::Left) { camera.yaw -= camera.rotation_speed * dt; }
        if window.is_key_down(Key::Right) { camera.yaw += camera.rotation_speed * dt; }
        if window.is_key_down(Key::Up) { camera.pitch -= camera.rotation_speed * dt; }
        if window.is_key_down(Key::Down) { camera.pitch += camera.rotation_speed * dt; }
        camera.pitch=camera.pitch.clamp(-std::f32::consts::FRAC_PI_2+0.01,std::f32::consts::FRAC_PI_2-0.01);

        // WASD movement
        let forward=Point3::new(camera.yaw.sin(),0.0,camera.yaw.cos());
        let right_v=Point3::new(camera.yaw.cos(),0.0,-camera.yaw.sin());
        if window.is_key_down(Key::W) { camera.pos.x+=forward.x*camera.speed*dt; camera.pos.z+=forward.z*camera.speed*dt; }
        if window.is_key_down(Key::S) { camera.pos.x-=forward.x*camera.speed*dt; camera.pos.z-=forward.z*camera.speed*dt; }
        if window.is_key_down(Key::A) { camera.pos.x-=right_v.x*camera.speed*dt; camera.pos.z-=right_v.z*camera.speed*dt; }
        if window.is_key_down(Key::D) { camera.pos.x+=right_v.x*camera.speed*dt; camera.pos.z+=right_v.z*camera.speed*dt; }

        // clear
        buf.fill(0xFF87CEEB);
        
        // render meshes
        for (mi,mesh) in map.meshes.iter().enumerate() {
            for (ti,tri) in mesh.triangles().iter().enumerate() {
                if let (Some(a),Some(b),Some(c))=(
                    project(camera.world_to_camera(tri.a),WIDTH,HEIGHT),
                    project(camera.world_to_camera(tri.b),WIDTH,HEIGHT),
                    project(camera.world_to_camera(tri.c),WIDTH,HEIGHT),
                ) {
                    let color = if mi==1 {
                        match ti {
                            0=>0xFFFF0000,1=>0xFF00FF00,2=>0xFF0000FF,
                            3=>0xFFFFFF00,4=>0xFFFF00FF,5=>0xFF00FFFF,_=>0xFFFFFFFF
                        }
                    } else { 0xFF888888 };
                    if mi==1 { fill_triangle(&mut buf,a,b,c,color); }
                    else { draw_line(&mut buf,a.0,a.1,b.0,b.1,color); draw_line(&mut buf,b.0,b.1,c.0,c.1,color); draw_line(&mut buf,c.0,c.1,a.0,a.1,color);}                }
            }
        }
        window.update_with_buffer(&buf,WIDTH,HEIGHT).unwrap();
    }
}
