use nalgebra::{Vector3, Point3};
use nalgebra::{BaseFloat};
use color::Rgb;


pub fn gouraud<N>(color0: Vector3<N>, 
                  color1: Vector3<N>, 
                  color2: Vector3<N>, 
                  bary:   Point3<N>) -> Vector3<N>
    where N: BaseFloat
{
    color0 * bary[0] + color1 * bary[1] + color2 * bary[2]
}

pub fn color_rgb(color: Vector3<f32>) -> Rgb {
    let r = (255.0 * color.x).trunc() as u8;
    let g = (255.0 * color.y).trunc() as u8;
    let b = (255.0 * color.z).trunc() as u8;

    Rgb::from_channels(r, g, b)
}

/// Perspective correct vertex attributes
#[inline(always)]
pub fn perspective_correct<N>(position: Point3<N>, 
                              vertex: Vector3<N>) -> Vector3<N> 
    where N: BaseFloat
{
    Vector3::new(vertex.x / position.z, vertex.y / position.z, vertex.z)
}

/// Perspective correct vertex attributes in place.
#[inline(always)]
pub fn perspective_correct_inplace<N>(position: Point3<N>, 
                                      vertex: &mut Vector3<N>) 
    where N: BaseFloat
{
    vertex.x /= position.z;
    vertex.y /= position.z;
}

#[cfg(test)]
mod tests {
    
}