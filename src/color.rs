use std::ops;
use std::convert::From;
use std::fmt;


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
    pub fn channel_width() -> usize {
        3
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

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb(0x{:x}, 0x{:x}, 0x{:x})", self.data[0], self.data[1], self.data[2])
    }
}
