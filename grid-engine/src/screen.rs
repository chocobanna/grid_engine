pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

pub fn put_pixel(buf: &mut [u32], x: i32, y: i32, color: u32) {
    if x < 0 || y < 0 { return }
    let (x, y) = (x as usize, y as usize);
    if x >= WIDTH || y >= HEIGHT { return }
    buf[y * WIDTH + x] = color;
}

pub fn fill_triangle(buf: &mut [u32], mut pts: [(i32,i32);3], color: u32) {
    pts.sort_by_key(|&(_,y)| y);
    let [(x0,y0),(x1,y1),(x2,y2)] = pts;
    let invs = |x0,y0,x1,y1| if y1 != y0 { (x1-x0) as f32/(y1-y0) as f32 } else { 0.0 };
    let dx01 = invs(x0,y0,x1,y1);
    let dx02 = invs(x0,y0,x2,y2);
    let dx12 = invs(x1,y1,x2,y2);

    let mut xa = x0 as f32;
    let mut xb = x0 as f32;
    for y in y0..=y1 {
        for x in (xa.ceil() as i32)..=(xb.floor() as i32) {
            put_pixel(buf, x, y, color);
        }
        xa += dx01; xb += dx02;
    }
    xa = x1 as f32;
    xb = x0 as f32 + dx02 * ((y1-y0+1) as f32);
    for y in (y1+1)..=y2 {
        for x in (xa.ceil() as i32)..=(xb.floor() as i32) {
            put_pixel(buf, x, y, color);
        }
        xa += dx12; xb += dx02;
    }
}
