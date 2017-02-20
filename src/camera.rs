use nalgebra::{Vector3, Point3, Matrix4};
use num_traits::Float;
use alga::general::Real;
use raster;


type CameraGenerator<N> = Fn(N, N) -> Matrix4<N>;

///
/// Create a generator for the transformation of perspective coordinates
/// to the canonical view volume from the parameters of the camera.
///
pub fn from_specification<N>(focal_length: N, 
                             aperture_width: N, 
                             aperture_height: N) -> Box<CameraGenerator<N>>
    where N: Float + Real
{
    // Here we generate the ratio of (right, left, top, bottom)/near,
    // so to create an orthographic matrix, we pass 
    // in near * (right, left, top, bottom)/near.
    let _1 = N::one();
    let _2 = _1 + _1;

    let r_over_n = aperture_width / (_2 * focal_length);
    let l_over_n = -r_over_n;
    let t_over_n = aperture_height / (_2 * focal_length);
    let b_over_n = -t_over_n;

    Box::new(move |near: N, far: N| {
        let left = near * l_over_n;
        let right = near * r_over_n;
        let top = near * t_over_n;
        let bottom = near * b_over_n;

        raster::orthographic_matrix(left, right, top, bottom, near, far)
    })
}
