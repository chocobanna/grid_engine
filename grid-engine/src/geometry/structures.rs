#[derive(Clone, Copy, Debug)]
/// A point in 3D space
pub struct Point3 { pub x: f32, pub y: f32, pub z: f32 }
impl Point3 { pub fn new(x: f32, y: f32, z: f32) -> Self { Point3 { x, y, z } } }

/// A triangle composed of three 3D points
pub struct Triangle { pub a: Point3, pub b: Point3, pub c: Point3 }
impl Triangle { pub fn new(a: Point3, b: Point3, c: Point3) -> Self { Triangle { a, b, c } } }

/// A mesh is a set of vertices and triangle indices
pub struct Mesh { pub vertices: Vec<Point3>, pub indices: Vec<[usize; 3]> }
impl Mesh {
    pub fn new(vertices: Vec<Point3>, indices: Vec<[usize; 3]>) -> Self { Mesh { vertices, indices } }
    /// Returns the triangles of the mesh
    pub fn triangles(&self) -> Vec<Triangle> {
        self.indices.iter().map(|&[i,j,k]| Triangle::new(self.vertices[i], self.vertices[j], self.vertices[k])).collect()
    }
}
