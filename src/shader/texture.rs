#![macro_use]
use nalgebra::{Vector3};
use num_traits::Float;
use alga::general::Real;
use std::ops;


pub trait TextureMap<N, Args> where N: Float + Real {
    fn apply(&self, args: Args) -> Vector3<N>;
}

macro_rules! fn_impl {
    ($type_name : ty, $float_type: ident, $args_type : ty) => {
        impl<$float_type> FnOnce<$args_type> for $type_name where $float_type: Float + Real {
            type Output = Vector3<$float_type>;
            extern "rust-call" fn call_once(self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl<$float_type> FnMut<$args_type> for $type_name where $float_type: Float + Real {
            extern "rust-call" fn call_mut(&mut self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl<$float_type> Fn<$args_type> for $type_name where $float_type: Float + Real {
            extern "rust-call" fn call(&self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
    }
}

macro_rules! specific_fn_impl {
    ($type_name : ty, $float_type: ty, $args_type : ty) => {
        impl FnOnce<$args_type> for $type_name {
            type Output = Vector3<$float_type>;
            extern "rust-call" fn call_once(self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl FnMut<$args_type> for $type_name {
            extern "rust-call" fn call_mut(&mut self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl Fn<$args_type> for $type_name {
            extern "rust-call" fn call(&self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
    }
}

/// Texture arrays are sample points (texels) in the range [-1,1] x [-1,1] that we
/// interpolate over.
pub struct TextureArray<N> where N: Real + Float {
    data: Vec<Vec<Vector3<N>>>,
}

impl<N> TextureArray<N> where N: Real + Float {
    pub fn new(width: usize, height: usize) -> TextureArray<N> {
        let mut data = Vec::with_capacity(height);
        
        for _ in 0..height {
            data.push(Vec::with_capacity(width));
        }

        TextureArray {
            data: data
        }
    }
}

impl<N> ops::Index<usize> for TextureArray<N> where N: Float + Real {
    type Output = [Vector3<N>];

    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

pub struct Texture<N> where N: Real + Float {
    width: usize,
    height: usize,
    data: Box<TextureArray<N>>,
}

macro_rules! texture_impl {
    ($type_name: ty) => {
        impl Texture<$type_name> {
            /// Fetch the nearest texel. This is useful for debugging.
            fn lookup(&self, u: $type_name, v: $type_name) -> Vector3<$type_name> {
                let i = Float::round(u * (self.width as $type_name) - 0.5) as usize;
                let j = Float::round(v * (self.height as $type_name) - 0.5) as usize;
                self.get_texel(i, j)
            }

            fn get_texel(&self, i: usize, j: usize) -> Vector3<$type_name> {
                (*self.data)[i][j]
            }
        }
    }
}

texture_impl!(f32);
texture_impl!(f64);
