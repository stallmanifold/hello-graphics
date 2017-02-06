use std::ops;
use std::f32;
use std::f64;
use vecmath;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2d<T> where T: Copy {
    inner: vecmath::Vector2<T>
}


impl<T> Vec2d<T> where T: Copy {
    #[inline]
    pub fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d {
            inner: [x,y]
        }
    }
}

