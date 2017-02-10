extern crate nalgebra;

extern crate image;
extern crate num_traits;

mod raster;
mod z_buffer;
mod frame_buffer;
mod util;
mod shade;
mod color;

use nalgebra::{Matrix4, Vector4, Vector3, Point3, Point4, 
               FromHomogeneous, Transpose, ToHomogeneous};
use std::path::Path;
use z_buffer::ZBuffer;
use image::ColorType;
use color::Rgb;
use std::ops::Mul;

// TODO: Add perspective correction to gouraud model.

fn make_buffer(size: usize) -> Box<Vec<u8>> {
    let mut buf = Box::new(Vec::with_capacity(size));
    for _ in 0..buf.capacity() {
        buf.push(0x00);
    }

    buf
}

fn main() {
    // The triangle in world space.
    let v0: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let v1: Point3<f32> = Point3::new(30.0, -30.0, 0.0);
    let v2: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);
    
    // Color attributes at triangle vertices.
    let c0: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c2: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

    // We place a camera focal point on the z axis x units in the 
    // positive direction. This puts it in front of the triangle that way.
    let eye: Vector3<f32> = Vector3::new(0.0, 0.0, 5.0);
    // The gaze direction is the -z axis.
    let gaze: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    // Top is defined to be the positive y axis.
    let top: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    let width: usize = 512;
    let height: usize = 512;
    // perspective matrix parameters.
    let l = -40.0; 
    let r = 40.0;
    let t = 40.0;
    let b = -40.0;
    let n = -10.0;
    let f = -20.0;

    let m_cam = raster::world_to_camera_matrix::<f32>(eye, gaze, top);
    let m_per = raster::perspective_projection_matrix::<f32>(l, r, t, b, n, f);
    let m_vp  = raster::viewport_matrix::<f32>(width, height);
    let m_total = m_vp * m_per * m_cam;

    let v0_vp = m_total * (v0.to_homogeneous());
    let v1_vp = m_total * (v1.to_homogeneous());
    let v2_vp = m_total * (v2.to_homogeneous());

    let v0 = FromHomogeneous::from(&v0_vp);
    let v1 = FromHomogeneous::from(&v1_vp);
    let v2 = FromHomogeneous::from(&v2_vp);

    let area: f32 = raster::compute_area(&v0, &v1, &v2);

    //let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer = frame_buffer::frame_buffer(width, height);

    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                //w /= area;
                //let color = shade::gouraud(c0, c1, c2, w);
                //let rgb = shade::color_rgb(color);
                frame_buffer[i][j] = Rgb::from_channels(0xFF, 0x00, 0x00);//rgb;
            } else {
                frame_buffer[i][j] = Rgb::from_channels(0x00, 0x00, 0x00);
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
