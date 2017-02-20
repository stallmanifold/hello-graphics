use color::rgb::Rgb;
use nalgebra::Vector3;
use nalgebra::Point3;
use num_traits::Float;
use alga::general::Real;


///
/// Calculate the RGB color of a color vector.
///
#[inline(always)]
pub fn rgb<V, R: RgbCast<V, RgbValue=R>>(color: V) -> R {
    R::rgb_cast(color)
}

pub trait RgbCast<V> {
    type RgbValue;

    fn rgb_cast(color: V) -> Self::RgbValue;
}

/*
macro_rules! rgb_cast_impl {
    ($float_type: ty) => {
        impl RgbCast<> for Rgb {
            type RgbValue = Rgb;

            #[inline]
            fn rgb_cast(color: )
        }
    }
}
*/

impl RgbCast<Vector3<f32>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Vector3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl<'a> RgbCast<&'a Vector3<f32>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Vector3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl RgbCast<Vector3<f64>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Vector3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl<'a> RgbCast<&'a Vector3<f64>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Vector3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl RgbCast<Point3<f32>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Point3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl<'a> RgbCast<&'a Point3<f32>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Point3<f32>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl RgbCast<Point3<f64>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: Point3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}

impl<'a> RgbCast<&'a Point3<f64>> for Rgb {
    type RgbValue = Rgb;

    #[inline]
    fn rgb_cast(color: &Point3<f64>) -> Self::RgbValue {
        let r: u8 = Real::trunc(255.0 * color.x) as u8;
        let g: u8 = Real::trunc(255.0 * color.y) as u8;
        let b: u8 = Real::trunc(255.0 * color.z) as u8;

        Rgb::from_channels(r, g, b)
    }
}
