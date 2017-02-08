use nalgebra::{Vector3, Vector4, Point3, Point4, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat};


/// Generate the camera transformation from the given data.
pub fn world_to_camera<N>(eye: Point3<N>, gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
    where N: Copy + BaseFloat
{
    // The vectors are all cast into homogeneous coordinates here. Points are affected
    // by translation, so `eye` has a `1` in its fourth component, while vectors are
    // not affected by translation, so they have a `0` in their fourth components.
    let w = -gaze / gaze.norm();
    let top_cross_w = top.cross(&w);
    let u = top_cross_w / top_cross_w.norm();
    let v = w.cross(&u);
    let zero = N::zero();
    let one = N::one();

    Matrix4::new(u.x,  v.x,  w.x,  -eye.x,
                 u.y,  v.y,  w.y,  -eye.y,
                 u.z,  v.z,  w.z,  -eye.z,
                 zero, zero, zero,  one  )
}
