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

    // Transformations in graphics tend to be 4x4 so we can take advantage 
    // of homogeneous coordinates. This converts translations from affine transformations
    // to linear ones in one greater dimension.
    Matrix4::new(u.x,  v.x,  w.x,  -eye.x,
                 u.y,  v.y,  w.y,  -eye.y,
                 u.z,  v.z,  w.z,  -eye.z,
                 zero, zero, zero,  one  )
}

/// Generate the perspective matrix from creating perspective projection
/// transformations. This is for looking down the -z axis.
pub fn perspective<N>(near: N, far: N) -> Matrix4<N>
    where N: Copy + BaseFloat 
{
    assert!(near > far);

    let zero = N::zero();
    let one = N::one();

    Matrix4::new(near, zero, zero,       zero,
                 zero, near, zero,       zero,
                 zero, zero, near + far, one,
                 zero, zero, -far*near,  zero)
}

/// Convert from projected coordinates to the canonical view 
/// volume [-1, 1] x [-1, 1] x [-1, 1].
pub fn orthographic<N>(left: N, right: N, top: N, 
                       bottom: N, near: N, far: N) -> Matrix4<N> 
    where N: Copy + BaseFloat
{
    assert!(near > far);

    let zero = N::zero();
    let one = N::one();
    let two = one + one;

    let m11 = two / (right - left);
    let m21 = zero;
    let m31 = zero;
    let m41 = zero;
    let m12 = zero;
    let m22 = two / (top - bottom);
    let m32 = zero;
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = two / (near - far);
    let m43 = zero;
    let m14 = -(right + left) / (right - left);
    let m24 = -(top + bottom) / (top - bottom);
    let m34 = -(near + far) / (near - far);
    let m44 = one; 

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

/// Perspective projection transformation. This takes us from camera coordinates to
/// the canonical view volume.
pub fn perspective_project<N>(left: N, right: N, top: N, 
                              bottom: N, near: N, far: N) -> Matrix4<N> 
    where N: Copy + BaseFloat
{
    let zero = N::zero();
    let one = N::one();
    let two = one + one;

    let m11 = (two * near) / (right - left);
    let m21 = zero;
    let m31 = zero;
    let m41 = zero;
    let m12 = zero;
    let m22 = (two * near) / (top - bottom);
    let m32 = zero;
    let m42 = zero;
    let m13 = (left + right) / (left - right);
    let m23 = (bottom + top) / (bottom - top);
    let m33 = (far + near) / (near - far);
    let m43 = one;
    let m14 = zero;
    let m24 = zero;
    let m34 = (two * far * near) / (far - near);
    let m44 = zero;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

