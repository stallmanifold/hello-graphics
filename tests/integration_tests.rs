extern crate nalgebra;
extern crate graphics;

use graphics::color;
use graphics::camera::CameraModel;
use graphics::frame_buffer;
use graphics::frame_buffer::{FrameBuffer, TopLeft};
use graphics::z_buffer;
use graphics::z_buffer::ZBuffer;
use graphics::raster;
use graphics::shader;
use nalgebra::{Vector3, Point3, Matrix4};


struct MockGraphicsPipelineWithZBuffer {
    z_buffer: Box<ZBuffer<f32>>,
    frame_buffer: Box<FrameBuffer<TopLeft>>,
}

impl MockGraphicsPipelineWithZBuffer {
    fn new(width: usize, height: usize) -> MockGraphicsPipelineWithZBuffer {
        MockGraphicsPipelineWithZBuffer {
            z_buffer: z_buffer::z_buffer(width, height),
            frame_buffer: frame_buffer::frame_buffer(width, height),
        }
    }

    fn reinitialize(&mut self) {
        self.z_buffer.initialize();
        self.frame_buffer.initialize();
    }

    fn run_with(&mut self, shader: MockShader, m_wtor: Matrix4<f32>, v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) {
        let height = self.frame_buffer.height();
        let width  = self.frame_buffer.width();

        let area: f32 = raster::compute_area(&v0, &v1, &v2);

        let v0_vp = m_wtor * (v0.to_homogeneous());
        let v1_vp = m_wtor * (v1.to_homogeneous());
        let v2_vp = m_wtor * (v2.to_homogeneous());

        let v0_rast: Point3<f32> = Point3::from_homogeneous(v0_vp).unwrap();
        let v1_rast: Point3<f32> = Point3::from_homogeneous(v1_vp).unwrap();
        let v2_rast: Point3<f32> = Point3::from_homogeneous(v2_vp).unwrap();

        for i in 0..height {
            for j in 0..width {
                let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
                let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
                if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                    w /= area;
                    // Do the z-buffer test.
                    let one_over_z: f32 = w[0] * v0_rast.z + w[1] * v1_rast.z + w[2] * v2_rast.z;
                    let z: f32 = 1.0 / one_over_z;
                    if z < (*self.z_buffer)[i][j] {
                        (*self.z_buffer)[i][j] = z;
                        // Write a shader value into the frame buffer.
                        let color = shader(v0, v1, v2, w);
                        self.frame_buffer[i][j] = color::rgb(color);
                    }
                }
            }
        }
    }

    fn run(&mut self, m_wtor: Matrix4<f32>, v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) {
        self.run_with(mock_shader(), m_wtor, v0, v1, v2)
    }
}

struct MockGraphicsPipelineWithoutZBuffer {
    frame_buffer: Box<FrameBuffer<TopLeft>>,
}

impl MockGraphicsPipelineWithoutZBuffer {
    fn new(width: usize, height: usize) -> MockGraphicsPipelineWithoutZBuffer {
        MockGraphicsPipelineWithoutZBuffer {
            frame_buffer: frame_buffer::frame_buffer(width, height),
        }
    }

    fn reinitialize(&mut self) {
        self.frame_buffer.initialize();
    }

    fn run_with(&mut self, shader: MockShader, m_wtor: Matrix4<f32>, v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) {
        let height = self.frame_buffer.height();
        let width  = self.frame_buffer.width();

        let area: f32 = raster::compute_area(&v0, &v1, &v2);
/*
        let v0_vp = m_wtor * (v0.to_homogeneous());
        let v1_vp = m_wtor * (v1.to_homogeneous());
        let v2_vp = m_wtor * (v2.to_homogeneous());

        let v0_rast: Point3<f32> = Point3::from_homogeneous(v0_vp).unwrap();
        let v1_rast: Point3<f32> = Point3::from_homogeneous(v1_vp).unwrap();
        let v2_rast: Point3<f32> = Point3::from_homogeneous(v2_vp).unwrap();
*/
        // Render the scene without z-buffering.
        for i in 0..height {
            for j in 0..width {
                let pixel = Point3::new((i as f32) + 0.5, (j as f32) + 0.5, 0.0);
                let mut w = raster::barycentric_coords(&v0, &v1, &v2, &pixel);
                if (w[0] >= 0.0) && (w[1] >= 0.0) && (w[2] >= 0.0) {
                    w /= area;
                    // Write a sentinel value into the frame buffer.
                    let color = shader(v0, v1, v2, w);
                    self.frame_buffer[i][j] = color::rgb(color);
                }
            }
        }
    }

    fn run(&mut self, m_wtor: Matrix4<f32>, v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) {
        self.run_with(mock_shader(), m_wtor, v0, v1, v2)
    }
}

fn make_pipeline_with_z_buffer(width: usize, height: usize) -> MockGraphicsPipelineWithZBuffer {
    MockGraphicsPipelineWithZBuffer::new(width, height)
}

fn make_pipeline_without_z_buffer(width: usize, height: usize) -> MockGraphicsPipelineWithoutZBuffer {
    MockGraphicsPipelineWithoutZBuffer::new(width, height)
}

type MockShader = Box<Fn(Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>) -> Vector3<f32>>;

#[allow(unused_variables)]
fn mock_shader() -> MockShader {
    Box::new(move |v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>, w: Point3<f32>| {
        Vector3::new(0.3, 0.3, 0.4)
    })
}

#[derive(Copy, Clone)]
struct CameraModelBuilder {
    focal_length: Option<f32>,
    aperture_width: Option<f32>,
    aperture_height: Option<f32>,
}

impl CameraModelBuilder {
    fn new() -> CameraModelBuilder {
        CameraModelBuilder {
            focal_length: None,
            aperture_width: None,
            aperture_height: None,
        }
    }

    fn with_focal_length(&mut self, focal_length: f32) -> Self {
        self.focal_length = Some(focal_length);

        *self
    }

    fn with_aperture_width(&mut self, aperture_width: f32) -> Self {
        self.aperture_width = Some(aperture_width);

        *self
    }

    fn with_aperture_height(&mut self, aperture_height: f32) -> Self {
        self.aperture_height = Some(aperture_height);

        *self
    }

    fn build_maybe(&self) -> Option<CameraModel<f32>> {
        match (self.focal_length, self.aperture_width, self.aperture_height) {
            (Some(l), Some(w), Some(h)) => {
                Some(CameraModel::from_spec(l, w, h))
            }
            _ => None
        }
    }

    fn build(&self) -> CameraModel<f32> {
        self.build_maybe().unwrap()
    }
}

#[derive(Copy, Clone)]
struct WorldToCameraBuilder {
    eye: Option<Vector3<f32>>,
    gaze: Option<Vector3<f32>>,
    top: Option<Vector3<f32>>,
}

impl WorldToCameraBuilder {
    fn new() -> WorldToCameraBuilder {
        WorldToCameraBuilder {
            eye: None,
            gaze: None,
            top: None,
        }
    }

    fn with_eye(&mut self, eye: Vector3<f32>) -> Self {
        self.eye = Some(eye);

        *self
    }

    fn with_gaze(&mut self, gaze: Vector3<f32>) -> Self {
        self.gaze = Some(gaze);

        *self
    }

    fn with_top(&mut self, top: Vector3<f32>) -> Self {
        self.top = Some(top);

        *self
    }

    fn build_maybe(&self) -> Option<Matrix4<f32>> {
        match (self.eye, self.gaze, self.top) {
            (Some(e), Some(g), Some(t)) => {
                Some(raster::world_to_camera_matrix::<f32>(e, g, t))
            }
            _ => None
        }
    }

    fn build(&self) -> Matrix4<f32> {
        self.build_maybe().unwrap()
    }
}

#[derive(Copy, Clone)]
struct RasterToViewportBuilder {
    width: Option<usize>,
    height: Option<usize>,
}

impl RasterToViewportBuilder {
    fn new() -> RasterToViewportBuilder {
        RasterToViewportBuilder {
            width: None,
            height: None,
        }
    }

    fn with_width(&mut self, width: usize) -> Self {
        self.width = Some(width);

        *self
    }

    fn with_height(&mut self, height: usize) -> Self {
        self.height = Some(height);

        *self
    }

    fn build_maybe(&self) -> Option<Matrix4<f32>> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => {
                Some(raster::viewport_matrix::<f32>(w, h))
            }
            _ => None
        }
    }

    fn build(&self) -> Matrix4<f32> {
        self.build_maybe().unwrap()
    }
}

#[derive(Copy, Clone)]
struct PerspectiveMatrixBuilder {
    camera_builder: Option<CameraModelBuilder>,
    near_plane: Option<f32>,
    far_plane: Option<f32>,
}

impl PerspectiveMatrixBuilder {
    fn new() -> Self {
        PerspectiveMatrixBuilder {
            camera_builder: None,
            near_plane: None,
            far_plane: None,
        }
    }

    fn with_camera_builder(&mut self, camera_builder: CameraModelBuilder) -> Self {
        self.camera_builder = Some(camera_builder);

        *self
    } 

    fn with_near_plane(&mut self, near_plane: f32) -> Self {
        self.near_plane = Some(near_plane);

        *self
    }

    fn with_far_plane(&mut self, far_plane: f32) -> Self {
        self.far_plane = Some(far_plane);

        *self
    }

    fn build_maybe(&self) -> Option<Matrix4<f32>> {
        match (self.camera_builder, self.near_plane, self.far_plane) {
            (Some(cb), Some(near), Some(far)) => {
                let camera = cb.build();
                let m_per = camera.get_matrix(near, far);

                Some(m_per)
            }
            _ => None,
        }
    }

    fn build(&self) -> Matrix4<f32> {
        self.build_maybe().unwrap()
    }
}

#[derive(Copy, Clone)]
struct WorldToRasterBuilder {
    wtoc_builder: Option<WorldToCameraBuilder>,
    per_builder: Option<PerspectiveMatrixBuilder>,
    viewport_builder: Option<RasterToViewportBuilder>, 
}

impl WorldToRasterBuilder {
    fn new() -> Self {
        WorldToRasterBuilder {
            wtoc_builder: None,
            per_builder: None,
            viewport_builder: None,
        }
    }

    fn with_world_to_camera(&mut self, wtoc_builder: WorldToCameraBuilder) -> Self {
        self.wtoc_builder = Some(wtoc_builder);

        *self
    }

    fn with_perspective(&mut self, per_builder: PerspectiveMatrixBuilder) -> Self {
        self.per_builder = Some(per_builder);

        *self
    }

    fn with_viewport(&mut self, viewport_builder: RasterToViewportBuilder) -> Self {
        self.viewport_builder = Some(viewport_builder);

        *self
    }

    fn build_maybe(&self) -> Option<Matrix4<f32>> {
        match (self.wtoc_builder, self.per_builder, self.viewport_builder) {
            (Some(wtoc_b), Some(per_b), Some(vp_b)) => {
                let m_wtoc = wtoc_b.build();
                let m_per  = per_b.build();
                let m_vp   = vp_b.build();

                Some(m_vp * m_per * m_wtoc)
            }
            _ => None,
        }
    }

    fn build(&self) -> Matrix4<f32> {
        self.build_maybe().unwrap()
    }
}

/// Each integration test uses the same camera parameters.
fn make_camera(width: usize, height: usize) -> Matrix4<f32> {
    // Given a camera focal point on the z axis x units in the 
    // positive z direction. This puts it in front of the triangle that way.
    // The gaze direction is the -z axis.
    // Top is defined to be the positive y axis.
    let camera_builder = 
        CameraModelBuilder::new().with_focal_length(5.0)
                                 .with_aperture_width(80.0)
                                 .with_aperture_height(80.0);

    // Initialize the world space to camera space transformation.
    let wtoc_builder = 
        WorldToCameraBuilder::new().with_eye(Vector3::new(0.0, 0.0, 5.0))
                                   .with_gaze(Vector3::new(0.0, 0.0, -1.0))
                                   .with_top(Vector3::new(0.0, 1.0, 0.0));

    //let m_cam = wtoc_builder.build();
    // Initialize the perspective projection matrix.
    let per_builder = 
        PerspectiveMatrixBuilder::new().with_camera_builder(camera_builder)
                                       .with_near_plane(-5.0)
                                       .with_far_plane(-10.0);
    
    // The viewport matrix. We index pixels from the bottom of the screen.
    let vp_builder = 
        RasterToViewportBuilder::new().with_width(width)
                                      .with_height(height);
    let wtor_builder = 
        WorldToRasterBuilder::new().with_world_to_camera(wtoc_builder)
                                   .with_perspective(per_builder)
                                   .with_viewport(vp_builder);

    wtor_builder.build()
}

// This test runs through the rendering of one triangle primitive from its placement in
// world space to the pixel values being written to the frame buffer. We then check that
// the state of the graphics system makes sense when it is hypothetically displayed on screen.
// In this case, the z buffer should not affect the output with only one triangle.
#[test]
fn test_z_buffer_should_not_affect_rendering_with_one_primitive() {
    // GIVEN: A scene with exactly one triangle primitive.
    let v0: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let v1: Point3<f32> = Point3::new(30.0, -30.0, 0.0);
    let v2: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);

    let width: usize = 512;
    let height: usize = 512;

    let m_total = make_camera(width, height);
    let mut pipeline_z = make_pipeline_with_z_buffer(width, height);
    let mut pipeline_no_z = make_pipeline_without_z_buffer(width, height);

    // WHEN: The scene is rendered using z-buffering.
    pipeline_z.run(m_total, v0, v1, v2);
    pipeline_no_z.run(m_total, v0, v1, v2);

    // THEN: The Z-buffered pipeline should actually render something.
    let zero = frame_buffer::frame_buffer(width, height);
    assert_ne!(pipeline_z.frame_buffer, zero);
    assert_ne!(pipeline_no_z.frame_buffer, zero);

    // THEN: The z-buffered pipeline should render the entire triangle
    //       just like the pipeline without z-buffering.
    assert_eq!(pipeline_z.frame_buffer, pipeline_no_z.frame_buffer);
}

#[test]
fn test_z_buffer_should_give_same_results_as_rendering_objects_back_to_front() {
    // GIVEN: A scene with two triangles ordered back to front.
    let u0: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
    let u1: Point3<f32> = Point3::new(0.0, -30.0, 0.0);
    let u2: Point3<f32> = Point3::new(-30.0, 30.0, 0.0);

    let v0: Point3<f32> = Point3::new(20.0, 20.0, 0.0);
    let v1: Point3<f32> = Point3::new(20.0, -20.0, 0.0);
    let v2: Point3<f32> = Point3::new(-20.0, -20.0, 0.0);

    let width: usize = 512;
    let height: usize = 512;

    let m_total = make_camera(width, height);
    let mut pipeline_z = make_pipeline_with_z_buffer(width, height);
    let mut pipeline_no_z = make_pipeline_without_z_buffer(width, height);

    // WHEN: The scene is rendered using z-buffering.
    pipeline_z.run(m_total, v0, v1, v2);
    pipeline_z.run(m_total, u0, u1, u2);

    // THEN: The scene should be rendered as though the triangles were rendered in back to front order without the z buffer.
    pipeline_no_z.run(m_total, u0, u1, u2);
    pipeline_no_z.run(m_total, v0, v1, v2);

    assert_eq!(pipeline_z.frame_buffer, pipeline_no_z.frame_buffer);
}
