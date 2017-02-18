#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![crate_name="graphics"]
extern crate nalgebra;
extern crate num_traits;
extern crate alga;
#[macro_use]
extern crate approx;

pub mod raster;
pub mod z_buffer;
pub mod frame_buffer;
mod util;
pub mod shade;
pub mod color;
mod ppm;