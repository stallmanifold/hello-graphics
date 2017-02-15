#![allow(dead_code)]
use nalgebra::{Vector3, Point3, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat, Inverse};
use util;


/// TODO: Check divisions for zeros in matrix code.
/// TODO: Convert all raster functions to homogeneous 4D coordinates.
/// TODO: Make interface to raster functions consistent (with pointers to data types instead of copying them.)

///
/// Generate the world transformation from the given camera data.
///
pub fn camera_to_world_matrix<N>(eye: Vector3<N>, gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
    where N: BaseFloat
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
    let m41 = eye.x;
    let m12 = u.y;
    let m22 = v.y;
    let m32 = w.y;
    let m42 = eye.y;
    let m13 = u.z;
    let m23 = v.z;
    let m33 = w.z;
    let m43 = eye.z;
    let m14 = zero;
    let m24 = zero;
    let m34 = zero;
    let m44 = one;

    // Transformations in graphics tend to be 4x4 so we can take advantage 
    // of homogeneous coordinates. This converts translations from affine transformations
    // to linear ones in one greater dimension.
    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Generates the world to camera matrix.
///
pub fn world_to_camera_matrix<N>(eye: Vector3<N>, gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
    where N: BaseFloat
{
    camera_to_world_matrix(eye, gaze, top).inverse().unwrap()
}

///
/// Generate the perspective matrix from creating perspective projection
/// transformations. This is for looking down the negative z-axis.
///
pub fn perspective_matrix<N>(near: N, far: N) -> Matrix4<N>
    where N: BaseFloat 
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
    let m43 = -far * near;
    let m14 = zero;
    let m24 = zero;
    let m34 = one;
    let m44 = zero; 

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Constructs a translation matrix from a three-dimensional vector. 
///
pub fn translation_matrix<N>(eye: Vector3<N>) -> Matrix4<N>
    where N: BaseFloat
{
    let zero = N::zero();
    let one = N::one();

    let m11 = one;
    let m21 = zero;
    let m31 = zero;
    let m41 = eye.x;
    let m12 = zero;
    let m22 = one;
    let m32 = zero;
    let m42 = eye.y;
    let m13 = zero;
    let m23 = zero;
    let m33 = one;
    let m43 = eye.z;
    let m14 = zero;
    let m24 = zero;
    let m34 = zero;
    let m44 = one; 

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Constructs a rotation matrix from a set of coordinate axes.
///
pub fn rotation_matrix<N>(gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
    where N: BaseFloat
{
    let zero = N::zero();
    let one  = N::one();

    // Compute the orientation of the world space axes in the rotated space.
    let w = -gaze / gaze.norm();
    let t_cross_w = top.cross(&w);
    let u = t_cross_w / t_cross_w.norm();
    let v = w.cross(&u);

    let m11 = u.x;
    let m21 = u.y;
    let m31 = u.z;
    let m41 = zero;
    let m12 = v.x;
    let m22 = v.y;
    let m32 = v.z;
    let m42 = zero;
    let m13 = w.x;
    let m23 = w.y;
    let m33 = w.z;
    let m43 = zero;
    let m14 = zero;
    let m24 = zero;
    let m34 = zero;
    let m44 = one;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Convert from projected coordinates to the canonical view 
/// volume [-1, 1] x [-1, 1] x [-1, 1].
///
pub fn orthographic_matrix<N>(left: N, 
                              right: N, 
                              top: N, 
                              bottom: N, 
                              near: N, 
                              far: N) -> Matrix4<N> 
    where N: BaseFloat
{
    assert!(near > far);

    let zero = N::zero();
    let one  = N::one();
    let two  = one + one;

    let m11 = two / (right - left);
    let m21 = zero;
    let m31 = zero;
    let m41 = -(right + left) / (right - left);
    let m12 = zero;
    let m22 = two / (top - bottom);
    let m32 = zero;
    let m42 = -(top + bottom) / (top - bottom);
    let m13 = zero;
    let m23 = zero;
    let m33 = two / (near - far);
    let m43 = -((near + far) / (near - far));
    let m14 = zero;
    let m24 = zero;
    let m34 = zero;
    let m44 = one;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Perspective projection transformation. This takes us from camera coordinates to
/// the canonical view volume.
///
pub fn perspective_projection_matrix<N>(left: N, 
                                        right: N,
                                        top: N, 
                                        bottom: N, 
                                        near: N, 
                                        far: N) -> Matrix4<N> 
    where N: BaseFloat
{
    let zero = N::zero();
    let one = N::one();
    let two = one + one;

    let m11 = (two * near) / (right - left);
    let m21 = zero;
    let m31 = (left + right) / (left - right);
    let m41 = zero;
    let m12 = zero;
    let m22 = (two * near) / (top - bottom);
    let m32 = (bottom + top) / (bottom - top);
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = (far + near) / (near - far);
    let m43 = (two * far * near) / (far - near);
    let m14 = zero;
    let m24 = zero;
    let m34 = one;
    let m44 = zero;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

///
/// Compute the viewport, (windowing) transformation. This takes vertices
/// from the canonical view volume (projection coordinates) to pixel coordinates.
/// This depends only on the width of the image (to be used for calculating colors in 
/// the frame buffer). Note that the viewport transformation is a special case of 
/// an orthographic (length preserving) transformation. This casts vertices into the
/// coordinate system [-0.5, n_x - 0.5] x [-0.5, n_y - 0.5], where n_x is the number 
/// of pixels going in the x-direction, and n_y is the number of pixels going in the 
/// y-direction, i.e. (n_x, n_y) is the resolution of the screen. A pixel is one unit
/// wide in this coordinate system.
///
use std::fmt;
pub fn viewport_matrix<N>(num_x: usize, num_y: usize) -> Matrix4<N>
    where N: BaseFloat + fmt::Debug
{
    let zero = N::zero();
    let one  = N::one();
    let two  = one + one;

    // Approximate num_x using type N.
    let mut image_width = zero;
    for _ in 0..num_x {
        image_width += one;
    }

    // Approximate num_y using type N.
    let mut image_height = zero;
    for _ in 0..num_y {
        image_height += one;
    }

    let m11 = image_width / two;
    let m21 = zero;
    let m31 = zero;
    let m41 = (image_width - one) / two;
    let m12 = zero;
    let m22 = image_height / two;
    let m32 = zero;
    let m42 = (image_height - one) / two;
    let m13 = zero;
    let m23 = zero;
    let m33 = one;
    let m43 = zero;
    let m14 = zero;
    let m24 = zero;
    let m34 = zero;
    let m44 = one;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

/// Generate a matrix to convert from world space to raster space.
pub fn world_to_raster_matrix<N>(left: N, 
                                 right: N, 
                                 top: N, 
                                 bottom: N, 
                                 near: N, 
                                 far: N, 
                                 image_width: usize, 
                                 image_height: usize) -> Matrix4<N> 
    where N: BaseFloat + fmt::Debug
{
    let pp_matrix: Matrix4<N> = perspective_projection_matrix(left, right, top, bottom, near, far);
    let vp_matrix: Matrix4<N> = viewport_matrix(image_width, image_height);

    vp_matrix * pp_matrix
}

pub struct BoundingBox<N> {
    pub x_min: N,
    pub x_max: N,
    pub y_min: N,
    pub y_max: N
}

/// Given a triangle primitive, this function computers the bounding bounding_box
/// on the screen for that primitive. This function does not take into account 
/// the boundaries of the image frame.
pub fn bounding_box<N>(p1: &Point3<N>,
                       p2: &Point3<N>,
                       p3: &Point3<N>) -> BoundingBox<N>
    where N: BaseFloat
{
    let x_min = util::min3(p1.x, p2.x, p3.x).floor();
    let x_max = util::max3(p1.x, p2.x, p3.x).ceil();
    let y_min = util::min3(p1.y, p2.y, p3.y).floor();
    let y_max = util::max3(p1.y, p2.y, p3.y).ceil();

    BoundingBox {
        x_min: x_min,
        x_max: x_max,
        y_min: y_min,
        y_max: y_max
    }   
}

/// Compute on which side of a triangle edge a point is on. They are defined such that
/// They are iterated in clockwise order. This way a point is positive if it lies
/// entirely within the triangle.
#[inline]
pub fn compute_edge<N>(v1: &Point3<N>, 
                       v2: &Point3<N>,
                        p: &Point3<N>) -> N
    where N: BaseFloat
{
    (p.x - v1.x)*(v2.y - v1.y) - (p.y - v1.y)*(v2.x - v1.x)
}

///
/// Compute the coordinates of a ray in barycentric coordinates. The coordinates
/// `v0`, `v1`, and `v2` are assumed to be in clockwise order.
///
pub fn barycentric_coords<N>(v0: &Point3<N>,
                             v1: &Point3<N>,
                             v2: &Point3<N>,
                              p: &Point3<N>) -> Point3<N>
    where N: BaseFloat
{
    let w0 = compute_edge(v1, v2, p);
    let w1 = compute_edge(v2, v0, p);
    let w2 = compute_edge(v0, v1, p);

    Point3::new(w0, w1, w2)
}

///
/// Computes the area of a triangle primitive.
///
pub fn compute_area<N>(v0: &Point3<N>,
                       v1: &Point3<N>,
                       v2: &Point3<N>,) -> N
    where N: BaseFloat
{
    compute_edge(v0, v1, v2).abs()
}

#[cfg(test)]
mod tests {
    use nalgebra::{Vector3, 
                   Point3,
                   Point4,
                   Matrix4,
                   ApproxEq, 
                   ToHomogeneous,
                   Cross,
                   Norm};


    #[test]
    fn test_bounding_box_should_bound_a_primitive() {
        let v0: Point3<f32>  = Point3::new(-48.0, -10.0, 82.0);
        let v1: Point3<f32>  = Point3::new(29.0, -15.0, 44.0);
        let v2: Point3<f32>  = Point3::new(13.0, 34.0, 114.0);

        let bbox = super::bounding_box(&v0, &v1, &v2);
        let vertices = vec![v0, v1, v2];

        for vertex in vertices {
            assert!(bbox.x_min <= vertex.x);
            assert!(bbox.x_max >= vertex.x);
            assert!(bbox.y_min <= vertex.y);
            assert!(bbox.y_max >= vertex.y);
        }
    }

    #[test]
    fn test_perspective_matrix_equation_should_be_the_product_of_orthographic_and_perspective() {
        let left: f32   = -4.5; 
        let right: f32  =  3.5; 
        let top: f32    =  5.4;
        let bottom: f32 = -3.4; 
        let near: f32   = -1.0; 
        let far: f32    = -6.2;
        let m_persp_proj = super::perspective_projection_matrix(left, right, top, bottom, near, far);
        let m_persp = super::perspective_matrix(near, far);
        let m_orth  = super::orthographic_matrix(left, right, top, bottom, near, far);

        println!("m_persp_proj = {:?}", m_persp_proj);
        println!("\n");
        println!("m_orth * m_persp = {:?}", m_orth * m_persp);
        println!("\n");
        println!("m_orth * m_persp = {:?}", m_persp * m_orth);

        assert!(m_persp_proj.approx_eq(&(m_orth * m_persp)));
    }

    #[test]
    fn test_translation_matrix_should_be_same_as_vector_displacement() {
        let trans    = Vector3::new(2.0, 2.0, 2.0);
        let m_trans  = super::translation_matrix(trans);
        let point    = Point3::new(-4.5, 7.5, 80.0);
        let point_h  = point.to_homogeneous();
        let point2_h = m_trans * point_h;
        let point2   = (point + trans).to_homogeneous();

        assert!(point2_h.approx_eq(&point2));
    }

    #[test]
    fn test_translation_matrix_should_respect_homogeneous_coordinates() {
        let trans = Vector3::new(2.0, 2.0, 2.0);
        let m_trans = super::translation_matrix(trans);
        let point = Point4::new(-4.5, 7.5, 8.0, 0.0);

        println!("{}", m_trans * point);

        assert!(point.approx_eq(&(m_trans * point)));
    }

    #[test]
    fn test_translation_matrix_with_no_displacement_should_be_identity() {
        let trans = Vector3::new(0.0, 0.0, 0.0);
        let m_trans = super::translation_matrix(trans);
        let point = Point4::new(-4.5, 7.5, 8.0, 1.0);

        println!("{}", m_trans * point);

        assert!(point.approx_eq(&(m_trans * point)));

        let identity = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                    0.0, 1.0, 0.0, 0.0,
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0);

        assert!(identity.approx_eq(&(m_trans)));
    }

    #[test]
    fn test_world_to_raster_matrix_should_equal_viewport_times_perspective_projection() {
        let left: f32     = -4.5; 
        let right: f32    =  3.5; 
        let top: f32      =  5.4;
        let bottom: f32   = -3.4; 
        let near: f32     = -1.0; 
        let far: f32      = -6.2;
        let width: usize  = 1920;
        let height: usize = 1080;

        let world_to_raster = super::world_to_raster_matrix(left, 
                                                            right, 
                                                            top, 
                                                            bottom, 
                                                            near, 
                                                            far, 
                                                            width, 
                                                            height);
        let vp = super::viewport_matrix::<f32>(width, height);
        let ppm = super::perspective_projection_matrix(left, right, top, bottom, near, far);

        assert!(world_to_raster.approx_eq(&(vp * ppm)));
    }
    
    #[test]
    fn test_world_to_camera_matrix_should_be_a_rigid_body_transformation() {
        // A world to camera transformation should be the product of a displacement of the
        // world space origin to the camera followed by a rotation.
        let eye  = Vector3::new(45.0, 32.5, -19.0);
        let gaze = Vector3::new(-3.6, -4.0, 5.0);
        let top  = Vector3::new(0.0, 0.0, 1.0); 

        let m_trans = super::translation_matrix(-eye);
        let m_rot   = super::rotation_matrix(gaze, top);
        let m_wtoc  = super::world_to_camera_matrix(eye, gaze, top);

        println!("{}\n", m_trans);
        println!("{}\n", m_rot);
        println!("{}\n", (m_rot * m_trans));
        println!("Result from world to camera:");
        println!("{}\n", m_wtoc);        

        // TODO: Benchmark this.

        assert!(m_wtoc.approx_eq(&(m_rot * m_trans)));
    }

    #[test]
    fn test_world_to_camera_matrix() {
        let p_xyz = Point4::new(10.0, 5.0, 9.0, 1.0);
        let eye   = Vector3::new(45.0, 32.5, -19.0);
        let gaze  = Vector3::new(-3.6, -4.0, 5.0);
        let top   = Vector3::new(0.0, 0.0, 1.0);
        let w     = -gaze / gaze.norm();
        let t_cross_w = top.cross(&w);
        let u = t_cross_w / t_cross_w.norm();
        let v = w.cross(&u);

        let m_rot = Matrix4::new(u.x, u.y, u.z, 0.0,
                                 v.x, v.y, v.z, 0.0,
                                 w.x, w.y, w.z, 0.0,
                                 0.0, 0.0, 0.0, 1.0);

        let m_trans = Matrix4::new(1.0, 0.0, 0.0, -eye.x,
                                   0.0, 1.0, 0.0, -eye.y,
                                   0.0, 0.0, 1.0, -eye.z,
                                   0.0, 0.0, 0.0,    1.0);

        let m = m_rot * m_trans;
        let m_wtoc = super::world_to_camera_matrix(eye, gaze, top);

        assert!(m_wtoc.approx_eq(&m));
        assert!((m_wtoc * p_xyz).approx_eq(&(m * p_xyz)));
    }

    #[test]
    fn test_triangle_area_function_should_return_same_area() {
        let v0: Point3<f32> = Point3::new(13.0, 34.0, 114.0);
        let v1: Point3<f32> = Point3::new(29.0, -15.0, 44.0);
        let v2: Point3<f32> = Point3::new(-48.0, -10.0, 82.0);

        let area0: f32 = super::compute_area(&v0, &v1, &v2);
        let area1: f32 = super::compute_area(&v1, &v2, &v0);
        let area2: f32 = super::compute_area(&v2, &v0, &v1);
        let area3: f32 = super::compute_area(&v2, &v1, &v0);
        let area4: f32 = super::compute_area(&v0, &v2, &v1);

        assert_eq!(area0.abs(), area1.abs());
        assert_eq!(area1.abs(), area2.abs());
        assert_eq!(area2.abs(), area3.abs());
        assert_eq!(area3.abs(), area4.abs());
    }
}
