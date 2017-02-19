use std::ops;
use std::convert::From;
use std::fmt;
use num_traits::Float;
use alga::general::Real;
use nalgebra::Vector3;
use std::fmt::Debug;


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rgb<N> where N: Primitive {
    data: [N; 3],
}

impl Rgb<N> where N: Primitive {
    #[inline(always)]
    pub fn from_channels(r: N, g: N, b: N) -> Rgb<N> {
        Rgb {
            data: [r, g, b]
        }
    }

    #[inline(always)]
    pub fn r(&self) -> N {
        self.data[0]
    }

    #[inline(always)]
    pub fn g(&self) -> N {
        self.data[1]
    }

    #[inline(always)]
    pub fn b(&self) -> N {
        self.data[2]
    }

    /// Returns the number of RGB channels. Each one is a byte in size.
    #[inline(always)]
    pub fn channel_count() -> usize {
        3
    }

    /// Returns a slice of subpixels, one for each RGB channel.
    pub fn channels(&self) -> &[N] {
        &self.data
    }
}

impl<N> ops::Index<usize> for Rgb<N> {
    type Output = N;

    #[inline(always)]
    fn index(&self, _index: usize) -> &N {
        &self.data[_index]
    }
}

impl<N> ops::IndexMut<usize> for Rgb<N> {
    #[inline(always)]
    fn index_mut(&mut self, _index: usize) -> &mut N {
        &mut self.data[_index]
    }
}

impl<N> From<[N; 3]> for Rgb<N> {
    #[inline(always)]
    fn from(arr: [u8; 3]) -> Rgb<N> {
        Rgb {
            data: arr
        }
    }
}

impl<'a, N> From<&'a [N; 3]> for Rgb<N> {
    #[inline(always)]
    fn from(arr: &[N; 3]) -> Rgb<N> {
        Rgb {
            data: arr.clone()
        }
    }
}

impl<N> From<(N, N, N)> for Rgb<N> {
    #[inline(always)]
    fn from(tuple: (N, N, N)) -> Rgb<N> {
        Rgb {
            data: [tuple.0, tuple.1, tuple.2]
        }
    }
}

impl<N> Default for Rgb<N> {
    fn default() -> Rgb {
        Rgb::from_channels(0x00, 0x00, 0x00)
    }
}

impl<N> fmt::Display for Rgb<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X}{:X}{:X}", self.data[0], self.data[1], self.data[2])
    }
}
