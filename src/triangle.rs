use mesh::Mesh;
use nalgebra::{Vector3, Point3};


type Triangle<N> = Mesh<N>;

pub fn create(v0: Point3<N>, v1: Point3<N>, v2: Point3<N>) -> Triangle<N> {
    let mut mesh = Mesh::with_dims(3, 1);

    mesh.push_vertex(v0);
    mesh.push_vertex(v1);
    mesh.push_vertex(v2);

    let face = Face::new(0, 1, 2);

    mesh.push_face(face);

    mesh
}