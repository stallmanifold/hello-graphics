use std::ops;
use std::convert::From;
use std::fmt;
use num_traits::Float;
use alga::general::Real;
use nalgebra::Vector3;
use std::fmt::Debug;


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


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rgb {
    data: [u8; 3],
}

impl Rgb {
    #[inline(always)]
    pub fn from_channels(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            data: [r, g, b]
        }
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.data[0]
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.data[1]
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.data[2]
    }

    /// Returns the number of RGB channels. Each one is a byte in size.
    #[inline(always)]
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

    #[inline(always)]
    fn index(&self, _index: usize) -> &u8 {
        &self.data[_index]
    }
}

impl ops::IndexMut<usize> for Rgb {
    #[inline(always)]
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        &mut self.data[_index]
    }
}

impl From<[u8; 3]> for Rgb {
    #[inline(always)]
    fn from(arr: [u8; 3]) -> Rgb {
        Rgb {
            data: arr
        }
    }
}

impl<'a> From<&'a [u8; 3]> for Rgb {
    #[inline(always)]
    fn from(arr: &[u8; 3]) -> Rgb {
        Rgb {
            data: arr.clone()
        }
    }
}

impl From<(u8, u8, u8)> for Rgb {
    #[inline(always)]
    fn from(tuple: (u8, u8, u8)) -> Rgb {
        Rgb {
            data: [tuple.0, tuple.1, tuple.2]
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X}{:X}{:X}", self.data[0], self.data[1], self.data[2])
    }
}
