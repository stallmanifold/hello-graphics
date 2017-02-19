use color::rgb::Rgb;
use nalgebra::Vector3;
use num_traits::Float;
use alga::general::Real;


///
/// Calculate the RGB color of a color vector.
///
#[inline(always)]
pub fn rgb(color: Vector3<f32>) -> Rgb {
    let r: u8 = Real::trunc(255.0 * color.x) as u8;
    let g: u8 = Real::trunc(255.0 * color.y) as u8;
    let b: u8 = Real::trunc(255.0 * color.z) as u8;

    Rgb::from_channels(r, g, b)
}
