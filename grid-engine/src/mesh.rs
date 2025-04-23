use crate::math::Vec3;

pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn cube() -> Self {
        let verts = vec![
            Vec3::new(-1., -1., -1.), Vec3::new(1., -1., -1.),
            Vec3::new(1.,  1., -1.), Vec3::new(-1.,  1., -1.),
            Vec3::new(-1., -1.,  1.), Vec3::new(1., -1.,  1.),
            Vec3::new(1.,  1.,  1.), Vec3::new(-1.,  1.,  1.),
        ];
        let faces = vec![
            [4,5,6], [4,6,7], // front
            [1,0,3], [1,3,2], // back
            [0,4,7], [0,7,3], // left
            [5,1,2], [5,2,6], // right
            [7,6,2], [7,2,3], // top
            [0,1,5], [0,5,4], // bottom
        ];
        Mesh { verts, faces }
    }
}
