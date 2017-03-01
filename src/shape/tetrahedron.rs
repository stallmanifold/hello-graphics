use mesh::{Face, Mesh};
use nalgebra::{Vector3, Point3};
use num_traits::Float;
use alga::general::Real;


type Tetrahedron<N> = Mesh<N>;

pub fn create<N: Real + Float>(v0: Point3<N>, v1: Point3<N>, v2: Point3<N>, v3: Point3<N>) -> Tetrahedron<N> {
    let mut mesh = Mesh::with_dims(4, 4);

    mesh.push_vertex(v0);
    mesh.push_vertex(v1);
    mesh.push_vertex(v2);
    mesh.push_vertex(v3);

    let face032 = Face::new(0, 3, 2);
    let face231 = Face::new(2, 3, 1);
    let face012 = Face::new(0, 1, 2);
    let face130 = Face::new(1, 3, 0);

    mesh.push_face(&face032);
    mesh.push_face(&face231);
    mesh.push_face(&face012);
    mesh.push_face(&face130);

    mesh
}
