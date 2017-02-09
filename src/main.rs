extern crate nalgebra;
extern crate num_traits;

use nalgebra::{Matrix4, Vector4, Vector3, Point3, Point4, Transpose, ToHomogeneous};
use std::convert::From;

mod raster;
mod util;
mod shading;

fn main() {
    let v2: Vector3<f32> = Vector3::new(-48.0, -10.0, 82.0);
    let v1: Vector3<f32> = Vector3::new(29.0, -15.0, 44.0);
    let v0: Vector3<f32> = Vector3::new(13.0, 34.0, 114.0);
    let c2: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c0: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

    let width: usize = 512;
    let height: usize = 512;
}