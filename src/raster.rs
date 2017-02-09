use nalgebra::{Vector3, Point3, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat};
use num_traits::Float;
use util;
use std::ops;
use color::Rgb;

/// TODO: Check divisions for zeros in matrix code.

/// Generate the camera transformation from the given data.
pub fn world_to_camera_matrix<N>(eye: Point3<N>, gaze: Vector3<N>, top: Vector3<N>) -> Matrix4<N>
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

/// Constructs a translation matrix from a three-dimensional vector. 
pub fn translation_matrix<N>(eye: &Vector3<N>) -> Matrix4<N>
    where N: BaseFloat
{
    let zero = N::zero();
    let one = N::one();

    let m11 = one;
    let m21 = zero;
    let m31 = zero;
    let m41 = zero;
    let m12 = zero;
    let m22 = one;
    let m32 = zero;
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = one;
    let m43 = zero;
    let m14 = eye.x;
    let m24 = eye.y;
    let m34 = eye.z;
    let m44 = one; 

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

/// Convert from projected coordinates to the canonical view 
/// volume [-1, 1] x [-1, 1] x [-1, 1].
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
pub fn viewport_matrix<N>(num_x: usize, num_y: usize) -> Matrix4<N>
    where N: BaseFloat
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
    let m41 = zero;
    let m12 = zero;
    let m22 = image_height / two;
    let m32 = zero;
    let m42 = zero;
    let m13 = zero;
    let m23 = zero;
    let m33 = one;
    let m43 = zero;
    let m14 = (image_width - one) / two;
    let m24 = (image_height - one) / two;
    let m34 = zero;
    let m44 = one;

    Matrix4::new(m11, m21, m31, m41,
                 m12, m22, m32, m42,
                 m13, m23, m33, m43,
                 m14, m24, m34, m44)
}

pub fn world_to_raster_matrix<N>(left: N, 
                                 right: N, 
                                 top: N, 
                                 bottom: N, 
                                 near: N, 
                                 far: N, 
                                 image_width: usize, 
                                 image_height: usize) -> Matrix4<N> 
    where N: BaseFloat
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
pub fn compute_edge<N>(v1: &Point3<N>, 
                       v2: &Point3<N>,
                        p: &Point3<N>) -> N
    where N: BaseFloat
{
    (p.x - v1.x)*(v2.y - v1.y) - (p.y - v1.y)*(v2.x - v1.x)
}

/// Compute the coordinates of a ray in barycentric coordinates. The coordinates
/// `v0`, `v1`, and `v2` are assumed to be in clockwise order.
pub fn barycentric_coords<N>(v0: &Point3<N>,
                             v1: &Point3<N>,
                             v2: &Point3<N>,
                              p: &Point3<N>) -> Point3<N>
    where N: BaseFloat
{
    let w0 = compute_edge(v0, v1, p);
    let w1 = compute_edge(v1, v2, p);
    let w2 = compute_edge(v2, v0, p);

    Point3::new(w0, w1, w2)
}

pub fn compute_area<N>(v0: &Point3<N>,
                       v1: &Point3<N>,
                       v2: &Point3<N>,) -> N
    where N: BaseFloat
{
    compute_edge(v0, v1, v2)
}

/// Return an initialized heap-allocated z-buffer.
pub fn z_buffer<N: Copy + BaseFloat>(width: usize, height: usize) -> Box<ZBuffer<N>> {
    let mut z_buffer = Box::new(ZBuffer::new(width, height));
    z_buffer.initialize();

    z_buffer
}

/// Use a floating point ZBuffer for right now.
/// TODO: Convert to an integer Z-Buffer.
pub struct ZBuffer<N> {
    width: usize,
    height: usize,
    buf: Vec<Vec<N>>,
}

impl<N> ZBuffer<N> where N: BaseFloat {
    pub fn new(width: usize, height: usize) -> ZBuffer<N> {
        let mut z_buffer = ZBuffer {
            width: width,
            height: height,
            buf: Vec::with_capacity(height)
        };

        for _ in 0..z_buffer.height {
            z_buffer.buf.push(Vec::with_capacity(width));
        }

        let zero = N::zero();

        for i in 0..z_buffer.height {
            for _ in 0..z_buffer.width {
                z_buffer.buf[i].push(zero);
            }
        }

        z_buffer
    }

    pub fn initialize(&mut self) {
        let inf = N::infinity();

        for i in 0..self.height {
            for j in 0..self.width {
                self.buf[i][j] = inf;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn lines(&self) -> ZBufferLineIter<N> {
        ZBufferLineIter {
            index: 0,
            lines: &self.buf
        }
    }
}

struct ZBufferLineIter<'a, N: 'a> {
    index: usize,
    lines: &'a [Vec<N>],
}

impl<'a, N: 'a> Iterator for ZBufferLineIter<'a, N> {
    type Item = &'a [N];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.lines.len() {
            Some(&self.lines[self.index])
        } else {
            None
        }
    }
}

/// Return an initialized heap allocated frame buffer.
pub fn frame_buffer(width: usize, height: usize) -> Box<FrameBuffer> {
    let mut frame_buffer = Box::new(FrameBuffer::new(width, height));
    (&mut (*frame_buffer)).initialize();

    frame_buffer
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buf: Vec<Vec<Rgb>>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let mut frame_buffer = FrameBuffer {
            width: width,
            height: height,
            buf: Vec::with_capacity(height)
        };

        for _ in 0..frame_buffer.height {
            frame_buffer.buf.push(Vec::with_capacity(width));
        }

        frame_buffer
    }

    pub fn initialize(&mut self) {
        for i in 0..self.height {
            for _ in 0..self.width {
                self.buf[i].push(Rgb::from_channels(0,0,0));
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn dump_frame(&self, other_buf: &mut [u8]) -> Option<usize> {
        if other_buf.len() >= 3 * self.height * self.width {
            for i in 0..self.height {
                for j in 0..self.width {
                    other_buf[self.width * i + j]   = self.buf[i][j][0]; 
                    other_buf[self.width * i + j+1] = self.buf[i][j][1];
                    other_buf[self.width * i + j+2] = self.buf[i][j][2];
                }
            }

            Some(3 * self.height * self.width)
        } else {
            None
        }
    }

    fn scanlines(&self) -> ScanlineIter {
        ScanlineIter {
            index: 0,
            lines: &self.buf
        }
    }
}

struct ScanlineIter<'a> {
    index: usize,
    lines: &'a [Vec<Rgb>],
}

impl<'a> Iterator for ScanlineIter<'a> {
    type Item = &'a [Rgb];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.lines.len() {
            Some(&self.lines[self.index])
        } else {
            None
        }
    }
}

impl ops::Index<usize> for FrameBuffer {
    type Output = Vec<Rgb>;

    fn index(&self, index: usize) -> &Vec<Rgb> {
        &self.buf[index]
    } 
}

impl<'a> ops::Index<usize> for &'a FrameBuffer {
    type Output = Vec<Rgb>;

    fn index(&self, index: usize) -> &Vec<Rgb> {
        &self.buf[index]
    } 
}

impl ops::IndexMut<usize> for FrameBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{Vector3, Point3};
    use color::Rgb;


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
    fn test_frame_buffer_should_correctly_report_dimensions() {
        let width  = 512;
        let height = 512;
        let buf = super::frame_buffer(width, height);

        assert_eq!(buf.width(), width);
        assert_eq!(buf.height(), height);

        for line in buf.scanlines() {
            assert_eq!(line.len(), buf.width());
        }
    }

    #[test]
    fn test_frame_buffer_should_be_zero_after_initialization() {
        let width  = 512;
        let height = 512;
        let buf  = super::frame_buffer(width, height);
        let zero = Rgb::from_channels(0,0,0);

        for line in buf.scanlines() {
            for pixel in line {
                assert_eq!(pixel, &zero);
            }
        }
    }

    #[test]
    fn test_z_buffer_should_correctly_report_dimensions() {
        let width  = 512;
        let height = 512;
        let buf = super::z_buffer::<f32>(width, height);

        assert_eq!(buf.width(), width);
        assert_eq!(buf.height(), height);

        for line in buf.lines() {
            assert_eq!(line.len(), buf.width());
        }
    }

    #[test]
    fn test_z_buffer_should_have_only_infinite_values_after_initialization() {
        use num_traits::Float;

        let width  = 512;
        let height = 512;
        let buf = super::z_buffer::<f32>(width, height);
        let inf: f32 = Float::infinity();

        for line in buf.lines() {
            for pixel in line {
                assert_eq!(pixel, &inf);
            }
        }
    }
}
