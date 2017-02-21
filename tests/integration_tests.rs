extern crate nalgebra;
extern crate graphics;

use graphics::color::Rgb;
use graphics::vertex;
use graphics::camera::CameraModel;
use graphics::frame_buffer;
use graphics::z_buffer;
use graphics::z_buffer::ZBuffer;
use graphics::raster;
use nalgebra::{Vector3, Point3, Matrix4};


// This test runs through the rendering of one triangle primitive from its placement in
// world space to the pixel values being written to the frame buffer. We then check that
// the state of the graphics system makes sense when it is hypothetically displayed on screen.
// In this case, the z buffer should not affect the output with only one triangle.
#[test]
fn test_z_buffer_should_not_affect_rendering_with_one_primitive() {
    // Initialize a z-axis aligned triangle in world space.
    let v0: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let v1: Point3<f32> = Point3::new(30.0, -30.0, 0.0);
    let v2: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);
    
    // Color attributes at triangle vertices.
    let c0: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    let c1: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let c2: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

    // We place a camera focal point on the z axis x units in the 
    // positive z direction. This puts it in front of the triangle that way.
    let eye: Vector3<f32> = Vector3::new(0.0, 0.0, 5.0);
    // The gaze direction is the -z axis.
    let gaze: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    // Top is defined to be the positive y axis.
    let top: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    let width: usize = 256;
    let height: usize = 256;

    // These are the viewing parameters for the camera model.
    let focal_length = 5.0;
    let aperture_width = 80.0;
    let aperture_height = 80.0;

    let near = -5.0;
    let far = -10.0;

    let camera = CameraModel::from_spec(focal_length, aperture_width, aperture_height);

    // Initialize the world space to camera space transformation.
    let m_cam = raster::world_to_camera_matrix::<f32>(eye, gaze, top);
    // Initialize the perspective projection matrix.
    let m_per = camera.get_matrix(near, far);
    // The viewport matrix. We index pixels from the bottom of the screen.
    let m_vp  = raster::viewport_matrix::<f32>(width, height);
    let m_total = m_vp * m_per * m_cam;

    let v0_vp = m_total * (v0.to_homogeneous());
    let v1_vp = m_total * (v1.to_homogeneous());
    let v2_vp = m_total * (v2.to_homogeneous());

    let v0_rast: Point3<f32> = Point3::from_homogeneous(v0_vp).unwrap();
    let v1_rast: Point3<f32> = Point3::from_homogeneous(v1_vp).unwrap();
    let v2_rast: Point3<f32> = Point3::from_homogeneous(v2_vp).unwrap();

    let area: f32 = raster::compute_area(&v0, &v1, &v2);

    // Initialize the z buffer and frame buffer.
    let mut z_buffer: Box<ZBuffer<f32>> = z_buffer::z_buffer(width, height);
    let mut frame_buffer_z = frame_buffer::frame_buffer(width, height);
    let mut frame_buffer_no_z = frame_buffer::frame_buffer(width, height);

    // Render the scene with z-buffering.
    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                w /= area;
                // Do the z-buffer test.
                let one_over_z: f32 = w[0] * v0_rast.z + w[1] * v1_rast.z + w[2] * v2_rast.z;
                let z: f32 = 1.0 / one_over_z;
                if z < (*z_buffer)[i][j] {
                    (*z_buffer)[i][j] = z;
                    // Write a sentinel value into the frame buffer.
                    frame_buffer_z[i][j] = Rgb::from_channels(0xFF, 0xFF, 0xFF);
                }
            }
        }
    }

    // Render the scene without z-buffering.
    for i in 0..height {
        for j in 0..width {
            let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
            let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
            if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                w /= area;
                // Write a sentinel value into the frame buffer.
                frame_buffer_no_z[i][j] = Rgb::from_channels(0xFF, 0xFF, 0xFF);
            }
        }
    }

    // Since we are only rendering one triangle primitive, the frame buffers should
    // agree with each other (i.e. the z buffer did not filter out any fragments.)
    assert_eq!(frame_buffer_z, frame_buffer_no_z);
    // The Z-buffering code actually wrote something in.
    let zero = frame_buffer::frame_buffer(width, height);
    assert_ne!(frame_buffer_z, zero);
}
