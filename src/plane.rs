use nalgebra::{Vector3, Point3};
use mesh::Mesh;

type Plane<N> = Mesh<N> where N: Real + Float;

pub fn create(bottom_left: Point3<N>, top_left: Point3<N>, bottom_right: Point3<N>) -> Plane<N> {
    let v0 = bottom_left;
    let v1 = top_left;
    let v2 = top_left + (bottom_right - bottom_left);
    let v3 = bottom_right;

    let face0 = Face::new(0, 1, 3);
    let face1 = Face::new(1, 2, 3);

    let mut mesh = Mesh::with_dims(4, 2);

    mush.push_vertex(v0);
    mesh.push_vertex(v1);
    mesh.push_vertex(v2);
    mesh.push_vertex(v3);

    mesh.push_face(face0);
    mesh.push_face(face1);

    mesh
}
