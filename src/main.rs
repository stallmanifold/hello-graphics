extern crate nalgebra;
extern crate num_traits;

mod raster;
mod util;
mod shade;

use nalgebra::{Matrix4, Vector4, Vector3, Point3, Point4, Transpose, ToHomogeneous};
use std::convert::From;
use raster::ZBuffer;


// TODO: Add perspective correction to gouraud model.

fn main() {
    let v2: Point3<f32>  = Point3::new(-48.0, -10.0, 82.0);
    let v1: Point3<f32>  = Point3::new(29.0, -15.0, 44.0);
    let v0: Point3<f32>  = Point3::new(13.0, 34.0, 114.0);
    let c2: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c0: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

    let width: usize = 512;
    let height: usize = 512;

    let mut z_buffer: Box<ZBuffer<f32>> = raster::z_buffer(width, height);
    let mut frame_buffer = raster::frame_buffer(width, height);

    let area: f32 = raster::compute_area(&v0, &v1, &v2);

    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (height - j) as f32 + 0.5, 0.0);
            //let mut w0 = raster::compute_edge(&v1, &v2, &pixel);
            //let mut w1 = raster::compute_edge(&v2, &v0, &pixel);
            //let mut w2 = raster::compute_edge(&v0, &v1, &pixel);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                //w0 /= area;
                //w1 /= area;
                //w2 /= area;
                w /= area;
                let color = shade::gouraud(c0, c1, c2, w);
                let rgb = shade::rgb_color(color);
                //let r = w0 * c0.x + w1 * c1.x + w2 * c2.x;
                //let g = w0 * c0.y + w1 * c1.y + w2 * c2.y;
                //let b = w0 * c0.z + w1 * c2.z + w2 * c2.z;
                frame_buffer[i][j] = rgb;
            }
        }
    }
}