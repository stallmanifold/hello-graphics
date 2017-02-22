use std::ops;
use std::convert::From;
use std::fmt;
use alga::general::Real;
use nalgebra::{Point3, Vector3};


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rgb {
    data: [u8; 3],
}

impl Rgb {
    #[inline]
    pub fn from_channels(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            data: [r, g, b]
        }
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.data[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.data[1]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.data[2]
    }

    /// Returns the number of RGB channels. Each one is a byte in size.
    #[inline]
    pub fn channel_count() -> usize {
        3
    }

    /// Returns a slice of subpixels, one for each RGB channel.
    pub fn channels(&self) -> &[u8] {
        &self.data
    }
}

impl ops::Index<usize> for Rgb {
    type Output = u8;

    #[inline]
    fn index(&self, _index: usize) -> &u8 {
        &self.data[_index]
    }
}

impl ops::IndexMut<usize> for Rgb {
    #[inline]
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        &mut self.data[_index]
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(arr: [u8; 3]) -> Rgb {
        Rgb {
            data: arr
        }
    }
}

impl<'a> From<&'a [u8; 3]> for Rgb {
    fn from(arr: &[u8; 3]) -> Rgb {
        Rgb {
            data: *arr
        }
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(tuple: (u8, u8, u8)) -> Rgb {
        Rgb {
            data: [tuple.0, tuple.1, tuple.2]
        }
    }
}

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb::from_channels(0x00, 0x00, 0x00)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X}{:X}{:X}", self.data[0], self.data[1], self.data[2])
    }
}

pub trait RgbCast<V> {
    type RgbValue;

    fn rgb_cast(color: V) -> Self::RgbValue;
}


macro_rules! rgb_cast_impl {
    ($ident: ident, $float_type: ty, $max_value: expr) => {
        impl RgbCast<$ident<$float_type>> for Rgb {
            type RgbValue = Rgb;

            #[inline]
            fn rgb_cast(color: $ident<$float_type>) -> Self::RgbValue {
                let r: u8 = Real::trunc($max_value * color.x) as u8;
                let g: u8 = Real::trunc($max_value * color.y) as u8;
                let b: u8 = Real::trunc($max_value * color.z) as u8;

                Rgb::from_channels(r, g, b)
            }
        }

        impl<'a> RgbCast<&'a $ident<$float_type>> for Rgb {
            type RgbValue = Rgb;

            #[inline]
            fn rgb_cast(color: &'a $ident<$float_type>) -> Self::RgbValue {
                let r: u8 = Real::trunc($max_value * color.x) as u8;
                let g: u8 = Real::trunc($max_value * color.y) as u8;
                let b: u8 = Real::trunc($max_value * color.z) as u8;

                Rgb::from_channels(r, g, b)
            }
        }
    }
}

rgb_cast_impl!(Vector3, f32, 255.0);
rgb_cast_impl!(Vector3, f64, 255.0);
rgb_cast_impl!(Point3, f32, 255.0);
rgb_cast_impl!(Point3, f64, 255.0);
