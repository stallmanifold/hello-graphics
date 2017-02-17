#![macro_use]
use nalgebra::{BaseFloat, Vector3};


pub trait TextureMap<N, Args> where N: BaseFloat {
    fn apply(&self, args: Args) -> Vector3<N>;
}

macro_rules! fn_impl {
    ($type_name : ty, $float_type: ident, $args_type : ty) => {
        impl<$float_type> FnOnce<$args_type> for $type_name where $float_type: BaseFloat {
            type Output = Vector3<$float_type>;
            extern "rust-call" fn call_once(self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl<$float_type> FnMut<$args_type> for $type_name where $float_type: BaseFloat {
            extern "rust-call" fn call_mut(&mut self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
        
        impl<$float_type> Fn<$args_type> for $type_name where $float_type: BaseFloat {
            extern "rust-call" fn call(&self, args: $args_type) -> Self::Output {
                self.apply(args)
            }
        }
    }
}
