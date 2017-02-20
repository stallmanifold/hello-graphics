use color::rgb::Rgb;
use nalgebra::Vector3;
use num_traits::Float;
use alga::general::Real;


///
/// Calculate the RGB color of a color vector.
///
#[inline(always)]
pub fn rgb<N: Float + Real, R: RgbCast<N>>(color: Vector3<N>) -> R {
    R::rgb_cast(color)
}

pub trait RgbCast<N> where N: Float + Real {
    type RgbValue;

    fn rgb_cast(color: Vector3<N>) -> Self::RgbValue;
}

impl RgbCast<f32> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Vector3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}
/*
impl RgbCast<f32> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Vector3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}
*/
impl RgbCast<f64> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Vector3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}
/*
impl RgbCast<f64> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Vector3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}
*/