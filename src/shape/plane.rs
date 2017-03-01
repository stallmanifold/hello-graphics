use mesh::{Face, Mesh};
use nalgebra::{Vector3, Point3};
use num_traits::Float;
use alga::general::Real;


type Plane<N> = Mesh<N>;

pub fn create<N: Real + Float>(bottom_left: Point3<N>, 
                               top_left: Point3<N>, 
                               bottom_right: Point3<N>) -> Plane<N> {
    let v0 = bottom_left;
    let v1 = top_left;
    let v2 = top_left + (bottom_right - bottom_left);
    let v3 = bottom_right;

    let face0 = Face::new(0, 1, 3);
    let face1 = Face::new(3, 1, 2);

    let mut mesh = Mesh::with_dims(4, 2);

    mesh.push_vertex(v0);
    mesh.push_vertex(v1);
    mesh.push_vertex(v2);
    mesh.push_vertex(v3);

    mesh.push_face(&face0);
    mesh.push_face(&face1);

    mesh
}
