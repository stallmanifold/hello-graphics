extern crate nalgebra;

use nalgebra::{Matrix4, Vector4, Vector3, Point3, Point4, Transpose, ToHomogeneous};
use std::convert::From;

mod raster;

fn main() {
    let m = Matrix4::from(&[[1.0, 0.0, 0.0, 2.0], 
                            [0.0, 1.0, 0.0, 3.0], 
                            [0.0, 0.0, 1.0, 4.0], 
                            [0.0, 0.0, 0.0, 1.0]]).transpose();
    let vec = Vector4::from(&[1.0, 1.0, 1.0, 1.0]);
    let vec1 = Point3::from(&[1.0, 1.0, 1.0]);
    let res = m * vec;
    println!("{}", res);
    let res = vec1.to_homogeneous();
    println!("{}", res);
}