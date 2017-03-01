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
mod color;
mod ppm;
mod shader;
mod vertex;
mod camera;
mod shape;

use nalgebra::{Vector2, Vector3, Point3};
use z_buffer::ZBuffer;
use color::Rgb;
use std::fs::File;
use ppm::NetPBMEncoder;
use shader::checkerboard;
use camera::CameraModel;
use shape::plane;
use shape::triangle;


fn make_buffer(size: usize) -> Box<Vec<u8>> {
    let mut buf = Box::new(Vec::with_capacity(size));
    for _ in 0..buf.capacity() {
        buf.push(0x00);
    }

    buf
}

fn main() {
    // The triangle in world space.
    let nv0: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let nv1: Point3<f32> = Point3::new(30.0, -30.0, 0.0);
    let nv2: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);

    let mesh = triangle::create(nv0, nv1, nv2);

    // We place a camera focal point on the z axis x units in the 
    // positive direction. This puts it in front of the triangle that way.
    let eye: Vector3<f32> = Vector3::new(0.0, 0.0, 5.0);
    // The gaze direction is the -z axis.
    let gaze: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    // Top is defined to be the positive y axis.
    let top: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    let width: usize = 512;
    let height: usize = 512;

    // Camera model parameters.
    let focal_length = 5.0;
    let aperture_width = 80.0;
    let aperture_height = 80.0;

    let near = -5.0;
    let far = -10.0;

    let camera = CameraModel::from_spec(focal_length, aperture_width, aperture_height);

    let m_cam = raster::world_to_camera_matrix::<f32>(eye, gaze, top);
    let m_per = camera.get_matrix(near, far);
    let m_vp  = raster::viewport_matrix::<f32>(width, height);
    let m_total = m_vp * m_per * m_cam;

    let shader = checkerboard::shader::<f32>(5);

    // Initialize the z buffer and frame buffer.
    let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer = frame_buffer::frame_buffer(width, height);

    // Graphite color.
    let default_rgb = Rgb::from_channels(0x3B, 0x44, 0x4B);

    let verts = mesh.vertices();

    for face in mesh.faces() {
        let v0 = Point3::from_homogeneous(m_total * verts[face[0]].to_homogeneous()).unwrap();
        let v1 = Point3::from_homogeneous(m_total * verts[face[1]].to_homogeneous()).unwrap();
        let v2 = Point3::from_homogeneous(m_total * verts[face[2]].to_homogeneous()).unwrap();

        let one_over_z0 = 1.0 / v0.z;
        let one_over_z1 = 1.0 / v1.z;
        let one_over_z2 = 1.0 / v2.z;

        let area: f32 = raster::compute_area(&v0, &v1, &v2);

        let st0 = Vector2::new(0.0, 0.0);
        let st1 = Vector2::new(0.0, 1.0);
        let st2 = Vector2::new(1.0, 0.0);

        // Render the current scene.
        for i in 0..height {
            for j in 0..width {
                let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
                let mut w = raster::barycentric_coords(&v2, &v0, &v1, &pixel);
                if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                    w /= area;
                    // Apply perspective correction.
                    //let z = 1.0 / (w[0] * one_over_z0 + w[1] * one_over_z1 + w[2] * one_over_z2);
                    //let color = z * shader(c0_pc, c1_pc, c2_pc, w);
                    let color = shader(st0, st1, st2, v0, v1, v2, w);
                    let rgb = color::rgb(color);
                    frame_buffer[i][j] = rgb;
                } else {
                    // Use a background color.
                    frame_buffer[i][j] = default_rgb;
                }
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
