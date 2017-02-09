use nalgebra::{Vector3, Vector4, Point3, Point4, Matrix4};
use nalgebra::{Cross, Norm, BaseFloat};
use num_traits::Float;


pub fn gouraud<N>(color0: Vector3<N>, 
                  color1: Vector3<N>, 
                  color2: Vector3<N>, 
                  bary:   Point3<N>) -> Vector3<N>
    where N: Copy + BaseFloat
{
    //color0*alpha + color1*beta + color2*gamma
    color0 * bary[0] + color1 * bary[1] + color2 * bary[2]
}

pub fn color_rgb(color: Vector3<f32>) -> [u8; 3] {
    let r = (255.0 * color.x).trunc() as u8;
    let g = (255.0 * color.y).trunc() as u8;
    let b = (255.0 * color.z).trunc() as u8;

    [r,g,b]
}
