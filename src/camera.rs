use nalgebra::{Vector3, Point3, Matrix4};
use num_traits::Float;
use alga::general::Real;
use raster;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CameraSpec<N> where N: Float + Real {
    pub focal_length: N,
    pub aperture_width: N,
    pub aperture_height: N,
}

impl<N> CameraSpec<N> where N: Float + Real {
    fn new(focal_length: N, aperture_width: N, aperture_height: N) -> CameraSpec<N> {
        CameraSpec {
            focal_length: focal_length,
            aperture_width: aperture_width,
            aperture_height: aperture_height,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CameraModel<N> where N: Float + Real {
    spec: CameraSpec<N>,
    l_over_n: N,
    r_over_n: N,
    t_over_n: N,
    b_over_n: N,
}

impl<N> CameraModel<N> where N: Float + Real {
    pub fn new(spec: CameraSpec<N>) -> CameraModel<N> {
        // Here we generate the ratio of (right, left, top, bottom)/near,
        // so to create an orthographic matrix, we pass 
        // in near * (right, left, top, bottom)/near.
        let _1 = N::one();
        let _2 = _1 + _1;

        let r_over_n = spec.aperture_width / (_2 * spec.focal_length);
        let l_over_n = -r_over_n;
        let t_over_n = spec.aperture_height / (_2 * spec.focal_length);
        let b_over_n = -t_over_n;

        CameraModel {
            spec: spec,
            l_over_n: l_over_n,
            r_over_n: r_over_n,
            t_over_n: t_over_n,
            b_over_n: b_over_n,
        }
    }

    ///
    /// Create a generator for the transformation of perspective coordinates
    /// to the canonical view volume from the parameters of the camera.
    ///
    pub fn get_matrix(&self, near: N, far: N) -> Matrix4<N> {
        let left   = near * self.l_over_n;
        let right  = near * self.r_over_n;
        let top    = near * self.t_over_n;
        let bottom = near * self.b_over_n;

        raster::orthographic_matrix(left, right, top, bottom, near, far)
    }

    ///
    /// Create a camera model by passing the parameters directly instead of packing them into a 
    /// camera specification first.
    pub fn from_spec(focal_length: N, aperture_width: N, aperture_height: N) -> CameraModel<N> {
        let spec = CameraSpec::new(focal_length, aperture_width, aperture_height);
        CameraModel::new(spec)
    }
}
