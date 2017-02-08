use nalgebra::{Vector3, Vector4, Point3, Point4, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat};


/// Generate the camera transformation from the given data.
pub fn world_to_camera<N>(eye: Point3<N>, gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
    where N: Copy + BaseFloat
{
    // The vectors are all cast into homogeneous coordinates here. Points are affected
    // by translation, so `eye` has a `1` in its fourth component, while vectors are
    // not affected by translation, so they have a `0` in their fourth components.
    let zero = N::zero();
    let one = N::one();
    
    let w = -gaze / gaze.norm();
    let top_cross_w = top.cross(&w);
    let u = top_cross_w / top_cross_w.norm();
    let v = w.cross(&u);

    let m11 = u.x;
    let m21 = v.x;
    let m31 = w.x;
    let m41 = zero;
    let m12 = u.y;
    let m22 = v.y;
    let m32 = w.y;
    let m42 = zero;
    let m13 = u.z;
    let m23 = v.z;
    let m33 = w.z;
    let m43 = zero;
    let m14 = -eye.x;
    let m24 = -eye.y;
    let m34 = -eye.z;
    let m44 = one;

    // Transformations in graphics tend to be 4x4 so we can take advantage 
    // of homogeneous coordinates. This converts translations from affine transformations
    // to linear ones in one greater dimension.
    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

/// Generate the perspective matrix from creating perspective projection
/// transformations. This is for looking down the -z axis.
pub fn perspective<N>(near: N, far: N) -> Matrix4<N>
    where N: Copy + BaseFloat 
{
    assert!(near > far);

    let zero = N::zero();
    let one = N::one();

    let m11 = near;
    let m21 = zero;
    let m31 = zero;
    let m41 = zero;
    let m12 = zero;
    let m22 = near;
    let m32 = zero;
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = near + far;
    let m43 = one;
    let m14 = zero;
    let m24 = zero;
    let m34 = -far * near;
    let m44 = zero; 

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
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

/// Compute the viewport, (windowing) transformation. This takes vertices
/// from the canonical view volume (projection coordinates) to pixel coordinates.
/// This depends only on the width of the image (to be used for calculating colors in 
/// the frame buffer). Note that the viewport transformation is a special case of 
/// an orthographic (length preserving) transformation. This casts vertices into the
/// coordinate system [-0.5, n_x - 0.5] x [-0.5, n_y - 0.5], where n_x is the number 
/// of pixels going in the x-direction, and n_y is the number of pixels going in the 
/// y-direction, i.e. (n_x, n_y) is the resolution of the screen.
pub fn viewport<N>(num_x: usize, num_y: usize) -> Matrix4<N>
    where N: Copy + BaseFloat
{
    let zero = N::zero();
    let one  = N::one();
    let two  = one + one;

    let mut _num_x = zero;
    for i in 0..num_x {
        _num_x += one;
    }

    let mut _num_y = zero;
    for i in 0..num_y {
        _num_y += one;
    };

    let m11 = _num_x / two;
    let m21 = zero;
    let m31 = zero;
    let m41 = zero;
    let m12 = zero;
    let m22 = _num_y / two;
    let m32 = zero;
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = one;
    let m43 = zero;
    let m14 = (_num_x - one) / two;
    let m24 = (_num_y - one) / two;
    let m34 = zero;
    let m44 = one;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}
