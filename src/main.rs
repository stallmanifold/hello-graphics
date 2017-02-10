extern crate nalgebra;

extern crate image;
extern crate num_traits;

mod raster;
mod z_buffer;
mod frame_buffer;
mod util;
mod shade;
mod color;

use nalgebra::{Matrix4, Vector4, Vector3, Point3, Point4, Transpose, ToHomogeneous};
use std::path::Path;
use z_buffer::ZBuffer;
use image::ColorType;
use color::Rgb;

// TODO: Add perspective correction to gouraud model.

fn make_buffer(size: usize) -> Box<Vec<u8>> {
    let mut buf = Box::new(Vec::with_capacity(size));
    for _ in 0..buf.capacity() {
        buf.push(0x00);
    }

    buf
}

fn main() {
    let v2: Point3<f32>  = Point3::new(-48.0, -10.0, 82.0);
    let v1: Point3<f32>  = Point3::new(29.0, -15.0, 44.0);
    let v0: Point3<f32>  = Point3::new(13.0, 34.0, 114.0);
    let c2: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c0: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

    let width: usize = 512;
    let height: usize = 512;

    // Project triangle into screen.
    // TODO: convert to raster coordinates.

    let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer = frame_buffer::frame_buffer(width, height);

    let area: f32 = raster::compute_area(&v0, &v1, &v2);

    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (height - j) as f32 + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                w /= area;
                let color = shade::gouraud(c0, c1, c2, w);
                let rgb = shade::color_rgb(color);
                frame_buffer[i][j] = rgb;
            }
        }
    }

    let mut buf = make_buffer(Rgb::channel_width() * height * width);

    frame_buffer.dump_frame(&mut *buf)
                .expect("Something went wrong!");

    let path = Path::new("./triangle.png");
    image::save_buffer(&path, buf.as_ref(), 
                       width as u32, 
                       height as u32, 
                       ColorType::RGB(8))
          .expect("Something went wrong with saving the image!");
}
