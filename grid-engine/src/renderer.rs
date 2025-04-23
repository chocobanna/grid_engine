use crate::math::Vec3;
use crate::mesh::Mesh;
use crate::camera::Camera;
use crate::screen::{WIDTH, HEIGHT, fill_triangle};

pub fn render(mesh: &Mesh, cam: &Camera, cube_rot: (f32,f32), buffer: &mut [u32]) {
    let mut tris: Vec<([(i32,i32);3], f32)> = Vec::new();
    let (angle_x, angle_y) = cube_rot;

    for face in &mesh.faces {
        let v0 = mesh.verts[face[0]].rotate_x(angle_x).rotate_y(angle_y);
        let v1 = mesh.verts[face[1]].rotate_x(angle_x).rotate_y(angle_y);
        let v2 = mesh.verts[face[2]].rotate_x(angle_x).rotate_y(angle_y);

        let t0 = Vec3::new(v0.x, v0.y, v0.z + 5.0);
        let t1 = Vec3::new(v1.x, v1.y, v1.z + 5.0);
        let t2 = Vec3::new(v2.x, v2.y, v2.z + 5.0);

        let c0 = cam.transform(t0);
        let c1 = cam.transform(t1);
        let c2 = cam.transform(t2);

        // Back-face culling removed to render all triangles

        let proj = |p: Vec3| {
            let pd = 200.0;
            let x = (p.x / p.z) * pd + WIDTH as f32/2.0;
            let y = (p.y / p.z) * pd + HEIGHT as f32/2.0;
            (x as i32, y as i32)
        };
        let p0 = proj(c0);
        let p1 = proj(c1);
        let p2 = proj(c2);
        let avg_z = (c0.z + c1.z + c2.z) / 3.0;

        tris.push(([p0,p1,p2], avg_z));
    }

    tris.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    for (pts, _) in tris {
        fill_triangle(buffer, pts, 0x00FF00FF);
    }
}