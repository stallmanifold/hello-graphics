use nalgebra::{Vector3, Vector4, Point3, Point4, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat};
use num_traits::Float;


pub fn gouraud_shade<N>(color0: Vector3<N>, 
                     color1: Vector3<N>, 
                     color2: Vector3<N>, 
                     alpha: N,
                     beta: N,
                     gamma: N) -> Vector3<N>
    where N: Copy + BaseFloat
{
    color0*alpha + color1*beta + color2*gamma
}
