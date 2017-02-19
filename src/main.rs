#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![crate_name="graphics"]
extern crate nalgebra;
extern crate num_traits;
extern crate num_integer;
extern crate alga;
#[macro_use]
extern crate approx;

mod raster;
mod z_buffer;
mod frame_buffer;
mod util;
mod mesh;
mod shade;
mod color;
mod ppm;
mod shader;

use nalgebra::{Vector2, Vector3, Point3};
use z_buffer::ZBuffer;
use color::Rgb;
use std::fs::File;
use ppm::NetPBMEncoder;
use shader::gouraud;
use shader::checkerboard;


fn make_buffer(size: usize) -> Box<Vec<u8>> {
    let mut buf = Box::new(Vec::with_capacity(size));
    for _ in 0..buf.capacity() {
        buf.push(0x00);
    }

    buf
}

fn main() {
    // The triangle in world space.
    let v0: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);
    let v1: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let v2: Point3<f32> = Point3::new(30.0, -30.0, 0.0);
    
    // Color attributes at triangle vertices.
    let c0: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c2: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

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

    let v0: Point3<f32> = Point3::from_homogeneous(v0_vp).unwrap();
    let v1: Point3<f32> = Point3::from_homogeneous(v1_vp).unwrap();
    let v2: Point3<f32> = Point3::from_homogeneous(v2_vp).unwrap();
    // Perspective correction
    let c0_pc = shade::perspective_correct(v0, c0);
    let c1_pc = shade::perspective_correct(v1, c1);
    let c2_pc = shade::perspective_correct(v2, c2);

    let area: f32 = raster::compute_area(&v0, &v1, &v2);
    let one_over_z0 = 1.0 / v0.z;
    let one_over_z1 = 1.0 / v1.z;
    let one_over_z2 = 1.0 / v2.z;

    let shader = checkerboard::shader::<f32>(5);
    //let shader = gouraud::shader::<f32>();

    // Initialize the z buffer and frame buffer.
    let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer = frame_buffer::frame_buffer(width, height);

    // Graphite color.
    let default_rgb = Rgb::from_channels(0x3B, 0x44, 0x4B);

    // Render the current scene.
    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v2, &v0, &v1, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                w /= area;
                // Apply perspective correction.
                //let z = 1.0 / (w[0] * one_over_z0 + w[1] * one_over_z1 + w[2] * one_over_z2);
                //let color = z * shade::gouraud(c0_pc, c1_pc, c2_pc, w);
                let st0 = Vector2::new(0.0, 0.0);
                let st1 = Vector2::new(0.0, 1.0);
                let st2 = Vector2::new(1.0, 0.0);
                let color = shader(st0, st1, st2, v0, v1, v2, w);
                let rgb = shade::color_rgb(color);
                frame_buffer[i][j] = rgb;
            } else {
                // Use a background color.
                frame_buffer[i][j] = default_rgb;
            }
        }
    }

    let mut buf = make_buffer(Rgb::channel_count() * height * width);
    
    frame_buffer.dump_frame(&mut *buf)
                .expect("Could not write into buffer!");

    let mut f: File = File::create("triangle.ppm")
                           .expect("Could not create file.");
    let mut ppm = NetPBMEncoder::new(ppm::NetPBM::PixMapAscii, &mut f);
    let _ = ppm.encode(&buf, width as u32, height as u32);
}
