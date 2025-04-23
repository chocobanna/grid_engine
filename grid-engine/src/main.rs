use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Lines & Polygons—No Frills")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

    event_loop..expect("Failure to draw lines.").run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent { event, .. } = event {
            if matches!(event, WindowEvent::CloseRequested | WindowEvent::KeyboardInput { .. }) {
                *control_flow = ControlFlow::Exit;
            }
        }

        // <-- HERE: get a mutable frame slice
        let frame = pixels.frame_mut();
        clear(frame);

        draw_line(frame, (50,  50),  (200, 120), [0xFF, 0x00, 0x00, 0xFF]);
        draw_line(frame, (50, 120),  (200, 200), [0x00, 0xFF, 0x00, 0xFF]);

        let tri = [(320, 100), (100, 380), (540, 400)];
        fill_triangle(frame, tri, [0x00, 0x00, 0xFF, 0xFF]);

        pixels.render().unwrap();
    });
}

fn clear(frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0x20, 0x20, 0x20, 0xFF]);
    }
}

fn put_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8;4]) {
    if x < 0 || y < 0 { return }
    let (x, y) = (x as u32, y as u32);
    if x >= WIDTH || y >= HEIGHT { return }
    let idx = ((y * WIDTH + x) * 4) as usize;
    frame[idx..idx+4].copy_from_slice(&color);
}

fn draw_line(frame: &mut [u8], start: (i32,i32), end: (i32,i32), color: [u8;4]) {
    let (x0, y0) = start;
    let (x1, y1) = end;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 {1} else {-1};
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 {1} else {-1};
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        put_pixel(frame, x, y, color);
        if x == x1 && y == y1 { break }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x += sx; }
        if e2 <= dx { err += dx; y += sy; }
    }
}

fn fill_triangle(frame: &mut [u8], mut pts: [(i32,i32);3], color: [u8;4]) {
    // sort by y ascending
    pts.sort_by_key(|&(_, y)| y);
    // <-- Destructure the array itself
    let [(x0,y0), (x1,y1), (x2,y2)] = pts;

    let mut span = |y: i32, xa: f32, xb: f32| {
        for x in (xa.ceil() as i32)..=(xb.floor() as i32) {
            put_pixel(frame, x, y, color);
        }
    };

    let slope = |x0,y0,x1,y1| {
        if y1 != y0 { (x1 - x0) as f32 / (y1 - y0) as f32 } else { 0.0 }
    };
    let dx01 = slope(x0,y0,x1,y1);
    let dx02 = slope(x0,y0,x2,y2);
    let dx12 = slope(x1,y1,x2,y2);

    // bottom part
    let mut xa = x0 as f32;
    let mut xb = x0 as f32;
    for y in y0..=y1 {
        span(y, xa, xb);
        xa += dx01;
        xb += dx02;
    }
    // top part
    xa = x1 as f32;
    xb = x0 as f32 + dx02 * ((y1 - y0 + 1) as f32);
    for y in (y1+1)..=y2 {
        span(y, xa, xb);
        xa += dx12;
        xb += dx02;
    }
}
