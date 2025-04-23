use crate::{Mesh, Point3};
/// A 3D map composed of multiple meshes
pub struct Map { pub meshes: Vec<Mesh> }
impl Map {
    pub fn new() -> Self { Map { meshes: Vec::new() } }
    pub fn add_mesh(&mut self, mesh: Mesh) { self.meshes.push(mesh); }
    /// A demo map: a grid ground and a cube on top
    pub fn demomap() -> Self {
        let mut map = Map::new();
        // ground plane grid
        let grid_size = 20;
        let spacing = 1.0;
        let mut vertices = Vec::new();
        for z in 0..=grid_size {
            for x in 0..=grid_size {
                vertices.push(Point3::new(
                    (x as f32 - grid_size as f32 / 2.0) * spacing,
                    -1.0,
                    (z as f32 - grid_size as f32 / 2.0) * spacing,
                ));
            }
        }
        let mut indices = Vec::new();
        for z in 0..grid_size {
            for x in 0..grid_size {
                let i0 = z * (grid_size + 1) + x;
                let i1 = i0 + 1;
                let i2 = i0 + (grid_size + 1);
                let i3 = i2 + 1;
                indices.push([i0, i2, i1]);
                indices.push([i1, i2, i3]);
            }
        }
        map.add_mesh(Mesh::new(vertices, indices));
        // cube mesh
        let cube_vertices = vec![
            Point3::new(-1.0, 0.0, -1.0), Point3::new(1.0, 0.0, -1.0),
            Point3::new(1.0, 2.0, -1.0),  Point3::new(-1.0, 2.0, -1.0),
            Point3::new(-1.0, 0.0, 1.0),  Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 2.0, 1.0),    Point3::new(-1.0, 2.0, 1.0),
        ];
        let cube_indices = vec![
            [0,1,2],[2,3,0], // back
            [4,5,6],[6,7,4], // front
            [0,4,7],[7,3,0], // left
            [5,1,2],[2,6,5], // right
            [3,7,6],[6,2,3], // top
            [0,1,5],[5,4,0], // bottom
        ];
        map.add_mesh(Mesh::new(cube_vertices, cube_indices));
        map
    }
}
