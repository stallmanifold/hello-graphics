#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![crate_name="graphics"]
extern crate nalgebra;
extern crate num_traits;
extern crate num_integer;
extern crate alga;
#[macro_use]
extern crate approx;

pub mod vertex;
pub mod raster;
pub mod z_buffer;
pub mod frame_buffer;
pub mod camera;
mod util;
pub mod shader;
pub mod color;
pub mod shape;
pub mod ppm;