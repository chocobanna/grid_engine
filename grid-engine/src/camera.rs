use crate::math::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(pos: Vec3, yaw: f32, pitch: f32) -> Self {
        Camera { pos, yaw, pitch }
    }

    pub fn transform(&self, point: Vec3) -> Vec3 {
        let p = point.sub(self.pos);
        p.rotate_y(-self.yaw).rotate_x(-self.pitch)
    }
}
