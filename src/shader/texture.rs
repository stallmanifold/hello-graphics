#![macro_use]
use nalgebra::{Vector3};
use num_traits::Float;
use alga::general::Real;


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
