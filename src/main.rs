#![crate_name="graphics"]
extern crate nalgebra;
extern crate num_traits;

mod raster;
mod z_buffer;
mod frame_buffer;
mod util;
mod shade;
mod color;
mod ppm;

use nalgebra::{Vector3, Point3, FromHomogeneous, ToHomogeneous};
use z_buffer::ZBuffer;
use color::Rgb;
use std::fs::File;
use ppm::NetPBMEncoder;


fn make_buffer(size: usize, color: u8) -> Box<Vec<u8>> {
    let mut buf = Box::new(Vec::with_capacity(size));
    for _ in 0..buf.capacity() {
        buf.push(color);
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
    let n = -5.0;
    let f = -20.0;

    let m_cam = raster::world_to_camera_matrix::<f32>(eye, gaze, top);
    let m_per = raster::perspective_projection_matrix::<f32>(l, r, t, b, n, f);
    let m_vp  = raster::viewport_matrix::<f32>(width, height);
    let m_total = m_vp * m_per * m_cam;

    let v0_vp = m_total * v0.to_homogeneous();
    let v1_vp = m_total * v1.to_homogeneous();
    let v2_vp = m_total * v2.to_homogeneous();

    let v0: Point3<f32> = FromHomogeneous::from(&v0_vp);
    let v1: Point3<f32> = FromHomogeneous::from(&v1_vp);
    let v2: Point3<f32> = FromHomogeneous::from(&v2_vp);
    // Perspective correction
    let c0_pc = shade::perspective_correct(v0, c0);
    let c1_pc = shade::perspective_correct(v1, c1);
    let c2_pc = shade::perspective_correct(v2, c2);

    let area: f32 = raster::compute_area(&v0, &v1, &v2);
    let one_over_z0 = 1.0 / v0.z;
    let one_over_z1 = 1.0 / v1.z;
    let one_over_z2 = 1.0 / v2.z;

    // Initialize the z buffer and frame buffer.
    let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer = frame_buffer::frame_buffer(width, height);

    // Render the current scene.
    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                w /= area;
                // Apply perspective correction.
                let z = 1.0 / (w[0] * one_over_z0 + w[1] * one_over_z1 + w[2] * one_over_z2);
                let color = z * shade::gouraud(c0_pc, c1_pc, c2_pc, w);
                let rgb = shade::color_rgb(color);
                frame_buffer[i][j] = rgb;
            }
        }
    }

    let mut buf = make_buffer(Rgb::channel_count() * height * width, 0x00);
    
    frame_buffer.dump_frame(&mut *buf)
                .expect("Could not write into buffer!");

    let mut f: File = File::create("triangle.ppm")
                           .expect("Could not create file.");
    let mut ppm = NetPBMEncoder::new(ppm::NetPBM::PixMapAscii, &mut f);
    let _ = ppm.encode(&buf, width as u32, height as u32);
}
